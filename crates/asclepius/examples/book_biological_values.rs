//! Validated biological-response values.
//!
//! Asclepius validates every quantity at construction time so downstream
//! computations can trust their inputs.  This example exercises the
//! complete validation surface of the most-used value types.

use asclepius::{Gamma50, InvalidValue, Probability, VolumeEffect};

fn main() {
    // ── Probability: closed [0, 1] ──
    let tcp = Probability::<f64>::new(0.75).expect("valid probability");
    println!("TCP = {:.2}", tcp.get());
    assert!((tcp.get() - 0.75).abs() < 1e-10);

    // Boundary values are valid.
    assert!(Probability::<f64>::new(0.0).is_ok());
    assert!(Probability::<f64>::new(1.0).is_ok());

    // Out-of-range and non-finite are rejected.
    assert!(matches!(
        Probability::<f64>::new(-0.1),
        Err(InvalidValue { .. })
    ));
    assert!(matches!(
        Probability::<f64>::new(1.1),
        Err(InvalidValue { .. })
    ));
    assert!(matches!(
        Probability::<f64>::new(f64::NAN),
        Err(InvalidValue { .. })
    ));
    println!("Probability validation: all checks pass");

    // ── VolumeEffect exponent: finite, non-zero ──
    let a_parallel = VolumeEffect::<f64>::new(1.0).expect("parallel model");
    let a_serial = VolumeEffect::<f64>::new(-16.0).expect("serial model");
    let a_poisson = VolumeEffect::<f64>::new(0.01).expect("near-Poisson");
    println!(
        "a(parallel)={}, a(serial)={}, a(Poisson)={}",
        a_parallel.get(),
        a_serial.get(),
        a_poisson.get()
    );

    assert!(
        VolumeEffect::<f64>::new(0.0).is_err(),
        "zero exponent rejected"
    );
    assert!(
        VolumeEffect::<f64>::new(f64::INFINITY).is_err(),
        "infinite exponent rejected"
    );
    println!("VolumeEffect validation: all checks pass");

    // ── Gamma50: finite, positive ──
    let gamma50 = Gamma50::<f64>::new(4.0).expect("valid gamma50");
    println!("γ₅₀ = {}", gamma50.get());
    assert!(
        Gamma50::<f64>::new(-1.0).is_err(),
        "negative gamma50 rejected"
    );
    println!("Gamma50 validation: all checks pass");

    println!("all biological-value assertions passed");
}
