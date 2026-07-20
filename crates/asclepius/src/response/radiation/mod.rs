//! Ionizing-radiation dose-response laws.
//!
//! The generalized equivalent uniform dose implementation evaluates the power
//! mean after scaling by an extremal dose. This preserves homogeneity and the
//! generalized-mean bounds while reducing overflow and underflow relative to
//! evaluating `D_i^a` directly.

mod equivalent_uniform_dose;
mod logistic_control;
mod normal_complication;
mod validation;

pub use equivalent_uniform_dose::GeneralizedEquivalentUniformDose;
pub use logistic_control::LogisticControlProbability;
pub use normal_complication::LymanComplicationProbability;
