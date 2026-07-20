use aequitas::systems::si::quantities::{
    Dimensionless, MolarEnergy, MolarHeatCapacity, ReciprocalTime, ThermodynamicTemperature, Time,
};
use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{DamageIntegral, InvalidValue, Probability, ResponseError, ValueKind, validation},
};

use super::{TemperatureHistory, UniformTemperatureObservation};

/// First-order Arrhenius thermal-damage law.
///
/// `Omega(t) = integral A exp(-Ea/(R T(t))) dt`.
///
/// # Theorem: non-negativity and monotonicity
///
/// With `A > 0`, `Ea > 0`, `R > 0`, `T > 0`, and `dt > 0`, the exponential
/// rate and every rectangle-rule increment are positive. Therefore cumulative
/// damage is non-negative and non-decreasing.
///
/// # Theorem: survival relation
///
/// First-order loss obeys `dS/dt = -k(t) S`. Separating variables and
/// integrating gives `S(t)/S(0) = exp(-Omega(t))`; cell-death probability is
/// consequently `1 - exp(-Omega)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrheniusDamage<T> {
    frequency_factor: ReciprocalTime<T>,
    activation_energy: MolarEnergy<T>,
    gas_constant: MolarHeatCapacity<T>,
}

impl<T: RealField> ArrheniusDamage<T> {
    /// Construct an Arrhenius damage law.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when any parameter is not finite and positive.
    pub fn new(
        frequency_factor: ReciprocalTime<T>,
        activation_energy: MolarEnergy<T>,
        gas_constant: MolarHeatCapacity<T>,
    ) -> Result<Self, InvalidValue<T>> {
        validation::positive(ValueKind::FrequencyFactor, *frequency_factor.as_base())?;
        validation::positive(ValueKind::ActivationEnergy, *activation_energy.as_base())?;
        validation::positive(ValueKind::GasConstant, *gas_constant.as_base())?;
        Ok(Self {
            frequency_factor,
            activation_energy,
            gas_constant,
        })
    }

    /// Return the frequency factor.
    #[must_use]
    pub const fn frequency_factor(&self) -> ReciprocalTime<T> {
        self.frequency_factor
    }

    /// Return the activation energy.
    #[must_use]
    pub const fn activation_energy(&self) -> MolarEnergy<T> {
        self.activation_energy
    }

    /// Return the molar gas constant.
    #[must_use]
    pub const fn gas_constant(&self) -> MolarHeatCapacity<T> {
        self.gas_constant
    }

    /// Evaluate the instantaneous first-order damage rate.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] when the absolute temperature is not finite
    /// and positive.
    pub fn rate(
        &self,
        temperature: ThermodynamicTemperature<T>,
    ) -> Result<ReciprocalTime<T>, ResponseError<T>> {
        validation::positive(ValueKind::Temperature, *temperature.as_base())?;
        Ok(self.rate_validated(temperature))
    }

    /// Evaluate one rectangle-rule damage increment.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] when the temperature or step lies outside the
    /// finite positive domain.
    pub fn increment(
        &self,
        temperature: ThermodynamicTemperature<T>,
        step: Time<T>,
    ) -> Result<DamageIntegral<T>, ResponseError<T>> {
        validation::positive(ValueKind::Temperature, *temperature.as_base())?;
        validation::positive(ValueKind::TimeStep, *step.as_base())?;
        self.increment_validated(temperature, step)
    }

    /// Convert accumulated damage to surviving fraction.
    #[must_use]
    pub fn survival(damage: DamageIntegral<T>) -> Probability<T> {
        Probability::from_validated((-damage.get()).exp())
    }

    /// Convert accumulated damage to cell-death probability.
    #[must_use]
    pub fn kill_probability(damage: DamageIntegral<T>) -> Probability<T> {
        let one = <T as NumericElement>::ONE;
        Probability::from_validated(one - (-damage.get()).exp())
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
    pub fn evaluate_uniform<O>(&self, observation: O) -> Result<DamageIntegral<T>, ResponseError<T>>
    where
        O: UniformTemperatureObservation<T>,
    {
        self.integrate(observation, &mut DiscardDamage)
    }

    /// Evaluate cumulative damage into caller-owned storage.
    ///
    /// The output at index `i` is damage accumulated through sample `i`.
    /// No allocation or input copy occurs.
    ///
    /// # Errors
    ///
    /// Returns [`ResponseError`] for an empty history, invalid temperature,
    /// non-finite accumulation, or output-length mismatch.
    pub fn cumulative_into<O>(
        &self,
        history: O,
        output: &mut [DamageIntegral<T>],
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
        let mut sink = DamageSlice(output);
        self.integrate(history, &mut sink)?;
        Ok(())
    }

    fn integrate<O, S>(
        &self,
        history: O,
        sink: &mut S,
    ) -> Result<DamageIntegral<T>, ResponseError<T>>
    where
        O: UniformTemperatureObservation<T>,
        S: DamageSink<T>,
    {
        if history.is_empty() {
            return Err(ResponseError::EmptyObservation);
        }

        let step = history.step();
        let mut accumulated = <T as NumericElement>::ZERO;
        for (index, temperature) in history.into_samples().enumerate() {
            validation::positive(ValueKind::Temperature, *temperature.as_base())
                .map_err(|source| ResponseError::InvalidObservation { index, source })?;
            accumulated += self.increment_validated(temperature, step)?.get();
            if !accumulated.is_finite() {
                return Err(ResponseError::NonFiniteResult {
                    kind: ValueKind::DamageIntegral,
                    value: accumulated,
                });
            }
            let damage = DamageIntegral::new(accumulated)?;
            sink.write(index, damage);
        }
        DamageIntegral::new(accumulated).map_err(ResponseError::from)
    }

    fn rate_validated(&self, temperature: ThermodynamicTemperature<T>) -> ReciprocalTime<T> {
        let denominator: MolarEnergy<T> = self.gas_constant * temperature;
        let normalized: Dimensionless<T> = self.activation_energy / denominator;
        self.frequency_factor * (-normalized.into_base()).exp()
    }

    fn increment_validated(
        &self,
        temperature: ThermodynamicTemperature<T>,
        step: Time<T>,
    ) -> Result<DamageIntegral<T>, ResponseError<T>> {
        let increment: Dimensionless<T> = self.rate_validated(temperature) * step;
        let increment = increment.into_base();
        if !increment.is_finite() {
            return Err(ResponseError::NonFiniteResult {
                kind: ValueKind::DamageIntegral,
                value: increment,
            });
        }
        DamageIntegral::new(increment).map_err(ResponseError::from)
    }
}

impl<T: RealField> BiologicalResponse<T> for ArrheniusDamage<T> {
    type Observation<'a>
        = TemperatureHistory<'a, T>
    where
        Self: 'a,
        T: 'a;
    type Output = DamageIntegral<T>;
    type Error = ResponseError<T>;

    fn evaluate<'a>(
        &'a self,
        observation: Self::Observation<'a>,
    ) -> Result<Self::Output, Self::Error>
    where
        T: 'a,
    {
        self.integrate(observation, &mut DiscardDamage)
    }
}

trait DamageSink<T> {
    fn write(&mut self, index: usize, damage: DamageIntegral<T>);
}

struct DiscardDamage;

impl<T> DamageSink<T> for DiscardDamage {
    #[inline]
    fn write(&mut self, _index: usize, _damage: DamageIntegral<T>) {}
}

struct DamageSlice<'output, T>(&'output mut [DamageIntegral<T>]);

impl<T: Copy> DamageSink<T> for DamageSlice<'_, T> {
    #[inline]
    fn write(&mut self, index: usize, damage: DamageIntegral<T>) {
        self.0[index] = damage;
    }
}
