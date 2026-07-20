use aequitas::systems::si::{
    quantities::{Dimensionless, ReciprocalTemperature, ThermodynamicTemperature, Time},
    units::Kelvin,
};
use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{
        CompensationFactor, EquivalentExposure, InvalidValue, ResponseError, ValueKind, validation,
    },
};

use super::{TemperatureHistory, UniformTemperatureObservation};

/// Cumulative equivalent minutes at 43 degrees Celsius.
///
/// For reference temperature `T_ref`, uniform step `dt`, and compensation
/// factor `R(T)`, the discrete exposure is
/// `sum dt R(T_i)^(T_ref - T_i)`.
///
/// # Theorem: canonical reference cases
///
/// With `T_ref = 43 C`, `R = 0.5` at and above the reference, and `R = 0.25`
/// below it, one minute at 43 C contributes one equivalent minute, one minute
/// at 44 C contributes two, and one minute at 42 C contributes one quarter.
///
/// # Theorem: monotonic accumulation
///
/// `dt > 0` and `0 < R <= 1` make every exponential factor and increment
/// positive. Cumulative equivalent exposure is therefore non-negative and
/// non-decreasing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cem43<T> {
    reference: ThermodynamicTemperature<T>,
    at_or_above: CompensationFactor<T>,
    below: CompensationFactor<T>,
}

impl<T: RealField> Cem43<T> {
    /// Construct a cumulative-equivalent-exposure law.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `reference` is not finite and positive.
    pub fn new(
        reference: ThermodynamicTemperature<T>,
        at_or_above: CompensationFactor<T>,
        below: CompensationFactor<T>,
    ) -> Result<Self, InvalidValue<T>> {
        validation::positive(ValueKind::Temperature, *reference.as_base())?;
        Ok(Self {
            reference,
            at_or_above,
            below,
        })
    }

    /// Construct the Sapareto-Dewey 43 degree Celsius convention.
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            reference: ThermodynamicTemperature::from_unit::<Kelvin>(T::from_f64(316.15)),
            at_or_above: CompensationFactor::from_validated(T::from_f64(0.5)),
            below: CompensationFactor::from_validated(T::from_f64(0.25)),
        }
    }

    /// Return the reference temperature.
    #[must_use]
    pub const fn reference(&self) -> ThermodynamicTemperature<T> {
        self.reference
    }

    /// Evaluate one uniform-step exposure increment.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] when the temperature or step lies outside the
    /// law's finite positive domain or the increment is non-finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use aequitas::systems::si::quantities::{ThermodynamicTemperature, Time};
    /// use asclepius::response::thermal::Cem43;
    ///
    /// let increment = Cem43::canonical()
    ///     .increment(
    ///         ThermodynamicTemperature::from_base(317.15_f64),
    ///         Time::from_base(60.0),
    ///     )
    ///     .expect("valid observation");
    /// assert_eq!(increment.get().into_base(), 120.0);
    /// ```
    pub fn increment(
        &self,
        temperature: ThermodynamicTemperature<T>,
        step: Time<T>,
    ) -> Result<EquivalentExposure<T>, ResponseError<T>> {
        validation::positive(ValueKind::Temperature, *temperature.as_base())?;
        validation::positive(ValueKind::TimeStep, *step.as_base())?;
        self.increment_validated(temperature, step)
    }

    /// Evaluate any supported uniform temperature observation.
    ///
    /// Borrowed histories and lazy exact-size sample streams share this method
    /// and the same monomorphized integration kernel.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] for an empty observation, invalid
    /// temperature, or non-finite accumulation.
    pub fn evaluate_uniform<O>(
        &self,
        observation: O,
    ) -> Result<EquivalentExposure<T>, ResponseError<T>>
    where
        O: UniformTemperatureObservation<T>,
    {
        self.integrate(observation, &mut DiscardExposure)
    }

    /// Evaluate cumulative equivalent exposure into caller-owned storage.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] for an empty history, invalid temperature,
    /// non-finite accumulation, or output-length mismatch.
    pub fn cumulative_into<O>(
        &self,
        history: O,
        output: &mut [EquivalentExposure<T>],
    ) -> Result<(), ResponseError<T>>
    where
        O: UniformTemperatureObservation<T>,
    {
        if output.len() != history.len() {
            return Err(ResponseError::OutputLength {
                expected: history.len(),
                actual: output.len(),
            });
        }
        let mut sink = ExposureSlice(output);
        self.integrate(history, &mut sink)?;
        Ok(())
    }

    fn integrate<O, S>(
        &self,
        history: O,
        sink: &mut S,
    ) -> Result<EquivalentExposure<T>, ResponseError<T>>
    where
        O: UniformTemperatureObservation<T>,
        S: ExposureSink<T>,
    {
        if history.is_empty() {
            return Err(ResponseError::EmptyObservation);
        }

        let step = history.step();
        let mut accumulated = Time::from_base(<T as NumericElement>::ZERO);
        for (index, temperature) in history.into_samples().enumerate() {
            validation::positive(ValueKind::Temperature, *temperature.as_base())
                .map_err(|source| ResponseError::InvalidObservation { index, source })?;
            accumulated += self.increment_validated(temperature, step)?.get();
            if !accumulated.as_base().is_finite() {
                return Err(ResponseError::NonFiniteResult {
                    kind: ValueKind::EquivalentExposure,
                    value: *accumulated.as_base(),
                });
            }
            let exposure = EquivalentExposure::new(accumulated)?;
            sink.write(index, exposure);
        }
        EquivalentExposure::new(accumulated).map_err(ResponseError::from)
    }

    fn increment_validated(
        &self,
        temperature: ThermodynamicTemperature<T>,
        step: Time<T>,
    ) -> Result<EquivalentExposure<T>, ResponseError<T>> {
        let inverse_kelvin = ReciprocalTemperature::from_base(<T as NumericElement>::ONE);
        let factor = if temperature >= self.reference {
            self.at_or_above.get()
        } else {
            self.below.get()
        };
        let exponent: Dimensionless<T> = inverse_kelvin * (self.reference - temperature);
        let increment = step * factor.powf(exponent.into_base());
        if !increment.as_base().is_finite() {
            return Err(ResponseError::NonFiniteResult {
                kind: ValueKind::EquivalentExposure,
                value: *increment.as_base(),
            });
        }
        EquivalentExposure::new(increment).map_err(ResponseError::from)
    }
}

impl<T: RealField> BiologicalResponse<T> for Cem43<T> {
    type Observation<'a>
        = TemperatureHistory<'a, T>
    where
        Self: 'a,
        T: 'a;
    type Output = EquivalentExposure<T>;
    type Error = ResponseError<T>;

    fn evaluate<'a>(
        &'a self,
        observation: Self::Observation<'a>,
    ) -> Result<Self::Output, Self::Error>
    where
        T: 'a,
    {
        self.integrate(observation, &mut DiscardExposure)
    }
}

trait ExposureSink<T> {
    fn write(&mut self, index: usize, exposure: EquivalentExposure<T>);
}

struct DiscardExposure;

impl<T> ExposureSink<T> for DiscardExposure {
    #[inline]
    fn write(&mut self, _index: usize, _exposure: EquivalentExposure<T>) {}
}

struct ExposureSlice<'output, T>(&'output mut [EquivalentExposure<T>]);

impl<T: Copy> ExposureSink<T> for ExposureSlice<'_, T> {
    #[inline]
    fn write(&mut self, index: usize, exposure: EquivalentExposure<T>) {
        self.0[index] = exposure;
    }
}
