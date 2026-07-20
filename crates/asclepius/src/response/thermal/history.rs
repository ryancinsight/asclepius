use aequitas::systems::si::quantities::{ThermodynamicTemperature, Time};
use eunomia::RealField;

use crate::value::{InvalidValue, ValueKind, validation};

mod sealed {
    pub trait Sealed {}
}

/// Uniformly sampled absolute-temperature observation.
///
/// The associated iterator is consumed exactly once. Asclepius provides
/// implementations for borrowed [`TemperatureHistory`] and arbitrary
/// exact-size [`TemperatureSamples`] iterators so consumers can select borrowed
/// storage or a lazy unit-conversion pipeline without allocation or copying.
///
/// # Examples
///
/// ```
/// use aequitas::systems::si::quantities::{ThermodynamicTemperature, Time};
/// use asclepius::response::thermal::{
///     TemperatureHistory, UniformTemperatureObservation,
/// };
///
/// fn observation_len<O: UniformTemperatureObservation<f64>>(observation: &O) -> usize {
///     observation.len()
/// }
///
/// let samples = [ThermodynamicTemperature::from_base(316.15)];
/// let observation =
///     TemperatureHistory::new(&samples, Time::from_base(1.0)).expect("positive step");
/// assert_eq!(observation_len(&observation), 1);
/// ```
pub trait UniformTemperatureObservation<T: RealField>: sealed::Sealed {
    /// Exact-size stream of absolute-temperature samples.
    type Samples: ExactSizeIterator<Item = ThermodynamicTemperature<T>>;

    /// Return the number of samples that evaluation will consume.
    #[must_use]
    fn len(&self) -> usize;

    /// Return whether the observation contains no samples.
    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the uniform time step.
    #[must_use]
    fn step(&self) -> Time<T>;

    /// Consume the observation and return its one-pass sample stream.
    #[must_use]
    fn into_samples(self) -> Self::Samples;
}

/// Borrowed, uniformly sampled absolute-temperature history.
#[derive(Clone, Copy, Debug)]
pub struct TemperatureHistory<'sample, T> {
    samples: &'sample [ThermodynamicTemperature<T>],
    step: Time<T>,
}

impl<'sample, T: RealField> TemperatureHistory<'sample, T> {
    /// Construct a borrowed temperature history.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `step` is not finite and positive.
    pub fn new(
        samples: &'sample [ThermodynamicTemperature<T>],
        step: Time<T>,
    ) -> Result<Self, InvalidValue<T>> {
        validation::positive(ValueKind::TimeStep, *step.as_base())?;
        Ok(Self { samples, step })
    }

    /// Borrow the absolute-temperature samples.
    #[must_use]
    pub const fn samples(&self) -> &'sample [ThermodynamicTemperature<T>] {
        self.samples
    }

    /// Return the uniform time step.
    #[must_use]
    pub const fn step(&self) -> Time<T> {
        self.step
    }
}

impl<T> sealed::Sealed for TemperatureHistory<'_, T> {}

impl<'sample, T: RealField> UniformTemperatureObservation<T> for TemperatureHistory<'sample, T> {
    type Samples = core::iter::Copied<core::slice::Iter<'sample, ThermodynamicTemperature<T>>>;

    fn len(&self) -> usize {
        self.samples.len()
    }

    fn step(&self) -> Time<T> {
        self.step
    }

    fn into_samples(self) -> Self::Samples {
        self.samples.iter().copied()
    }
}

/// One-pass, uniformly sampled absolute-temperature stream.
///
/// `I` remains inline in this value and is monomorphized into the response
/// kernel. Mapping a borrowed scalar slice into Aequitas temperatures therefore
/// requires neither an intermediate collection nor dynamic dispatch.
///
/// # Examples
///
/// ```
/// use aequitas::systems::si::quantities::{ThermodynamicTemperature, Time};
/// use asclepius::response::thermal::{Cem43, TemperatureSamples};
///
/// let celsius = [43.0_f64, 44.0];
/// let observation = TemperatureSamples::new(
///     celsius
///         .iter()
///         .copied()
///         .map(|value| ThermodynamicTemperature::from_base(value + 273.15)),
///     Time::from_base(60.0),
/// )
/// .expect("positive step");
/// let exposure = Cem43::canonical()
///     .evaluate_uniform(observation)
///     .expect("positive temperatures");
/// assert_eq!(exposure.get().into_base(), 180.0);
/// ```
#[derive(Clone, Debug)]
#[must_use]
pub struct TemperatureSamples<I, T> {
    samples: I,
    step: Time<T>,
}

impl<I, T> TemperatureSamples<I, T>
where
    I: ExactSizeIterator<Item = ThermodynamicTemperature<T>>,
    T: RealField,
{
    /// Construct a one-pass temperature observation.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `step` is not finite and positive.
    pub fn new(samples: I, step: Time<T>) -> Result<Self, InvalidValue<T>> {
        validation::positive(ValueKind::TimeStep, *step.as_base())?;
        Ok(Self { samples, step })
    }
}

impl<I, T> sealed::Sealed for TemperatureSamples<I, T> {}

impl<I, T> UniformTemperatureObservation<T> for TemperatureSamples<I, T>
where
    I: ExactSizeIterator<Item = ThermodynamicTemperature<T>>,
    T: RealField,
{
    type Samples = I;

    fn len(&self) -> usize {
        self.samples.len()
    }

    fn step(&self) -> Time<T> {
        self.step
    }

    fn into_samples(self) -> Self::Samples {
        self.samples
    }
}
