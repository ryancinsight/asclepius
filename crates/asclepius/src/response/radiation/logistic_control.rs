use aequitas::systems::si::quantities::AbsorbedDose;
use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{InvalidValue, Probability, ResponseError, ResponseSlope},
};

use super::validation;

/// Niemierko logistic tumour-control probability.
///
/// `TCP = 1 / (1 + (D50 / D)^(4 gamma50))`.
///
/// # Theorem: midpoint and bounds
///
/// Positive `D50` and `gamma50` make the denominator at least one. Therefore
/// `0 <= TCP <= 1`. At `D = D50`, the ratio is one and `TCP = 1/2`.
/// Differentiating for `D > 0` gives a positive derivative, so TCP is monotone
/// increasing in dose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogisticControlProbability<T> {
    midpoint: AbsorbedDose<T>,
    slope: ResponseSlope<T>,
}

impl<T: RealField> LogisticControlProbability<T> {
    /// Construct a logistic tumour-control law.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `midpoint` is not finite and positive.
    pub fn new(
        midpoint: AbsorbedDose<T>,
        slope: ResponseSlope<T>,
    ) -> Result<Self, InvalidValue<T>> {
        validation::positive_dose(midpoint)?;
        Ok(Self { midpoint, slope })
    }

    /// Return the 50% control dose.
    #[must_use]
    pub const fn midpoint(&self) -> AbsorbedDose<T> {
        self.midpoint
    }

    /// Return the normalized response slope.
    #[must_use]
    pub const fn slope(&self) -> ResponseSlope<T> {
        self.slope
    }
}

impl<T: RealField> BiologicalResponse<T> for LogisticControlProbability<T> {
    type Observation<'a>
        = AbsorbedDose<T>
    where
        Self: 'a,
        T: 'a;
    type Output = Probability<T>;
    type Error = ResponseError<T>;

    fn evaluate<'a>(
        &'a self,
        observation: Self::Observation<'a>,
    ) -> Result<Self::Output, Self::Error>
    where
        T: 'a,
    {
        let dose = validation::non_negative_dose(observation)?;
        let one = <T as NumericElement>::ONE;
        let four = T::from_f64(4.0);
        let ratio = (*self.midpoint.as_base() / dose).powf(four * self.slope.get());
        Probability::new((one + ratio).recip()).map_err(ResponseError::from)
    }
}
