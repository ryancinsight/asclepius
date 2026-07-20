use eunomia::{NumericElement, RealField};

use super::{InvalidValue, ValueConstraint, ValueKind};

#[inline]
pub(crate) fn non_negative<T: RealField>(kind: ValueKind, value: T) -> Result<T, InvalidValue<T>> {
    if value.is_finite() && value >= <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(InvalidValue::new(
            kind,
            value,
            ValueConstraint::FiniteNonNegative,
        ))
    }
}

#[inline]
pub(crate) fn positive<T: RealField>(kind: ValueKind, value: T) -> Result<T, InvalidValue<T>> {
    if value.is_finite() && value > <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(InvalidValue::new(
            kind,
            value,
            ValueConstraint::FinitePositive,
        ))
    }
}

#[inline]
pub(crate) fn non_zero<T: RealField>(kind: ValueKind, value: T) -> Result<T, InvalidValue<T>> {
    if value.is_finite() && value != <T as NumericElement>::ZERO {
        Ok(value)
    } else {
        Err(InvalidValue::new(
            kind,
            value,
            ValueConstraint::FiniteNonZero,
        ))
    }
}

#[inline]
pub(crate) fn unit_interval<T: RealField>(kind: ValueKind, value: T) -> Result<T, InvalidValue<T>> {
    let zero = <T as NumericElement>::ZERO;
    let one = <T as NumericElement>::ONE;
    if value.is_finite() && value >= zero && value <= one {
        Ok(value)
    } else {
        Err(InvalidValue::new(
            kind,
            value,
            ValueConstraint::UnitInterval,
        ))
    }
}

#[inline]
pub(crate) fn positive_unit_interval<T: RealField>(
    kind: ValueKind,
    value: T,
) -> Result<T, InvalidValue<T>> {
    let zero = <T as NumericElement>::ZERO;
    let one = <T as NumericElement>::ONE;
    if value.is_finite() && value > zero && value <= one {
        Ok(value)
    } else {
        Err(InvalidValue::new(
            kind,
            value,
            ValueConstraint::PositiveUnitInterval,
        ))
    }
}
