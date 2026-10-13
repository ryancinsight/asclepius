use core::fmt;

/// Constraint violated by a raw dose tensor element.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DoseConstraint {
    /// Dose must be finite.
    Finite,
    /// Dose must be greater than or equal to zero.
    NonNegative,
    /// A negative power-mean exponent requires every dose to be positive.
    PositiveForNegativeExponent,
}

impl fmt::Display for DoseConstraint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite => formatter.write_str("finite"),
            Self::NonNegative => formatter.write_str("non-negative"),
            Self::PositiveForNegativeExponent => {
                formatter.write_str("positive for a negative volume-effect exponent")
            }
        }
    }
}

/// Failure to construct a differentiable biological response.
///
/// `E` is the Coeus backend error (`B::Error`) raised while building the
/// differentiable graph; it defaults to [`coeus_core::BackendError`], the
/// error of every backend Coeus ships.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum AutodiffResponseError<E = coeus_core::BackendError> {
    /// The dose observation contains no elements.
    EmptyObservation,
    /// A dose element violates the model domain.
    InvalidDose {
        /// Logical flat index of the offending dose.
        index: usize,
        /// Offending dose value.
        value: f64,
        /// Required domain constraint.
        constraint: DoseConstraint,
    },
    /// The Coeus backend failed while building the response graph.
    Backend(E),
}

impl<E> From<E> for AutodiffResponseError<E> {
    fn from(error: E) -> Self {
        Self::Backend(error)
    }
}

impl<E: fmt::Display> fmt::Display for AutodiffResponseError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservation => formatter.write_str("dose observation must not be empty"),
            Self::InvalidDose {
                index,
                value,
                constraint,
            } => write!(
                formatter,
                "dose at logical index {index} is {value} but must be {constraint}"
            ),
            Self::Backend(error) => write!(formatter, "coeus backend failure: {error}"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for AutodiffResponseError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            Self::EmptyObservation | Self::InvalidDose { .. } => None,
        }
    }
}
