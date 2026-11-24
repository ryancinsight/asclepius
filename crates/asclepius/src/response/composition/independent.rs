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
///
/// Evaluation accumulates the union probability directly with
/// `c_next = c + (1 - c) * p`, starting at zero. Algebraically,
/// `1 - c_next = (1 - c) * (1 - p)`, so induction gives the same independent
/// survival product without subtracting that product from one at the end.
/// This avoids losing all small positive probabilities when `1 - p` rounds
/// to one. A singleton preserves its represented probability, including
/// subnormals. Arithmetic stays in `T`; rounding accumulates across multiple
/// inputs, so this is not a correctly rounded arbitrary-length product.
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
        let combined = observation
            .iter()
            .fold(<T as NumericElement>::ZERO, |combined, probability| {
                combined + (one - combined) * probability.get()
            });
        Probability::new(combined).map_err(ResponseError::from)
    }
}
