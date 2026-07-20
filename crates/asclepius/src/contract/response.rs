use eunomia::RealField;

/// Statically dispatched biological-response law.
///
/// The generic associated observation family permits each evaluation to
/// borrow dose samples, temperature histories, or fixed response arrays
/// without allocation or cloning.
#[diagnostic::on_unimplemented(
    message = "this type does not implement an Asclepius biological-response law",
    note = "implement BiologicalResponse<T> and define its borrowed Observation<'a> family"
)]
pub trait BiologicalResponse<T: RealField> {
    /// Observation view consumed by one evaluation.
    type Observation<'a>
    where
        Self: 'a,
        T: 'a;

    /// Successful response value.
    type Output;

    /// Evaluation failure.
    type Error;

    /// Evaluate the response law.
    ///
    /// # Errors
    ///
    /// Returns the implementation's typed failure when the observation lies
    /// outside the law's mathematical domain.
    fn evaluate<'a>(
        &'a self,
        observation: Self::Observation<'a>,
    ) -> Result<Self::Output, Self::Error>
    where
        T: 'a;
}
