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

    // Off-midpoint reference values (ASC-VER-010). The midpoint assertions
    // above hold for any positive parameters; these pin the Niemierko `4`
    // factor and the Lyman `TD50` normalization instead. Reference values are
    // independently evaluated (`CPython` `math`): with `D50 = TD50 = 50`,
    // `gamma50 = 2`, and `m = 0.2`,
    // `TCP(60) = 1/(1 + (50/60)^8) = 0.8113...`,
    // `TCP(40) = 1/(1 + (50/40)^8) = 0.1436...`,
    // `NTCP(60) = Phi((60 - 50)/(0.2 * 50)) = Phi(1) = 0.8413...`, and
    // `NTCP(40) = Phi(-1) = 0.1586...`.
    // A halved Niemierko factor gives 0.6746/0.2906 and a dropped `TD50`
    // normalization gives 1.0/0.0, all far outside the bound below, so the
    // assertions fail under either perturbation.
    // Bound derivation: one division plus one `powf` (TCP) or one `erfc`
    // (NTCP) each contribute single-digit ulps; 64 ulps covers the chain in
    // both supported precisions with margin.
    let steep = LogisticControlProbability::new(
        midpoint,
        Gamma50::new(T::from_f64(2.0)).expect("positive gamma50"),
    )
    .expect("positive midpoint");
    assert_close(
        steep.evaluate(dose::<T>(60.0)).expect("valid dose").get(),
        0.811_314_238_293_995_7,
        64.0,
    );
    assert_close(
        steep.evaluate(dose::<T>(40.0)).expect("valid dose").get(),
        0.143_668_573_157_284_37,
        64.0,
    );
    assert_close(
        ntcp.evaluate(dose::<T>(60.0)).expect("valid dose").get(),
        0.841_344_746_068_542_8,
        64.0,
    );
    assert_close(
        ntcp.evaluate(dose::<T>(40.0)).expect("valid dose").get(),
        0.158_655_253_931_457_13,
        64.0,
    );
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

/// gEUD against a precision-independent oracle (ASC-VER-013).
///
/// The `geud_matches_independent_legacy_formula` case above evaluates the
/// oracle in binary64 with the same `powf` chain as the implementation, so a
/// shared libm bias would cancel. This case instead pins the `[2, 4, 8, 16]`
/// sample at `a = 2.5` against a 50-digit decimal evaluation
/// (`decimal.Decimal` with `prec = 50`):
/// `((2^2.5 + 4^2.5 + 8^2.5 + 16^2.5)/4)^(1/2.5)`
/// `= 9.92931284656492885177...`, which rounds to
/// `9.929312846564929` in binary64. The implementation's `powf` chain carries
/// tens of ulps at most; 64 ulps covers it with margin.
#[test]
fn geud_matches_precision_independent_oracle() {
    let doses = [2.0_f64, 4.0, 8.0, 16.0];
    let exponent = 2.5_f64;
    let typed = doses.map(AbsorbedDose::from_base);
    let model =
        GeneralizedEquivalentUniformDose::new(VolumeEffect::new(exponent).expect("non-zero"));
    let actual = model.evaluate(&typed).expect("valid sample").into_base();
    // Binary64 rounding of 9.92931284656492885177... (trailing digits above are
    // not representable; the full-precision source is pinned in the doc
    // comment above).
    let oracle = 9.929_312_846_564_93_f64;
    let bound = 64.0 * f64::EPSILON * oracle;
    assert!(
        (actual - oracle).abs() <= bound,
        "actual={actual}, oracle={oracle}, bound={bound}"
    );
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
