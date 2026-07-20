use aequitas::systems::si::quantities::AbsorbedDose;
use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{ResponseError, ValueKind, VolumeEffect},
};

use super::validation;

/// Generalized equivalent uniform dose (gEUD) power-mean law.
///
/// For absorbed doses `D_i >= 0` and finite non-zero exponent `a`,
///
/// `gEUD = ((1/N) sum_i D_i^a)^(1/a)`.
///
/// # Theorem: generalized-mean bounds
///
/// For positive observations and finite `a != 0`, the generalized-mean
/// inequality gives `min(D) <= gEUD <= max(D)`. A uniform observation therefore
/// returns that dose exactly up to native floating-point rounding.
///
/// # Theorem: positive homogeneity
///
/// For `c >= 0`, substituting `c D_i` and factoring `c^a` out of the mean gives
/// `gEUD(c D) = c gEUD(D)`. The implementation performs this factorization
/// explicitly by normalizing with the maximum dose for positive `a` and the
/// minimum dose for negative `a`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeneralizedEquivalentUniformDose<T> {
    volume_effect: VolumeEffect<T>,
}

impl<T: RealField> GeneralizedEquivalentUniformDose<T> {
    /// Construct from a validated volume-effect exponent.
    #[must_use]
    pub const fn new(volume_effect: VolumeEffect<T>) -> Self {
        Self { volume_effect }
    }

    /// Return the volume-effect exponent.
    #[must_use]
    pub const fn volume_effect(&self) -> VolumeEffect<T> {
        self.volume_effect
    }

    fn evaluate_sample(
        &self,
        doses: &[AbsorbedDose<T>],
    ) -> Result<AbsorbedDose<T>, ResponseError<T>> {
        if doses.is_empty() {
            return Err(ResponseError::EmptyObservation);
        }

        let count = u32::try_from(doses.len()).map_err(|_| ResponseError::ObservationTooLong {
            length: doses.len(),
            maximum: u32::MAX,
        })?;
        let zero = <T as NumericElement>::ZERO;
        let exponent = self.volume_effect.get();

        let first = validation::non_negative_dose(doses[0])
            .map_err(|source| ResponseError::InvalidObservation { index: 0, source })?;
        if exponent < zero && first <= zero {
            return Err(ResponseError::InvalidObservation {
                index: 0,
                source: crate::value::InvalidValue::new(
                    ValueKind::AbsorbedDose,
                    first,
                    crate::value::ValueConstraint::FinitePositive,
                ),
            });
        }

        let mut scale = first;
        for (index, &dose) in doses.iter().enumerate().skip(1) {
            let value = validation::non_negative_dose(dose)
                .map_err(|source| ResponseError::InvalidObservation { index, source })?;
            if exponent < zero && value <= zero {
                return Err(ResponseError::InvalidObservation {
                    index,
                    source: crate::value::InvalidValue::new(
                        ValueKind::AbsorbedDose,
                        value,
                        crate::value::ValueConstraint::FinitePositive,
                    ),
                });
            }
            scale = if exponent > zero {
                scale.max_scalar(value)
            } else {
                scale.min_scalar(value)
            };
        }

        if scale == zero {
            return Ok(AbsorbedDose::from_base(zero));
        }

        let sum = doses.iter().fold(zero, |accumulator, dose| {
            let normalized = *dose.as_base() / scale;
            accumulator + normalized.powf(exponent)
        });
        let inverse_count = T::from_f64(f64::from(count)).recip();
        let result = scale * (sum * inverse_count).powf(exponent.recip());
        if result.is_finite() {
            Ok(AbsorbedDose::from_base(result))
        } else {
            Err(ResponseError::NonFiniteResult {
                kind: ValueKind::AbsorbedDose,
                value: result,
            })
        }
    }
}

impl<T: RealField> BiologicalResponse<T> for GeneralizedEquivalentUniformDose<T> {
    type Observation<'a>
        = &'a [AbsorbedDose<T>]
    where
        Self: 'a,
        T: 'a;
    type Output = AbsorbedDose<T>;
    type Error = ResponseError<T>;

    #[inline]
    fn evaluate<'a>(
        &'a self,
        observation: Self::Observation<'a>,
    ) -> Result<Self::Output, Self::Error>
    where
        T: 'a,
    {
        self.evaluate_sample(observation)
    }
}
