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
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use mnemosyne::counting::{AllocationDelta, CountingAllocator, measure};
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

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

/// Allocations made by another thread inside this thread's window are not
/// counted. This is the oracle that the per-thread instrument can replace
/// the former one-test-per-process workaround: a process-global counter
/// (`stats_alloc`) would absorb the worker's allocation and fail.
#[test]
fn allocations_on_another_thread_are_not_counted() {
    let subscriber = tracing_subscriber::fmt()
        .with_test_writer()
        .with_span_events(FmtSpan::FULL)
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        let (measure_started, worker_finished) =
            info_span!("arrange").in_scope(|| (AtomicBool::new(false), AtomicBool::new(false)));

        // Spawning and joining can allocate on the parent thread. Only the
        // acquire/release handshake runs inside its measurement window.
        thread::scope(|scope| {
            let worker = info_span!("arrange").in_scope(|| {
                scope.spawn(|| {
                    while !measure_started.load(Ordering::Acquire) {
                        thread::yield_now();
                    }
                    let ((), own) = measure(|| drop(black_box(Vec::<u8>::with_capacity(1024))));
                    worker_finished.store(true, Ordering::Release);
                    own
                })
            });

            let (observed, worker_delta) = info_span!("act").in_scope(|| {
                let ((), observed) = measure(|| {
                    measure_started.store(true, Ordering::Release);
                    while !worker_finished.load(Ordering::Acquire) {
                        thread::yield_now();
                    }
                });
                let worker_delta = worker.join().expect("the worker does not panic");
                (observed, worker_delta)
            });

            info_span!("assert").in_scope(|| {
                assert_eq!(observed, AllocationDelta::default(), "foreign thread zero");
                assert_eq!(
                    worker_delta,
                    AllocationDelta {
                        allocations: 1,
                        deallocations: 1,
                        bytes_allocated: 1024,
                        bytes_deallocated: 1024,
                        ..AllocationDelta::default()
                    }
                );
            });
        });
    });
}
