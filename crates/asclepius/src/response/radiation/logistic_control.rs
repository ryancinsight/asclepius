use aequitas::systems::si::quantities::AbsorbedDose;
use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{Gamma50, InvalidValue, Probability, ResponseError},
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
    gamma50: Gamma50<T>,
}

impl<T: RealField> LogisticControlProbability<T> {
    /// Construct a logistic tumour-control law.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `midpoint` is not finite and positive.
    ///
    /// ```compile_fail
    /// # use aequitas::systems::si::{quantities::AbsorbedDose, units::Gray};
    /// # use asclepius::{Gamma50, LymanSlope};
    /// # use asclepius::response::radiation::LogisticControlProbability;
    /// let m = LymanSlope::new(0.2_f64).expect("positive Lyman slope");
    /// let _ = LogisticControlProbability::new(
    ///     AbsorbedDose::from_unit::<Gray>(50.0),
    ///     m,
    /// );
    /// ```
    pub fn new(midpoint: AbsorbedDose<T>, gamma50: Gamma50<T>) -> Result<Self, InvalidValue<T>> {
        validation::positive_dose(midpoint)?;
        Ok(Self { midpoint, gamma50 })
    }

    /// Return the 50% control dose.
    #[must_use]
    pub const fn midpoint(&self) -> AbsorbedDose<T> {
        self.midpoint
    }

    /// Return the Niemierko `gamma50` parameter.
    #[must_use]
    pub const fn gamma50(&self) -> Gamma50<T> {
        self.gamma50
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
        let ratio = (*self.midpoint.as_base() / dose).powf(four * self.gamma50.get());
        Probability::new((one + ratio).recip()).map_err(ResponseError::from)
    }
}
