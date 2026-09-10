//! Composition, ownership, and representation invariants.

use core::mem::{align_of, size_of};

use aequitas::systems::si::quantities::Time;
use asclepius::{
    BiologicalResponse, DamageIntegral, EquivalentExposure, Probability,
    response::composition::IndependentInsults,
};

#[test]
fn independent_insults_match_survival_product() {
    let mechanisms = [
        Probability::new(0.2_f64).expect("valid probability"),
        Probability::new(0.3_f64).expect("valid probability"),
    ];
    let combined = IndependentInsults::<2>
        .evaluate(&mechanisms)
        .expect("validated inputs");
    // Three subtractions and one multiplication accumulate at most four
    // first-order roundings; decimal 0.2/0.3/0.44 are not binary-exact.
    assert!((combined.get() - 0.44).abs() <= 4.0 * f64::EPSILON);

    let none: [Probability<f64>; 0] = [];
    assert_eq!(
        IndependentInsults::<0>
            .evaluate(&none)
            .expect("empty product is one")
            .get()
            .to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn wrappers_are_transparent_and_strategies_are_zero_sized() {
    assert_eq!(size_of::<Probability<f64>>(), size_of::<f64>());
    assert_eq!(align_of::<Probability<f64>>(), align_of::<f64>());
    assert_eq!(size_of::<DamageIntegral<f32>>(), size_of::<f32>());
    assert_eq!(size_of::<EquivalentExposure<f64>>(), size_of::<Time<f64>>());
    assert_eq!(size_of::<IndependentInsults<2>>(), 0);
    assert_eq!(size_of::<IndependentInsults<8>>(), 0);
}

// The allocation-measurement windows (`borrowed_tissue_evaluation_is_allocation_free`,
// `lazy_temperature_conversion_is_allocation_free`) live in the dedicated
// `allocation_instrument` test binary: the stats_alloc instrument is
// process-global, so measuring inside a parallel harness let concurrent
// tests' bookkeeping allocations leak into the windows and flake the
// zero-allocation assertions.
