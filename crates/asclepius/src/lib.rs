//! Atlas biological-response and treatment-outcome model foundation.
//!
//! Asclepius owns pure biological-response laws over Aequitas quantities.
//! Consumer packages retain imaging, dose-volume storage, voxel grids,
//! treatment planning, and device execution.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../../../README.md")]

extern crate alloc;

pub mod contract;
pub mod response;
pub mod tissue;
pub mod value;

pub use contract::BiologicalResponse;
pub use tissue::Tissue;
pub use value::{
    CompensationFactor, DamageIntegral, EquivalentExposure, InvalidValue, Probability,
    ResponseError, ResponseSlope, ValueConstraint, ValueKind, VolumeEffect,
};
