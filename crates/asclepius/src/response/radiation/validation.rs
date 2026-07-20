use aequitas::systems::si::quantities::AbsorbedDose;
use eunomia::RealField;

use crate::value::{InvalidValue, ValueKind, validation};

#[inline]
pub(super) fn non_negative_dose<T: RealField>(dose: AbsorbedDose<T>) -> Result<T, InvalidValue<T>> {
    validation::non_negative(ValueKind::AbsorbedDose, dose.into_base())
}

#[inline]
pub(super) fn positive_dose<T: RealField>(dose: AbsorbedDose<T>) -> Result<T, InvalidValue<T>> {
    validation::positive(ValueKind::AbsorbedDose, dose.into_base())
}
