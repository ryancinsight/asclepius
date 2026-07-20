//! Thermal biological-response laws.

mod arrhenius;
mod cem;
mod history;

pub use arrhenius::ArrheniusDamage;
pub use cem::Cem43;
pub use history::TemperatureHistory;
