//! Evaluate one radiation response and one thermal response end to end.

use aequitas::systems::si::{
    quantities::{AbsorbedDose, ThermodynamicTemperature, Time},
    units::{Gray, Kelvin, Second},
};
use asclepius::{
    BiologicalResponse, ResponseSlope, VolumeEffect,
    response::{
        radiation::{GeneralizedEquivalentUniformDose, LogisticControlProbability},
        thermal::{Cem43, TemperatureHistory},
    },
};

fn main() {
    let doses = [40.0_f64, 50.0, 60.0].map(AbsorbedDose::from_unit::<Gray>);
    let geud = GeneralizedEquivalentUniformDose::new(
        VolumeEffect::new(2.0).expect("invariant: fixture exponent is finite and non-zero"),
    )
    .evaluate(&doses)
    .expect("invariant: fixture doses are finite and non-negative");
    let control = LogisticControlProbability::new(
        AbsorbedDose::from_unit::<Gray>(50.0),
        ResponseSlope::new(2.0).expect("invariant: fixture slope is finite and positive"),
    )
    .expect("invariant: fixture midpoint is finite and positive")
    .evaluate(geud)
    .expect("invariant: evaluated gEUD is finite and non-negative");

    let temperatures = [316.15_f64, 317.15].map(ThermodynamicTemperature::from_unit::<Kelvin>);
    let history = TemperatureHistory::new(&temperatures, Time::from_unit::<Second>(60.0))
        .expect("invariant: fixture time step is finite and positive");
    let exposure = Cem43::canonical()
        .evaluate(history)
        .expect("invariant: fixture temperatures are finite and positive");

    println!(
        "gEUD={:.3} Gy, TCP={:.5}, CEM43={:.3} min",
        geud.in_unit::<Gray>(),
        control.get(),
        exposure.get().in_unit::<Second>() / 60.0
    );
}
