//! Composition, ownership, representation, and allocation invariants.

use core::mem::{align_of, size_of};

use aequitas::systems::si::quantities::{AbsorbedDose, Time};
use asclepius::{
    BiologicalResponse, DamageIntegral, EquivalentExposure, Probability, ResponseSlope, Tissue,
    response::composition::IndependentInsults, response::radiation::LogisticControlProbability,
};
use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

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

#[test]
fn borrowed_tissue_evaluation_is_allocation_free() {
    let model = LogisticControlProbability::new(
        AbsorbedDose::from_base(50.0_f64),
        ResponseSlope::new(0.2).expect("positive slope"),
    )
    .expect("positive midpoint");

    let region = Region::new(ALLOCATOR);
    let tissue = Tissue::borrowed("reference tissue", model);
    let response = tissue
        .evaluate(AbsorbedDose::from_base(50.0))
        .expect("valid midpoint");
    let change = region.change();

    assert_eq!(response.get().to_bits(), 0.5_f64.to_bits());
    assert_eq!(tissue.name().as_ptr(), "reference tissue".as_ptr());
    assert_eq!(change.allocations, 0);
    assert_eq!(change.reallocations, 0);
    assert_eq!(change.deallocations, 0);
}
