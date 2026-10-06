//! Allocation-instrumentation windows.
//!
//! The measurement window counts allocations made by the calling thread
//! only. A process-wide counter is invalid here: libtest runs the test
//! body on a spawned thread while its main thread keeps inserting the
//! running test into its bookkeeping collections, and parallel tests
//! allocate concurrently, so a process-wide window occasionally absorbs
//! allocations unrelated to the response law under measurement. The
//! per-thread counter keeps both windows exact under the threaded
//! `cargo test` harness as well as under `cargo nextest run`, which
//! isolates every test into its own process anyway.

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
use mnemosyne::counting::{AllocationDelta, CountingAllocator, measure};

#[global_allocator]
static ALLOCATOR: CountingAllocator<std::alloc::System> =
    CountingAllocator::new(std::alloc::System);

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
    let name = "reference tissue";

    let ((tissue, response), delta) = measure(|| {
        let tissue = Tissue::borrowed(name, model);
        let response = tissue
            .evaluate(AbsorbedDose::from_base(50.0))
            .expect("valid midpoint");
        (tissue, response)
    });

    assert_eq!(response.get().to_bits(), 0.5_f64.to_bits());
    assert_eq!(tissue.name().as_ptr(), name.as_ptr());
    assert_eq!(delta, AllocationDelta::default(), "borrowed tissue window");
}

/// Lazily consumed Celsius temperature stream: single-step conversion and
/// the canonical CEM43 integral without allocating inside the window.
fn lazy_temperature_window() {
    const KELVIN_OFFSET: f64 = 273.15;
    let celsius = [42.0_f64, 43.0, 44.0];

    let (exposure, delta) = measure(|| {
        let observation = TemperatureSamples::new(
            celsius
                .iter()
                .copied()
                .map(|value| ThermodynamicTemperature::from_base(value + KELVIN_OFFSET)),
            Time::from_unit::<Second>(60.0),
        )
        .expect("valid stream");
        Cem43::canonical()
            .evaluate_uniform(observation)
            .expect("valid temperature stream")
    });

    assert_eq!(exposure.get().into_base().to_bits(), 195.0_f64.to_bits());
    assert_eq!(delta, AllocationDelta::default(), "temperature window");
}
