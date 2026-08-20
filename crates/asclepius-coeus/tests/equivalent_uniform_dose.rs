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

    sum(&response)
        .backward()
        .expect("fixture response supports reverse differentiation");
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

fn evaluate_value<B>(doses: &[f64], exponent: f64) -> f64
where
    B: BackendOps<f64> + Default,
    B::DeviceBuffer<f64>: CpuAddressableStorage<f64> + CpuAddressableStorageMut<f64>,
{
    let exponent = VolumeEffect::new(exponent).expect("fixture exponent is non-zero");
    let variable = Var::<f64, B>::new(Tensor::from_slice([doses.len()], doses), false);
    generalized_equivalent_uniform_dose(&variable, exponent)
        .expect("fixture doses satisfy the response domain")
        .tensor
        .as_slice()[0]
}

fn central_difference_gradient<B>(doses: [f64; 3], index: usize, exponent: f64) -> (f64, f64)
where
    B: BackendOps<f64> + Default,
    B::DeviceBuffer<f64>: CpuAddressableStorage<f64> + CpuAddressableStorageMut<f64>,
{
    let coordinate_scale = doses[index].abs().max(1.0);
    // h = sqrt(epsilon) * scale balances O(h²) central-difference truncation
    // against O(epsilon / h) value-evaluation roundoff.
    let step = f64::EPSILON.sqrt() * coordinate_scale;
    let half_step = step / 2.0;

    let mut plus = doses;
    plus[index] += step;
    let mut minus = doses;
    minus[index] -= step;
    let coarse = (evaluate_value::<B>(&plus, exponent) - evaluate_value::<B>(&minus, exponent))
        / (2.0 * step);

    let mut half_plus = doses;
    half_plus[index] += half_step;
    let mut half_minus = doses;
    half_minus[index] -= half_step;
    let fine = (evaluate_value::<B>(&half_plus, exponent)
        - evaluate_value::<B>(&half_minus, exponent))
        / (2.0 * half_step);

    // Richardson extrapolation cancels the leading O(h²) term. The remaining
    // truncation estimate is |fine - coarse| / 3. Existing adapter tests bound
    // one value path by 64 ulps; the half-step subtraction doubles that bound.
    let extrapolated = fine + (fine - coarse) / 3.0;
    let truncation = (fine - coarse).abs() / 3.0;
    let value_scale = [
        evaluate_value::<B>(&plus, exponent),
        evaluate_value::<B>(&minus, exponent),
        evaluate_value::<B>(&half_plus, exponent),
        evaluate_value::<B>(&half_minus, exponent),
    ]
    .into_iter()
    .map(f64::abs)
    .fold(1.0, f64::max);
    let roundoff = 128.0 * f64::EPSILON * value_scale / step;
    (extrapolated, truncation + roundoff)
}

#[test]
fn moirai_backend_matches_core_value_and_analytic_gradient() {
    assert_backend_contract::<MoiraiBackend>();
}

#[test]
fn sequential_backend_matches_core_value_and_analytic_gradient() {
    assert_backend_contract::<SequentialBackend>();
}

fn assert_central_difference_contract<B>()
where
    B: BackendOps<f64> + Default,
    B::DeviceBuffer<f64>: CpuAddressableStorage<f64> + CpuAddressableStorageMut<f64>,
{
    let doses = [40.0, 50.0, 60.0];
    let exponent = VolumeEffect::new(2.5).expect("fixture exponent is non-zero");
    let variable = Var::<f64, B>::new(Tensor::from_slice([doses.len()], &doses), true);
    let response = generalized_equivalent_uniform_dose(&variable, exponent)
        .expect("fixture doses satisfy the response domain");
    sum(&response)
        .backward()
        .expect("fixture response supports reverse differentiation");
    let gradient = variable
        .grad()
        .expect("tracked dose variable receives a gradient");

    for (index, &actual) in gradient.as_slice().iter().enumerate() {
        let (expected, tolerance) = central_difference_gradient::<B>(doses, index, exponent.get());
        assert!(
            (actual - expected).abs() <= tolerance,
            "gradient coordinate {index}: autodiff={actual}, finite_difference={expected}, bound={tolerance}"
        );
    }
}

#[test]
fn moirai_gradient_matches_independent_central_difference() {
    assert_central_difference_contract::<MoiraiBackend>();
}

#[test]
fn sequential_gradient_matches_independent_central_difference() {
    assert_central_difference_contract::<SequentialBackend>();
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
