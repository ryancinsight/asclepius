use aequitas::systems::si::quantities::{ThermodynamicTemperature, Time};
use eunomia::RealField;

use crate::value::{InvalidValue, ValueKind, validation};

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
