use eunomia::{NumericElement, RealField};

use super::{InvalidValue, ValueKind, validation};

/// Validated non-negative dimensionless Arrhenius damage integral.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DamageIntegral<T>(T);

impl<T: RealField> DamageIntegral<T> {
    /// Validate and construct a damage integral.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `value` is non-finite or negative.
    pub fn new(value: T) -> Result<Self, InvalidValue<T>> {
        validation::non_negative(ValueKind::DamageIntegral, value).map(Self)
    }

    /// Construct the additive identity.
    #[must_use]
    pub fn zero() -> Self {
        Self(<T as NumericElement>::ZERO)
    }

    /// Return the dimensionless damage scalar.
    #[must_use]
    pub const fn get(self) -> T {
        self.0
    }
}
