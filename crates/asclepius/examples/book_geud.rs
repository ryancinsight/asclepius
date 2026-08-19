//! Generalized equivalent uniform dose (gEUD) for a simple dose distribution.
//!
//! gEUD = ((1/N) Σ Dᵢᵃ)^(1/a) is the power-mean dose with volume-effect
//! exponent `a`.  For `a = 1` (parallel tissue) gEUD equals the mean dose;
//! for large positive `a` (serial tissue) gEUD approaches the maximum dose.
//! For uniform dose distributions gEUD equals the dose regardless of `a`.

extern crate aequitas;
extern crate asclepius;

use aequitas::systems::si::quantities::AbsorbedDose;
use asclepius::response::radiation::GeneralizedEquivalentUniformDose;
use asclepius::{BiologicalResponse, VolumeEffect};

fn main() {
    // ── Uniform dose: gEUD equals the dose for any exponent ──
    let uniform_dose = AbsorbedDose::from_base(2.0_f64); // 2 Gy
    let doses_uniform = [uniform_dose; 5];

    for &a in &[1.0_f64, 5.0, -5.0, 0.1] {
        let vol_effect = VolumeEffect::new(a).expect("valid exponent");
        let geud = GeneralizedEquivalentUniformDose::new(vol_effect);
        let result = geud
            .evaluate(doses_uniform.as_slice())
            .expect("valid doses");
        let geud_gy = *result.as_base();
        assert!(
            (geud_gy - 2.0).abs() < 1e-10,
            "gEUD of uniform 2 Gy must be 2 Gy for a={a}, got {geud_gy}"
        );
    }
    println!("uniform dose: gEUD = 2.0 Gy for all exponents ✓");

    // ── Non-uniform dose: a = 1 gives the mean ──
    let doses_mixed: Vec<AbsorbedDose<f64>> = [1.0_f64, 2.0, 3.0, 4.0]
        .iter()
        .copied()
        .map(AbsorbedDose::from_base)
        .collect();
    let a1 = VolumeEffect::new(1.0_f64).expect("a=1");
    let geud_mean = GeneralizedEquivalentUniformDose::new(a1)
        .evaluate(doses_mixed.as_slice())
        .expect("valid");
    let geud_mean_gy = *geud_mean.as_base();
    println!("mixed doses [1,2,3,4] Gy: gEUD(a=1) = {geud_mean_gy:.4} Gy (mean = 2.5)");
    assert!(
        (geud_mean_gy - 2.5).abs() < 1e-10,
        "gEUD(a=1) must equal mean dose"
    );

    // For large positive a, gEUD approaches max(D) = 4 Gy.
    let a_large = VolumeEffect::new(20.0_f64).expect("large a");
    let geud_serial = GeneralizedEquivalentUniformDose::new(a_large)
        .evaluate(doses_mixed.as_slice())
        .expect("valid");
    let geud_serial_gy = *geud_serial.as_base();
    println!("mixed doses [1,2,3,4] Gy: gEUD(a=20) = {geud_serial_gy:.4} Gy (→ 4.0)");
    assert!(
        geud_serial_gy >= geud_mean_gy,
        "gEUD(large a) must exceed gEUD(a=1)"
    );

    println!("all gEUD assertions passed");
}
