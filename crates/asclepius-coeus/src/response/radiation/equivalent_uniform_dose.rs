use asclepius::VolumeEffect;
use coeus_autograd::{Var, div, mean, mul, pow};
use coeus_core::{CpuAddressableStorage, CpuAddressableStorageMut};
use coeus_ops::BackendOps;
use coeus_tensor::Tensor;

use crate::{AutodiffResponseError, DoseConstraint};

/// Build the stabilized generalized equivalent uniform dose law on a Coeus tape.
///
/// The returned variable represents
///
/// `s * mean((D / s)^a)^(1/a)`,
///
/// where `s` is the maximum dose for positive `a` and the minimum dose for
/// negative `a`. This is algebraically equal to the Asclepius power mean while
/// reducing avoidable intermediate overflow. `s` is a detached scalar:
/// differentiating while holding it constant is exact because the expression
/// is independent of every positive choice of `s`.
///
/// The operation borrows the tensor and its CPU-addressable storage. It adds
/// only Coeus graph nodes and a scalar constant; it does not copy the dose
/// observation.
///
/// # Errors
///
/// Returns [`AutodiffResponseError::EmptyObservation`] for an empty tensor,
/// [`AutodiffResponseError::InvalidDose`] for non-finite or negative doses, or
/// for a zero dose with a negative volume-effect exponent.
///
/// # Examples
///
/// ```
/// use asclepius::VolumeEffect;
/// use asclepius_coeus::response::radiation::generalized_equivalent_uniform_dose;
/// use coeus_autograd::Var;
/// use coeus_core::SequentialBackend;
/// use coeus_tensor::Tensor;
///
/// let doses = Var::<f64, SequentialBackend>::new(
///     Tensor::from_slice([3], &[40.0, 50.0, 60.0]),
///     true,
/// );
/// let exponent = VolumeEffect::new(1.0).expect("finite non-zero exponent");
/// let geud = generalized_equivalent_uniform_dose(&doses, exponent)
///     .expect("valid absorbed doses");
///
/// assert_eq!(geud.tensor.as_slice(), &[50.0]);
/// ```
#[must_use = "the returned variable carries the differentiable response"]
pub fn generalized_equivalent_uniform_dose<B>(
    doses: &Var<f64, B>,
    volume_effect: VolumeEffect<f64>,
) -> Result<Var<f64, B>, AutodiffResponseError>
where
    B: BackendOps<f64> + Default,
    B::DeviceBuffer<f64>: CpuAddressableStorage<f64> + CpuAddressableStorageMut<f64>,
{
    let values = doses.tensor.as_slice();
    let (&first, remainder) = values
        .split_first()
        .ok_or(AutodiffResponseError::EmptyObservation)?;
    let exponent = volume_effect.get();

    validate_dose(0, first, exponent)?;
    let mut scale = first;
    for (offset, &dose) in remainder.iter().enumerate() {
        let index = offset + 1;
        validate_dose(index, dose, exponent)?;
        scale = if exponent.is_sign_positive() {
            scale.max(dose)
        } else {
            scale.min(dose)
        };
    }

    // A positive exponent admits an all-zero observation. Unit scaling avoids
    // division by zero while preserving the exact zero result.
    if scale == 0.0 {
        scale = 1.0;
    }

    let backend = B::default();
    let scale = Var::new(Tensor::from_slice_on([1], &[scale], &backend), false);
    let normalized = div(doses, &scale);
    let powered = pow(&normalized, exponent);
    let mean_power = mean(&powered);
    Ok(mul(&scale, &pow(&mean_power, exponent.recip())))
}

fn validate_dose(index: usize, dose: f64, exponent: f64) -> Result<(), AutodiffResponseError> {
    let constraint = if !dose.is_finite() {
        Some(DoseConstraint::Finite)
    } else if dose < 0.0 {
        Some(DoseConstraint::NonNegative)
    } else if exponent.is_sign_negative() && dose == 0.0 {
        Some(DoseConstraint::PositiveForNegativeExponent)
    } else {
        None
    };

    if let Some(constraint) = constraint {
        Err(AutodiffResponseError::InvalidDose {
            index,
            value: dose,
            constraint,
        })
    } else {
        Ok(())
    }
}
