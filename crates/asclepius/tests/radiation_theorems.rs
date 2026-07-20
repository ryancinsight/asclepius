//! Executable theorems for radiation response.

use aequitas::systems::si::quantities::AbsorbedDose;
use asclepius::{
    BiologicalResponse, ResponseSlope, VolumeEffect,
    response::radiation::{
        GeneralizedEquivalentUniformDose, LogisticControlProbability, LymanComplicationProbability,
    },
};
use eunomia::RealField;

fn dose<T>(value: f64) -> AbsorbedDose<T>
where
    T: RealField,
{
    AbsorbedDose::from_base(T::from_f64(value))
}

fn assert_close<T: RealField>(actual: T, expected: f64, units: f64) {
    let expected = T::from_f64(expected);
    let bound = T::from_f64(units) * T::EPSILON * expected.abs().max_scalar(T::from_f64(1.0));
    assert!(
        (actual - expected).abs() <= bound,
        "actual={actual:?}, expected={expected:?}, bound={bound:?}"
    );
}

fn assert_radiation_laws<T: RealField>() {
    let uniform = [dose::<T>(5.0); 4];
    for exponent in [T::from_f64(-4.0), T::from_f64(1.0), T::from_f64(8.0)] {
        let model = GeneralizedEquivalentUniformDose::new(
            VolumeEffect::new(exponent).expect("non-zero exponent"),
        );
        let result = model.evaluate(&uniform).expect("valid dose sample");
        assert_close(*result.as_base(), 5.0, 16.0);
    }

    let sample = [dose::<T>(1.0), dose::<T>(2.0), dose::<T>(10.0)];
    let low = GeneralizedEquivalentUniformDose::new(
        VolumeEffect::new(T::from_f64(-8.0)).expect("negative exponent"),
    )
    .evaluate(&sample)
    .expect("positive sample");
    let high = GeneralizedEquivalentUniformDose::new(
        VolumeEffect::new(T::from_f64(8.0)).expect("positive exponent"),
    )
    .evaluate(&sample)
    .expect("positive sample");
    assert!(*low.as_base() >= T::from_f64(1.0));
    assert!(*high.as_base() <= T::from_f64(10.0));
    assert!(*low.as_base() < *high.as_base());

    let midpoint = dose::<T>(50.0);
    let slope = ResponseSlope::new(T::from_f64(0.2)).expect("positive slope");
    let tcp = LogisticControlProbability::new(midpoint, slope).expect("positive midpoint");
    let ntcp = LymanComplicationProbability::new(midpoint, slope).expect("positive midpoint");
    assert_close(tcp.evaluate(midpoint).expect("valid dose").get(), 0.5, 8.0);
    assert_close(ntcp.evaluate(midpoint).expect("valid dose").get(), 0.5, 8.0);
    assert!(
        tcp.evaluate(dose::<T>(40.0)).expect("valid").get()
            < tcp.evaluate(dose::<T>(60.0)).expect("valid").get()
    );
    assert!(
        ntcp.evaluate(dose::<T>(40.0)).expect("valid").get()
            < ntcp.evaluate(dose::<T>(60.0)).expect("valid").get()
    );
}

#[test]
fn radiation_laws_hold_for_supported_real_scalars() {
    assert_radiation_laws::<f32>();
    assert_radiation_laws::<f64>();
}

#[test]
fn geud_matches_independent_legacy_formula() {
    let doses = [2.0_f64, 4.0, 8.0, 16.0];
    let exponent = 2.5_f64;
    let typed = doses.map(AbsorbedDose::from_base);
    let model =
        GeneralizedEquivalentUniformDose::new(VolumeEffect::new(exponent).expect("non-zero"));
    let actual = model.evaluate(&typed).expect("valid sample").into_base();
    let count = f64::from(u32::try_from(doses.len()).expect("fixture length fits u32"));
    let oracle =
        (doses.iter().map(|dose| dose.powf(exponent)).sum::<f64>() / count).powf(exponent.recip());
    let bound = 16.0 * f64::EPSILON * oracle;
    assert!((actual - oracle).abs() <= bound);
}

#[test]
fn geud_rejects_empty_and_non_positive_negative_power_samples() {
    let positive =
        GeneralizedEquivalentUniformDose::new(VolumeEffect::new(2.0).expect("positive exponent"));
    assert!(positive.evaluate(&[]).is_err());

    let negative =
        GeneralizedEquivalentUniformDose::new(VolumeEffect::new(-2.0).expect("negative exponent"));
    assert!(negative.evaluate(&[AbsorbedDose::from_base(0.0)]).is_err());
}

proptest::proptest! {
    #[test]
    fn positive_power_mean_stays_within_sample_bounds(
        values in proptest::collection::vec(1.0e-6_f64..1.0e4, 1..64),
        exponent in 0.125_f64..12.0,
    ) {
        let doses: Vec<_> = values.iter().copied().map(AbsorbedDose::from_base).collect();
        let model = GeneralizedEquivalentUniformDose::new(
            VolumeEffect::new(exponent).expect("generated exponent is positive"),
        );
        let response = model.evaluate(&doses).expect("generated doses are positive").into_base();
        let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
        let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let rounding = 64.0 * f64::EPSILON * maximum.max(1.0);
        proptest::prop_assert!(response >= minimum - rounding);
        proptest::prop_assert!(response <= maximum + rounding);
    }
}
