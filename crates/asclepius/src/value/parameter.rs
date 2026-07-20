use eunomia::RealField;

use super::{InvalidValue, ValueKind, validation};

/// Validated finite non-zero generalized-mean volume-effect exponent.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VolumeEffect<T>(T);

impl<T: RealField> VolumeEffect<T> {
    /// Validate a volume-effect exponent.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `value` is zero or non-finite.
    pub fn new(value: T) -> Result<Self, InvalidValue<T>> {
        validation::non_zero(ValueKind::VolumeEffect, value).map(Self)
    }

    /// Return the exponent.
    #[must_use]
    pub const fn get(self) -> T {
        self.0
    }
}

/// Validated finite positive dose-response slope.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResponseSlope<T>(T);

impl<T: RealField> ResponseSlope<T> {
    /// Validate a response slope.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `value` is not finite and positive.
    pub fn new(value: T) -> Result<Self, InvalidValue<T>> {
        validation::positive(ValueKind::ResponseSlope, value).map(Self)
    }

    /// Return the slope.
    #[must_use]
    pub const fn get(self) -> T {
        self.0
    }
}

/// Validated finite CEM temperature-compensation factor in `(0, 1]`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CompensationFactor<T>(T);

impl<T: RealField> CompensationFactor<T> {
    /// Validate a compensation factor.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `value` is non-finite or outside `(0, 1]`.
    pub fn new(value: T) -> Result<Self, InvalidValue<T>> {
        validation::positive_unit_interval(ValueKind::CompensationFactor, value).map(Self)
    }

    /// Return the factor.
    #[must_use]
    pub const fn get(self) -> T {
        self.0
    }

    pub(crate) const fn from_validated(value: T) -> Self {
        Self(value)
    }
}
