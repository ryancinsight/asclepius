//! Independent-composition regression laws at representable boundaries.

use asclepius::{BiologicalResponse, Probability, response::composition::IndependentInsults};
use eunomia::{NumericElement, RealField};
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

fn traced(body: impl FnOnce()) {
    let subscriber = tracing_subscriber::fmt()
        .with_test_writer()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::with_default(subscriber, body);
}

fn composed<T: RealField, const N: usize>(values: [T; N]) -> T {
    let inputs = info_span!("arrange").in_scope(|| {
        values.map(|value| Probability::new(value).expect("valid represented probability"))
    });
    info_span!("act").in_scope(|| {
        IndependentInsults::<N>
            .evaluate(&inputs)
            .expect("composition remains a probability")
            .get()
    })
}

fn assert_singletons<T: RealField>(values: impl IntoIterator<Item = T>) {
    for expected in values {
        let actual = composed([expected]);
        info_span!("assert").in_scope(|| assert_eq!(actual, expected));
    }
}

#[test]
fn singleton_identity_preserves_small_and_ordinary_probabilities() {
    traced(|| {
        assert_singletons([
            0.0_f32,
            f32::from_bits(1),
            f32::from_bits(0x007f_ffff),
            f32::MIN_POSITIVE,
            2.0_f32.powi(-30),
            0.3,
            0.5,
            f32::from_bits(1.0_f32.to_bits() - 1),
            1.0,
        ]);
        assert_singletons([
            0.0_f64,
            f64::from_bits(1),
            f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::MIN_POSITIVE,
            2.0_f64.powi(-60),
            0.3,
            0.5,
            f64::from_bits(1.0_f64.to_bits() - 1),
            1.0,
        ]);
    });
}

fn check_small_pair<T: RealField>(small: T) {
    // For the chosen power-of-two and subnormal values, p^2 is less than
    // half an ULP of 2p. Thus correctly rounding 2p - p^2 gives exactly 2p.
    let expected = small + small;
    let actual = composed([small, small]);
    info_span!("assert").in_scope(|| assert_eq!(actual, expected));
}

#[test]
fn two_small_insults_retain_the_combined_probability() {
    traced(|| {
        check_small_pair(f32::from_bits(1));
        check_small_pair(2.0_f32.powi(-30));
        check_small_pair(f64::from_bits(1));
        check_small_pair(2.0_f64.powi(-60));
        check_small_pair(eunomia::F32(f32::from_bits(1)));
        check_small_pair(eunomia::F32(2.0_f32.powi(-30)));
        check_small_pair(eunomia::F64(f64::from_bits(1)));
        check_small_pair(eunomia::F64(2.0_f64.powi(-60)));
    });
}

fn check_identities<T: RealField>(small: T) {
    let zero = <T as NumericElement>::ZERO;
    let one = <T as NumericElement>::ONE;
    let empty = composed::<T, 0>([]);
    let leading_zero = composed([zero, small]);
    let trailing_zero = composed([small, zero]);
    let certain_first = composed([one, small]);
    let certain_last = composed([small, one]);
    info_span!("assert").in_scope(|| {
        assert_eq!(empty, zero);
        assert_eq!(leading_zero, small);
        assert_eq!(trailing_zero, small);
        assert_eq!(certain_first, one);
        assert_eq!(certain_last, one);
    });
}

#[test]
fn empty_zero_and_certain_insults_obey_probability_identities() {
    traced(|| {
        check_identities(f32::from_bits(1));
        check_identities(f64::from_bits(1));
        check_identities(eunomia::F32(f32::from_bits(1)));
        check_identities(eunomia::F64(f64::from_bits(1)));
    });
}

#[test]
fn exact_dyadic_composition_matches_the_product_theorem() {
    traced(|| {
        // 1 - (1 - 1/4)(1 - 1/2)(1 - 3/4) = 29/32, exactly binary.
        let ordinary = composed([0.25_f32, 0.5, 0.75]);
        let precise = composed([0.25_f64, 0.5, 0.75]);
        info_span!("assert").in_scope(|| {
            assert_eq!(ordinary.to_bits(), (29.0_f32 / 32.0).to_bits());
            assert_eq!(precise.to_bits(), (29.0_f64 / 32.0).to_bits());
        });
    });
}
