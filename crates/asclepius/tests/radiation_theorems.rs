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

/// Maximum relative error of one correctly rounded IEEE 754 operation in
/// round-to-nearest, in units of `T::EPSILON`: the unit roundoff `u = EPSILON / 2`.
const ROUNDING_UNITS: f64 = 0.5;

/// Roundings common to every control case: the sum `1 + r`, the reciprocal,
/// the rounding of the expected value to `T` in `assert_close`, and the one
/// division of the `f64` oracle (its numerator and denominator are exact).
const CONTROL_COMMON_ROUNDINGS: u32 = 4;

/// Roundings in `r = (D50/D)^k` for the exponents `k` used below.
///
/// `libm` evaluates `powf(x, 1)` as `x` and `powf(x, 2)` as `x * x`
/// (libm 0.2.16 `pow.rs` lines 170 and 175, `powf.rs` lines 129 and 134), so
/// `k = 1` carries the one rounding of the quotient `D50/D`, and `k = 2`
/// carries it twice (it is squared) plus the rounding of the product. No
/// accuracy figure for `libm` enters.
fn power_roundings(exponent: u32) -> u32 {
    match exponent {
        1 => 1,
        2 => 3,
        other => panic!("exponent {other} has no exact libm special case"),
    }
}

/// Relative error budget, in units of `T::EPSILON`, of the logistic control
/// law at integer exponent `k = 4 gamma50` in `{1, 2}`.
///
/// The computed value is `fl(1 / fl(1 + r))` with `r` carrying
/// `power_roundings` roundings. A relative error `theta` in `r` moves
/// `1 + r` by at most `theta r / (1 + r) < theta`, so every rounding in the
/// chain contributes at most `u`, and the budget is the count of roundings
/// times `ROUNDING_UNITS`: `(power_roundings + CONTROL_COMMON_ROUNDINGS) / 2`.
/// Products of at most seven factors `1 + delta` with `|delta| <= u`
/// differ from first order by under `10^-5` units for `f32` and `f64`.
/// `assert_close` scales by `max(|expected|, 1) = 1`, while every expected
/// value below is under `0.7`, so the absolute bound exceeds the relative
/// budget by a factor above `1.4`, which dominates that second-order term.
fn control_units(exponent: u32) -> f64 {
    f64::from(power_roundings(exponent) + CONTROL_COMMON_ROUNDINGS) * ROUNDING_UNITS
}

/// Niemierko control probability for integer `k = 4 gamma50`, written as
/// `D^k / (D^k + D50^k)`, the rearrangement of `1 / (1 + (D50/D)^k)` that
/// involves no division before the final one. The powers are repeated
/// products of integers below `2^53`, hence exact, as is their sum.
fn niemierko_control(midpoint: f64, dose: f64, exponent: u32) -> f64 {
    let power = |base: f64| (0..exponent).fold(1.0, |product, _| product * base);
    let dose_power = power(dose);
    dose_power / (dose_power + power(midpoint))
}

fn assert_control_probability_off_midpoint<T: RealField>() {
    // (D50, doses): two midpoints so a constant-D50 mutant fails.
    let midpoints = [(50.0, [40.0, 60.0]), (80.0, [60.0, 100.0])];
    // (gamma50, 4 gamma50): both products are exact, and the exponent
    // differs per gamma50, so a factor other than 4 changes k and the value.
    let slopes = [(0.25, 1), (0.5, 2)];
    for (midpoint, doses) in midpoints {
        for (gamma50, exponent) in slopes {
            let law = LogisticControlProbability::new(
                dose::<T>(midpoint),
                Gamma50::new(T::from_f64(gamma50)).expect("positive gamma50"),
            )
            .expect("positive midpoint");
            for dose_value in doses {
                let actual = law
                    .evaluate(dose::<T>(dose_value))
                    .expect("valid dose")
                    .get();
                assert_close(
                    actual,
                    niemierko_control(midpoint, dose_value, exponent),
                    control_units(exponent),
                );
            }
        }
    }
}

/// Checks the Lyman law against `Phi(t) = erfc(-t / sqrt(2)) / 2` evaluated
/// by the same `erfc` at an exactly known `t = (D - TD50) / (m TD50)`.
///
/// The identity follows from the definitions. Substituting `s = sqrt(2) u`
/// in the standard normal density gives
/// `Phi(t) = (1/sqrt(pi)) int_{-inf}^{t/sqrt(2)} exp(-u^2) du`. With
/// `int_{-inf}^{inf} exp(-u^2) du = sqrt(pi)` and
/// `erf(x) = (2/sqrt(pi)) int_0^x exp(-u^2) du` (libm 0.2.16 `erf.rs` lines
/// 14-18), `Phi(t) = (1 + erf(t/sqrt(2))) / 2`, which is
/// `erfc(-t/sqrt(2)) / 2` because `erfc = 1 - erf` (same file, line 21).
///
/// Every `D - TD50` and `m TD50` below is a small integer (`m` is a power of
/// two), so the law's quotient `t` is exactly `-1`, `1` or `2`. The law then
/// forms `-t * c` with `c = T::from_f64(FRAC_1_SQRT_2)`, a product with `-1`,
/// `1` or `2` that rounds nothing, so the law and the reference apply `erfc`
/// to bit-identical arguments and the comparison is exact equality: no
/// accuracy figure for `erfc` enters, and `erfc` itself is not asserted here.
fn assert_complication_is_normal_cdf_of_normalized_dose<T: RealField>() {
    // (TD50, m): m TD50 is 10 Gy and 30 Gy, so a dose maps to a different t
    // under each fixture and a constant m or TD50 fails one of them.
    let fixtures = [(40.0, 0.25), (60.0, 0.5)];
    for (midpoint, slope) in fixtures {
        let m = LymanSlope::new(T::from_f64(slope)).expect("positive Lyman slope");
        let law =
            LymanComplicationProbability::new(dose::<T>(midpoint), m).expect("positive midpoint");
        let scale = midpoint * slope;
        for normalized in [-1.0, 1.0, 2.0] {
            let dose_value = midpoint + normalized * scale;
            let actual = law
                .evaluate(dose::<T>(dose_value))
                .expect("valid dose")
                .get();
            let argument = T::from_f64(-normalized) * T::from_f64(core::f64::consts::FRAC_1_SQRT_2);
            let expected = T::from_f64(0.5) * argument.erfc();
            assert_eq!(
                actual, expected,
                "dose {dose_value} Gy at TD50 {midpoint} Gy"
            );
        }
    }
}

#[test]
fn control_and_complication_match_closed_forms_off_midpoint() {
    assert_control_probability_off_midpoint::<f32>();
    assert_control_probability_off_midpoint::<f64>();
    assert_complication_is_normal_cdf_of_normalized_dose::<f32>();
    assert_complication_is_normal_cdf_of_normalized_dose::<f64>();
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
