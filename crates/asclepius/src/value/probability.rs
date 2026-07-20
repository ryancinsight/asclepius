use eunomia::RealField;

use super::{InvalidValue, ValueKind, validation};

/// Validated probability in the closed interval `[0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Probability<T>(T);

impl<T: RealField> Probability<T> {
    /// Validate and construct a probability.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `value` is non-finite or outside `[0, 1]`.
    pub fn new(value: T) -> Result<Self, InvalidValue<T>> {
        validation::unit_interval(ValueKind::Probability, value).map(Self)
    }

    /// Return the scalar probability.
    #[must_use]
    pub const fn get(self) -> T {
        self.0
    }

    pub(crate) const fn from_validated(value: T) -> Self {
        Self(value)
    }
}
