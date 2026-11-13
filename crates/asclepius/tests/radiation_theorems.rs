//! Executable theorems for radiation response.

use aequitas::systems::si::quantities::AbsorbedDose;
use asclepius::{
    BiologicalResponse, Gamma50, LymanSlope, VolumeEffect,
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
    let gamma50 = Gamma50::new(T::from_f64(0.2)).expect("positive gamma50");
    let m = LymanSlope::new(T::from_f64(0.2)).expect("positive Lyman slope");
    let tcp = LogisticControlProbability::new(midpoint, gamma50).expect("positive midpoint");
    let ntcp = LymanComplicationProbability::new(midpoint, m).expect("positive midpoint");
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

/// Fixed relative-error budget, in units of `T::EPSILON`, of the logistic
/// control law evaluated at integer `k = 4 gamma50`.
///
/// Terms: `powf` at most one unit; `1 + r` and the reciprocal half a unit
/// each; rounding of the expected value to `T` half a unit; the `f64` oracle
/// (exact `powi`, one addition, one division) one unit. The relative error of
/// `r = (D50/D)^k` is amplified by `r/(1+r) < 1`, and the division `D50/D`
/// contributes `k/2` units after exponentiation. The budget is `k/2 + 4`.
const CONTROL_FIXED_UNITS: f64 = 4.0;

/// Niemierko control probability for integer `k = 4 gamma50`, written as
/// `D^k / (D^k + D50^k)`, the rearrangement of `1 / (1 + (D50/D)^k)` that
/// involves no division before the final one. `powi` is exact here because
/// every `D^k` is an integer below `2^53`.
fn niemierko_control(midpoint: f64, dose: f64, exponent: i32) -> f64 {
    let dose_power = dose.powi(exponent);
    dose_power / (dose_power + midpoint.powi(exponent))
}

fn assert_control_probability_off_midpoint<T: RealField>() {
    let midpoint = 50.0;
    // (gamma50, 4 gamma50, dose): distinct exponents 1, 2, 8 separate the
    // factor 4 from any other multiple of gamma50.
    let cases = [
        (0.25, 1, 40.0),
        (0.5, 2, 40.0),
        (0.5, 2, 60.0),
        (2.0, 8, 40.0),
        (2.0, 8, 60.0),
    ];
    for (gamma50, exponent, dose_value) in cases {
        let law = LogisticControlProbability::new(
            dose::<T>(midpoint),
            Gamma50::new(T::from_f64(gamma50)).expect("positive gamma50"),
        )
        .expect("positive midpoint");
        let actual = law
            .evaluate(dose::<T>(dose_value))
            .expect("valid dose")
            .get();
        let units = f64::from(exponent) / 2.0 + CONTROL_FIXED_UNITS;
        assert_close(
            actual,
            niemierko_control(midpoint, dose_value, exponent),
            units,
        );
    }
}

/// Relative-error budget, in units of `T::EPSILON`, of the Lyman law at
/// `t = (D - TD50)/(m TD50)` with `|t| <= 2`.
///
/// The argument `x = -t/sqrt(2)` carries two roundings (the constant and the
/// product), one unit. `erfc` amplifies a relative argument error by
/// `|x| (2/sqrt(pi)) exp(-x^2) / erfc(x)`, which is `1.52` at `t = -1` and
/// smaller at `t = 1` and `t = 2`. The `libm` `erfc` is budgeted at two units
/// and the rounded expected value at half a unit: `1.52 + 2 + 0.5`, rounded
/// up to five. `t` is exact because `D - TD50`, `m TD50`, and their quotient
/// are representable in every supported scalar for the fixtures below.
const NORMAL_CDF_UNITS: f64 = 5.0;

fn assert_complication_probability_off_midpoint<T: RealField>() {
    let midpoint = 40.0;
    let m = LymanSlope::new(T::from_f64(0.25)).expect("positive Lyman slope");
    let law = LymanComplicationProbability::new(dose::<T>(midpoint), m).expect("positive midpoint");
    // m TD50 = 10 Gy, so t = (D - 40)/10. Expected values are the standard
    // normal CDF at t = 1, -1, 2, evaluated from its Maclaurin series
    // `1/2 + (1/sqrt(2 pi)) sum_n (-1)^n t^(2n+1) / (2^n n! (2n+1))` in
    // 40-digit decimal arithmetic, independent of `erfc`.
    let cases = [
        (50.0, 0.841_344_746_068_542_9),
        (30.0, 0.158_655_253_931_457_05),
        (60.0, 0.977_249_868_051_820_8),
    ];
    for (dose_value, expected) in cases {
        let actual = law
            .evaluate(dose::<T>(dose_value))
            .expect("valid dose")
            .get();
        assert_close(actual, expected, NORMAL_CDF_UNITS);
    }
}

#[test]
fn control_and_complication_match_closed_forms_off_midpoint() {
    assert_control_probability_off_midpoint::<f32>();
    assert_control_probability_off_midpoint::<f64>();
    assert_complication_probability_off_midpoint::<f32>();
    assert_complication_probability_off_midpoint::<f64>();
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
