use core::fmt;

/// Biological value whose validity constraint is being checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueKind {
    /// Absorbed dose.
    AbsorbedDose,
    /// Probability.
    Probability,
    /// Arrhenius damage integral.
    DamageIntegral,
    /// Generalized-mean volume-effect exponent.
    VolumeEffect,
    /// Positive outcome-response slope.
    ResponseSlope,
    /// CEM temperature-compensation factor.
    CompensationFactor,
    /// Arrhenius frequency factor.
    FrequencyFactor,
    /// Arrhenius activation energy.
    ActivationEnergy,
    /// Molar gas constant.
    GasConstant,
    /// Integration time step.
    TimeStep,
    /// Absolute thermodynamic temperature.
    Temperature,
    /// Cumulative equivalent exposure.
    EquivalentExposure,
}

/// Required mathematical domain of a biological value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueConstraint {
    /// Finite and greater than or equal to zero.
    FiniteNonNegative,
    /// Finite and strictly greater than zero.
    FinitePositive,
    /// Finite and not equal to zero.
    FiniteNonZero,
    /// Finite and in the closed interval `[0, 1]`.
    UnitInterval,
    /// Finite and in the half-open interval `(0, 1]`.
    PositiveUnitInterval,
}

/// A scalar violates a biological value's mathematical domain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InvalidValue<T> {
    kind: ValueKind,
    value: T,
    constraint: ValueConstraint,
}

impl<T> InvalidValue<T> {
    pub(crate) const fn new(kind: ValueKind, value: T, constraint: ValueConstraint) -> Self {
        Self {
            kind,
            value,
            constraint,
        }
    }

    /// Return the invalid value's domain role.
    #[must_use]
    pub const fn kind(&self) -> ValueKind {
        self.kind
    }

    /// Borrow the rejected scalar.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Return the violated constraint.
    #[must_use]
    pub const fn constraint(&self) -> ValueConstraint {
        self.constraint
    }
}

impl<T: fmt::Display> fmt::Display for InvalidValue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} value {} violates {:?}",
            self.kind, self.value, self.constraint
        )
    }
}

impl<T> core::error::Error for InvalidValue<T> where T: fmt::Debug + fmt::Display {}

/// Failure while evaluating a biological-response law.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum ResponseError<T> {
    /// A model parameter or scalar observation is invalid.
    InvalidValue(InvalidValue<T>),
    /// A sample-dependent observation is invalid.
    InvalidObservation {
        /// Zero-based location of the invalid observation.
        index: usize,
        /// Violated scalar domain.
        source: InvalidValue<T>,
    },
    /// A response law requiring samples received none.
    EmptyObservation,
    /// The observation count exceeds the exactly representable normalization
    /// range accepted by the implementation.
    ObservationTooLong {
        /// Supplied observation count.
        length: usize,
        /// Maximum accepted count.
        maximum: u32,
    },
    /// A caller-provided output buffer has the wrong length.
    OutputLength {
        /// Required output length.
        expected: usize,
        /// Supplied output length.
        actual: usize,
    },
    /// Valid inputs produced a non-finite result.
    NonFiniteResult {
        /// Result role.
        kind: ValueKind,
        /// Non-finite scalar.
        value: T,
    },
}

impl<T> From<InvalidValue<T>> for ResponseError<T> {
    fn from(value: InvalidValue<T>) -> Self {
        Self::InvalidValue(value)
    }
}

impl<T: fmt::Display> fmt::Display for ResponseError<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(source) => source.fmt(formatter),
            Self::InvalidObservation { index, source } => {
                write!(formatter, "observation {index}: {source}")
            }
            Self::EmptyObservation => formatter.write_str("the observation sample is empty"),
            Self::ObservationTooLong { length, maximum } => write!(
                formatter,
                "observation count {length} exceeds supported maximum {maximum}"
            ),
            Self::OutputLength { expected, actual } => write!(
                formatter,
                "output length {actual} does not match observation length {expected}"
            ),
            Self::NonFiniteResult { kind, value } => {
                write!(
                    formatter,
                    "{kind:?} evaluation produced non-finite value {value}"
                )
            }
        }
    }
}

impl<T> core::error::Error for ResponseError<T> where T: fmt::Debug + fmt::Display {}
