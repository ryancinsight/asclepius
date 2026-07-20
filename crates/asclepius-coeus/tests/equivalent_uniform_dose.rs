//! Coeus value, gradient, stabilization, and validation contracts for gEUD.

use aequitas::systems::si::quantities::AbsorbedDose;
use asclepius::{
    BiologicalResponse, VolumeEffect, response::radiation::GeneralizedEquivalentUniformDose,
};
use asclepius_coeus::{
    AutodiffResponseError, response::radiation::generalized_equivalent_uniform_dose,
};
use coeus_autograd::{Var, sum};
use coeus_core::{
    CpuAddressableStorage, CpuAddressableStorageMut, MoiraiBackend, SequentialBackend,
};
use coeus_ops::BackendOps;
use coeus_tensor::Tensor;

fn assert_backend_contract<B>()
where
    B: BackendOps<f64> + Default,
    B::DeviceBuffer<f64>: CpuAddressableStorage<f64> + CpuAddressableStorageMut<f64>,
{
    let doses = [40.0, 50.0, 60.0];
    let exponent = VolumeEffect::new(2.5).expect("fixture exponent is non-zero");
    let variable = Var::<f64, B>::new(Tensor::from_slice([doses.len()], &doses), true);
    let response = generalized_equivalent_uniform_dose(&variable, exponent)
        .expect("fixture doses satisfy the response domain");

    let typed_doses = doses.map(AbsorbedDose::from_base);
    let oracle = GeneralizedEquivalentUniformDose::new(exponent)
        .evaluate(&typed_doses)
        .expect("fixture doses satisfy the core response domain");
    // Power, mean, inverse power, and rescaling contribute fewer than 16
    // rounded operations along any value path; four ulps per transcendental
    // operation gives this conservative first-order bound.
    let tolerance = 64.0 * f64::EPSILON * oracle.as_base().abs();
    assert!((response.tensor.as_slice()[0] - oracle.as_base()).abs() <= tolerance);

    sum(&response).backward();
    let gradient = variable
        .grad()
        .expect("tracked dose variable receives a gradient");
    let mean_power = doses
        .iter()
        .map(|dose| dose.powf(exponent.get()))
        .sum::<f64>()
        / 3.0;
    let expected_scale = mean_power.powf(exponent.get().recip() - 1.0) / 3.0;
    for (&actual, dose) in gradient.as_slice().iter().zip(doses) {
        let expected = expected_scale * dose.powf(exponent.get() - 1.0);
        // The backward path adds one power, two products, and the forward
        // response chain to the value bound above.
        let gradient_tolerance = 96.0 * f64::EPSILON * expected.abs();
        assert!((actual - expected).abs() <= gradient_tolerance);
    }
}

#[test]
fn moirai_backend_matches_core_value_and_analytic_gradient() {
    assert_backend_contract::<MoiraiBackend>();
}

#[test]
fn sequential_backend_matches_core_value_and_analytic_gradient() {
    assert_backend_contract::<SequentialBackend>();
}

#[test]
fn stabilization_preserves_large_finite_doses() {
    let doses = [f64::MAX / 8.0, f64::MAX / 16.0];
    let exponent = VolumeEffect::new(4.0).expect("fixture exponent is non-zero");
    let variable = Var::<f64, MoiraiBackend>::new(Tensor::from_slice([2], &doses), true);
    let response = generalized_equivalent_uniform_dose(&variable, exponent)
        .expect("large fixture doses are finite and non-negative");

    assert!(response.tensor.as_slice()[0].is_finite());
    assert!(response.tensor.as_slice()[0] >= doses[1]);
    assert!(response.tensor.as_slice()[0] <= doses[0]);
}

#[test]
fn invalid_observations_return_typed_errors() {
    let positive = VolumeEffect::new(2.0).expect("fixture exponent is non-zero");
    let negative = VolumeEffect::new(-2.0).expect("fixture exponent is non-zero");

    let empty = Var::<f64, MoiraiBackend>::new(Tensor::from_slice([0], &[]), true);
    assert!(matches!(
        generalized_equivalent_uniform_dose(&empty, positive),
        Err(AutodiffResponseError::EmptyObservation)
    ));

    let invalid =
        Var::<f64, MoiraiBackend>::new(Tensor::from_slice([3], &[1.0, f64::NAN, 3.0]), true);
    assert!(matches!(
        generalized_equivalent_uniform_dose(&invalid, positive),
        Err(AutodiffResponseError::InvalidDose { index: 1, .. })
    ));

    let zero = Var::<f64, MoiraiBackend>::new(Tensor::from_slice([2], &[1.0, 0.0]), true);
    assert!(matches!(
        generalized_equivalent_uniform_dose(&zero, negative),
        Err(AutodiffResponseError::InvalidDose { index: 1, .. })
    ));
}
