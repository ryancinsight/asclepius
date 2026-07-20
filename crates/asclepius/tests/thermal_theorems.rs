//! Executable theorems for thermal response.

use aequitas::systems::si::{
    quantities::{MolarEnergy, MolarHeatCapacity, ReciprocalTime, ThermodynamicTemperature, Time},
    units::{JoulePerMole, JoulePerMoleKelvin, Kelvin, PerSecond, Second},
};
use asclepius::{
    BiologicalResponse, DamageIntegral, EquivalentExposure,
    response::thermal::{ArrheniusDamage, Cem43, TemperatureHistory},
};

fn kelvin(value: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::from_unit::<Kelvin>(value)
}

#[test]
fn canonical_cem_reference_cases_are_exact() {
    let law = Cem43::<f64>::canonical();
    for (temperature, expected_seconds) in [(316.15, 60.0), (317.15, 120.0), (315.15, 15.0)] {
        let samples = [kelvin(temperature)];
        let history =
            TemperatureHistory::new(&samples, Time::from_unit::<Second>(60.0)).expect("valid step");
        let exposure = law.evaluate(history).expect("valid temperature");
        let rounding = 16.0 * f64::EPSILON * expected_seconds;
        assert!((exposure.get().into_base() - expected_seconds).abs() <= rounding);
    }
}

#[test]
fn cumulative_cem_and_arrhenius_outputs_are_monotone() {
    let samples = [kelvin(316.15), kelvin(317.15), kelvin(315.15)];
    let history =
        TemperatureHistory::new(&samples, Time::from_unit::<Second>(60.0)).expect("valid step");
    let mut exposure = [EquivalentExposure::zero(); 3];
    Cem43::canonical()
        .cumulative_into(history, &mut exposure)
        .expect("valid history");
    assert!(exposure.windows(2).all(|window| window[0] <= window[1]));

    let arrhenius = ArrheniusDamage::new(
        ReciprocalTime::from_unit::<PerSecond>(2.0),
        MolarEnergy::from_unit::<JoulePerMole>(1.0),
        MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(1.0),
    )
    .expect("positive parameters");
    let kinetic_samples = [kelvin(1.0), kelvin(1.0)];
    let kinetic_history = TemperatureHistory::new(&kinetic_samples, Time::from_unit::<Second>(0.5))
        .expect("valid step");
    let mut damage = [DamageIntegral::zero(); 2];
    arrhenius
        .cumulative_into(kinetic_history, &mut damage)
        .expect("valid history");
    assert!(damage[0] > DamageIntegral::zero());
    assert!(damage[1] > damage[0]);

    let expected = 2.0 / core::f64::consts::E;
    let rounding = 16.0 * f64::EPSILON * expected;
    assert!((damage[1].get() - expected).abs() <= rounding);
}

#[test]
fn survival_and_kill_are_complements() {
    let damage = DamageIntegral::new(1.0_f64).expect("non-negative");
    let survival = ArrheniusDamage::survival(damage).get();
    let kill = ArrheniusDamage::kill_probability(damage).get();
    let bound = 4.0 * f64::EPSILON;
    assert!((survival + kill - 1.0).abs() <= bound);
    assert!((kill - (1.0 - (-1.0_f64).exp())).abs() <= bound);
}

#[test]
fn thermal_laws_reject_invalid_boundaries() {
    assert!(TemperatureHistory::new(&[], Time::from_base(0.0_f64)).is_err());
    assert!(
        ArrheniusDamage::new(
            ReciprocalTime::from_base(0.0_f64),
            MolarEnergy::from_base(1.0),
            MolarHeatCapacity::from_base(1.0),
        )
        .is_err()
    );

    let invalid = [ThermodynamicTemperature::from_base(f64::NAN)];
    let history = TemperatureHistory::new(&invalid, Time::from_base(1.0)).expect("valid step");
    assert!(Cem43::canonical().evaluate(history).is_err());
}
