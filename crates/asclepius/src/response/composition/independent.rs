use eunomia::{NumericElement, RealField};

use crate::{
    BiologicalResponse,
    value::{Probability, ResponseError},
};

/// Const-generic composition of statistically independent cell-kill insults.
///
/// The type is a zero-sized static strategy. For kill probabilities `p_i`,
/// independent survival probabilities multiply:
///
/// `p_combined = 1 - product_i (1 - p_i)`.
///
/// # Theorem: bounds and monotonicity
///
/// Every factor `1 - p_i` lies in `[0, 1]`, so their product lies in `[0, 1]`
/// and the combined probability also lies in `[0, 1]`. Its partial derivative
/// with respect to `p_j` is `product_(i != j) (1 - p_i) >= 0`, proving that
/// adding or increasing an insult cannot reduce combined cell kill.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IndependentInsults<const N: usize>;

impl<T: RealField, const N: usize> BiologicalResponse<T> for IndependentInsults<N> {
    type Observation<'a>
        = &'a [Probability<T>; N]
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
        let one = <T as NumericElement>::ONE;
        let survival = observation.iter().fold(one, |product, probability| {
            product * (one - probability.get())
        });
        Probability::new(one - survival).map_err(ResponseError::from)
    }
}
