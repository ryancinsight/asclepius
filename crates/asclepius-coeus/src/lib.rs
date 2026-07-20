//! Coeus reverse-mode differentiation for Asclepius response laws.
//!
//! This infrastructure crate keeps Coeus outside the `no_std` Asclepius law
//! core while providing statically dispatched, backend-generic tape
//! construction for differentiable treatment-planning objectives.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod response;
pub mod value;

pub use value::{AutodiffResponseError, DoseConstraint};
