use aequitas::systems::si::quantities::Time;
use eunomia::{NumericElement, RealField};

use super::{InvalidValue, ValueKind, validation};

/// Validated non-negative cumulative equivalent exposure.
///
/// The inner Aequitas time quantity preserves the time dimension while this
/// transparent newtype separates biological equivalent exposure from wall
/// time.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EquivalentExposure<T>(Time<T>);

impl<T: RealField> EquivalentExposure<T> {
    /// Validate and construct an equivalent exposure.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when the time value is negative or non-finite.
    pub fn new(value: Time<T>) -> Result<Self, InvalidValue<T>> {
        validation::non_negative(ValueKind::EquivalentExposure, value.into_base())
            .map(|valid| Self(Time::from_base(valid)))
    }

    /// Construct the additive identity.
    #[must_use]
    pub fn zero() -> Self {
        Self(Time::from_base(<T as NumericElement>::ZERO))
    }

    /// Return the dimensioned equivalent exposure.
    #[must_use]
    pub const fn get(self) -> Time<T> {
        self.0
    }
}
