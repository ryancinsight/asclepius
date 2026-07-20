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
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum AutodiffResponseError {
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
}

impl fmt::Display for AutodiffResponseError {
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
        }
    }
}

impl core::error::Error for AutodiffResponseError {}
