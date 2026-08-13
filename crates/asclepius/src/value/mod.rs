//! Validated biological-response values and failures.

mod damage;
mod error;
mod exposure;
mod parameter;
mod probability;
pub(crate) mod validation;

pub use damage::DamageIntegral;
pub use error::{InvalidValue, ResponseError, ValueConstraint, ValueKind};
pub use exposure::EquivalentExposure;
pub use parameter::{CompensationFactor, Gamma50, LymanSlope, VolumeEffect};
pub use probability::Probability;
