use aequitas::systems::si::quantities::AbsorbedDose;
use eunomia::RealField;

use crate::{
    BiologicalResponse,
    value::{InvalidValue, LymanSlope, Probability, ResponseError},
};

use super::validation;

/// Lyman normal-tissue complication probability evaluated at uniform dose.
///
/// `t = (D - TD50)/(m TD50)` and `NTCP = Phi(t)`.
///
/// # Theorem: midpoint and monotonicity
///
/// At `D = TD50`, `t = 0` and the standard normal CDF is `1/2`. Because
/// `m TD50 > 0` and the standard normal density is positive, the chain-rule
/// derivative of NTCP with respect to dose is positive. The CDF range proves
/// `0 <= NTCP <= 1`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LymanComplicationProbability<T> {
    midpoint: AbsorbedDose<T>,
    m: LymanSlope<T>,
}

impl<T: RealField> LymanComplicationProbability<T> {
    /// Construct a normal-tissue complication law.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `midpoint` is not finite and positive.
    ///
    /// # Examples
    ///
    /// The Lyman normalized slope `m` is the law's own parameter:
    ///
    /// ```
    /// # use aequitas::systems::si::{quantities::AbsorbedDose, units::Gray};
    /// # use asclepius::LymanSlope;
    /// # use asclepius::response::radiation::LymanComplicationProbability;
    /// let m = LymanSlope::new(0.2_f64).expect("positive Lyman slope");
    /// let law = LymanComplicationProbability::new(
    ///     AbsorbedDose::from_unit::<Gray>(50.0),
    ///     m,
    /// )?;
    /// assert_eq!(law.m().get(), 0.2);
    /// # Ok::<(), asclepius::InvalidValue<f64>>(())
    /// ```
    ///
    /// A Niemierko `gamma50` is a different quantity and does not substitute
    /// for it. The call below differs from the one above only in the type of
    /// the second argument, and does not compile:
    ///
    /// ```compile_fail
    /// # use aequitas::systems::si::{quantities::AbsorbedDose, units::Gray};
    /// # use asclepius::Gamma50;
    /// # use asclepius::response::radiation::LymanComplicationProbability;
    /// let gamma50 = Gamma50::new(2.0_f64).expect("positive gamma50");
    /// let _ = LymanComplicationProbability::new(
    ///     AbsorbedDose::from_unit::<Gray>(50.0),
    ///     gamma50,
    /// );
    /// ```
    pub fn new(midpoint: AbsorbedDose<T>, m: LymanSlope<T>) -> Result<Self, InvalidValue<T>> {
        validation::positive_dose(midpoint)?;
        Ok(Self { midpoint, m })
    }

    /// Return the 50% complication dose.
    #[must_use]
    pub const fn midpoint(&self) -> AbsorbedDose<T> {
        self.midpoint
    }

    /// Return the Lyman normalized slope `m`.
    #[must_use]
    pub const fn m(&self) -> LymanSlope<T> {
        self.m
    }
}

impl<T: RealField> BiologicalResponse<T> for LymanComplicationProbability<T> {
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
        let midpoint = *self.midpoint.as_base();
        let half = T::from_f64(0.5);
        let inverse_sqrt_two = T::from_f64(core::f64::consts::FRAC_1_SQRT_2);
        let normalized = (dose - midpoint) / (self.m.get() * midpoint);
        Probability::new(half * (-normalized * inverse_sqrt_two).erfc())
            .map_err(ResponseError::from)
    }
}
