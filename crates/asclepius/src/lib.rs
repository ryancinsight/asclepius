//! Atlas biological-response and treatment-outcome model foundation.
//!
//! Asclepius owns pure biological-response laws over Aequitas quantities.
//! Consumer packages retain imaging, dose-volume storage, voxel grids,
//! treatment planning, and device execution.

//! The repository README (usage example included) is part of the crate
//! documentation so its `rust` fences run under `cargo test --doc`
//! (ASC-DOC-019): a stale example fails the gate instead of drifting.
#![doc = include_str!("../../../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate alloc;

pub mod contract;
pub mod response;
pub mod tissue;
pub mod value;

pub use contract::BiologicalResponse;
pub use tissue::Tissue;
pub use value::{
    CompensationFactor, DamageIntegral, EquivalentExposure, Gamma50, InvalidValue, LymanSlope,
    Probability, ResponseError, ValueConstraint, ValueKind, VolumeEffect,
};
