//! Executable theorems for thermal response.

use aequitas::systems::si::{
    quantities::{MolarEnergy, MolarHeatCapacity, ReciprocalTime, ThermodynamicTemperature, Time},
    units::{JoulePerMole, JoulePerMoleKelvin, Kelvin, PerSecond, Second},
};
use asclepius::{
    BiologicalResponse, DamageIntegral, EquivalentExposure,
    response::thermal::{ArrheniusDamage, Cem43, TemperatureHistory, TemperatureSamples},
};

fn kelvin(value: f64) -> ThermodynamicTemperature {
    ThermodynamicTemperature::from_unit::<Kelvin>(value)
}

#[test]
fn canonical_cem_reference_cases_are_exact() {
    let law = Cem43::<f64>::canonical();
    for (temperature, expected_rate, expected_seconds) in [
        (316.15, 1.0, 60.0),
        (317.15, 2.0, 120.0),
        (315.15, 0.25, 15.0),
    ] {
        let actual_rate = law
            .rate(kelvin(temperature))
            .expect("valid temperature")
            .into_base();
        let rate_rounding = 16.0 * f64::EPSILON * expected_rate;
        assert!((actual_rate - expected_rate).abs() <= rate_rounding);

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

/// Henriques-Moritz epidermal-injury kinetics at a physiological temperature
/// (ASC-VER-011).
///
/// The `A = 2, Ea = 1, R = 1` oracle elsewhere in this file exercises the
/// algebra at `T = 1 K`; this case pins published kinetic parameters at
/// hyperthermia temperature instead: `A = 3.1e98 s^-1` with
/// `Ea = 627.6 kJ/mol` (Henriques & Moritz 1947, cutaneous burns) evaluated
/// at `T = 317.15 K` (44 C) with `R = 8.314462618 J/(mol K)`. The reference
/// rate `1.3414998998367949e-05 s^-1` is independently evaluated
/// (`CPython` `math`: `3.1e98 * exp(-627600/(8.314462618 * 317.15))`).
/// A Celsius/Kelvin mix-up at the boundary (`T = 44` instead of `317.15`)
/// underflows to exactly `0.0`, and a molar/gas-constant mix-up is
/// comparably far off, so the bound below fails on either unit slip.
///
/// Bound derivation: the rate's relative error is the absolute error of the
/// exponent `x = Ea/(R T) ~= 238`; the quotient carries a few ulps, so
/// `|dk/k| ~= |dx| ~= 238 * 3 * EPSILON ~= 1.6e-13`. `1e-12` relative covers
/// that with margin.
#[test]
fn arrhenius_matches_published_kinetics_at_hyperthermia() {
    let law = ArrheniusDamage::new(
        ReciprocalTime::from_unit::<PerSecond>(3.1e98),
        MolarEnergy::from_unit::<JoulePerMole>(627_600.0),
        MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(8.314_462_618),
    )
    .expect("positive parameters");
    let actual = law
        .rate(kelvin(317.15))
        .expect("physiological temperature is valid")
        .into_base();
    let expected = 1.341_499_899_836_794_9e-5;
    assert!(
        (actual - expected).abs() <= 1e-12 * expected,
        "actual={actual:e}, expected={expected:e}"
    );
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
fn lazy_temperature_stream_matches_borrowed_history() {
    const KELVIN_OFFSET: f64 = 273.15;
    let celsius = [43.0_f64, 44.0, 42.0];
    let absolute = celsius.map(|value| kelvin(value + KELVIN_OFFSET));
    let step = Time::from_unit::<Second>(60.0);

    let borrowed = TemperatureHistory::new(&absolute, step).expect("valid borrowed history");
    let streamed = TemperatureSamples::new(
        celsius
            .iter()
            .copied()
            .map(|value| kelvin(value + KELVIN_OFFSET)),
        step,
    )
    .expect("valid streamed history");
    let law = Cem43::<f64>::canonical();
    let expected = law.evaluate(borrowed).expect("valid borrowed history");
    let actual = law
        .evaluate_uniform(streamed)
        .expect("valid streamed history");
    assert_eq!(
        actual.get().into_base().to_bits(),
        expected.get().into_base().to_bits()
    );

    let arrhenius = ArrheniusDamage::new(
        ReciprocalTime::from_unit::<PerSecond>(2.0),
        MolarEnergy::from_unit::<JoulePerMole>(1.0),
        MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(1.0),
    )
    .expect("positive parameters");
    let borrowed = TemperatureHistory::new(&absolute, step).expect("valid borrowed history");
    let streamed = TemperatureSamples::new(absolute.into_iter(), step).expect("valid stream");
    assert_eq!(
        arrhenius
            .evaluate_uniform(streamed)
            .expect("valid streamed history")
            .get()
            .to_bits(),
        arrhenius
            .evaluate(borrowed)
            .expect("valid borrowed history")
            .get()
            .to_bits()
    );
}

#[test]
fn single_increment_matches_singleton_history() {
    let temperature = kelvin(316.15);
    let step = Time::from_unit::<Second>(60.0);
    let history = [temperature];
    let cem = Cem43::<f64>::canonical();
    assert_eq!(
        cem.increment(temperature, step)
            .expect("valid increment")
            .get()
            .into_base()
            .to_bits(),
        cem.evaluate(TemperatureHistory::new(&history, step).expect("valid singleton history"))
            .expect("valid singleton history")
            .get()
            .into_base()
            .to_bits()
    );

    let arrhenius = ArrheniusDamage::new(
        ReciprocalTime::from_unit::<PerSecond>(2.0),
        MolarEnergy::from_unit::<JoulePerMole>(1.0),
        MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(1.0),
    )
    .expect("positive parameters");
    assert_eq!(
        arrhenius
            .increment(temperature, step)
            .expect("valid increment")
            .get()
            .to_bits(),
        arrhenius
            .evaluate(TemperatureHistory::new(&history, step).expect("valid singleton history"))
            .expect("valid singleton history")
            .get()
            .to_bits()
    );
}

#[test]
fn thermal_laws_reject_invalid_boundaries() {
    assert!(TemperatureHistory::new(&[], Time::from_base(0.0_f64)).is_err());
    assert!(
        TemperatureSamples::new(
            core::iter::empty::<ThermodynamicTemperature<f64>>(),
            Time::from_base(0.0),
        )
        .is_err()
    );
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
    assert!(
        Cem43::canonical()
            .rate(ThermodynamicTemperature::from_base(f64::NAN))
            .is_err()
    );

    let empty = TemperatureSamples::new(
        core::iter::empty::<ThermodynamicTemperature<f64>>(),
        Time::from_base(1.0),
    )
    .expect("valid step");
    assert_eq!(
        Cem43::canonical().evaluate_uniform(empty),
        Err(asclepius::ResponseError::EmptyObservation)
    );

    let one = TemperatureSamples::new(
        core::iter::once(ThermodynamicTemperature::from_base(316.15_f64)),
        Time::from_base(1.0),
    )
    .expect("valid observation");
    let mut no_output = [];
    assert_eq!(
        Cem43::canonical().cumulative_into(one, &mut no_output),
        Err(asclepius::ResponseError::OutputLength {
            expected: 1,
            actual: 0,
        })
    );
}
