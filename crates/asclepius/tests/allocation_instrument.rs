//! Allocation-instrumentation windows.
//!
//! `stats_alloc` instruments the process-global allocator, so a `Region`
//! window counts every allocation on every thread of the test process.
//! Measured inside a parallel libtest harness, each window picks up other
//! tests' bookkeeping allocations and the zero-allocation assertion flakes
//! non-deterministically. The invariant is per-process: this binary hosts
//! exactly one test function that runs both measurement windows
//! sequentially, so nothing else in the process can allocate into them.
//! (The nextest CI gate isolates every test into its own process anyway;
//! this binary keeps plain `cargo test` honest too.)

use aequitas::systems::si::{
    quantities::{AbsorbedDose, ThermodynamicTemperature, Time},
    units::Second,
};
use asclepius::{
    Gamma50, Tissue,
    response::{
        radiation::LogisticControlProbability, thermal::Cem43, thermal::TemperatureSamples,
    },
};
use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

#[test]
fn allocation_measurement_windows_are_free_of_foreign_allocations() {
    borrowed_tissue_window();
    lazy_temperature_window();
}

/// Borrowed tissue evaluation at the logistic midpoint: value-exact, name
/// borrowed in place, and no allocation, reallocation, or deallocation.
fn borrowed_tissue_window() {
    let model = LogisticControlProbability::new(
        AbsorbedDose::from_base(50.0_f64),
        Gamma50::new(0.2).expect("positive gamma50"),
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
    assert_eq!(change.allocations, 0, "borrowed tissue window allocations");
    assert_eq!(
        change.reallocations, 0,
        "borrowed tissue window reallocations"
    );
    assert_eq!(
        change.deallocations, 0,
        "borrowed tissue window deallocations"
    );
}

/// Lazily consumed Celsius temperature stream: single-step conversion and
/// the canonical CEM43 integral without allocating inside the window.
fn lazy_temperature_window() {
    const KELVIN_OFFSET: f64 = 273.15;
    let celsius = [42.0_f64, 43.0, 44.0];
    let region = Region::new(ALLOCATOR);

    let observation = TemperatureSamples::new(
        celsius
            .iter()
            .copied()
            .map(|value| ThermodynamicTemperature::from_base(value + KELVIN_OFFSET)),
        Time::from_unit::<Second>(60.0),
    )
    .expect("valid stream");
    let exposure = Cem43::canonical()
        .evaluate_uniform(observation)
        .expect("valid temperature stream");
    let change = region.change();

    assert_eq!(exposure.get().into_base().to_bits(), 195.0_f64.to_bits());
    assert_eq!(change.allocations, 0, "temperature window allocations");
    assert_eq!(change.reallocations, 0, "temperature window reallocations");
    assert_eq!(change.deallocations, 0, "temperature window deallocations");
}
