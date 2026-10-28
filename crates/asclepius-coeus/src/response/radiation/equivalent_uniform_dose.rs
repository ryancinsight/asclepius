use asclepius::VolumeEffect;
use coeus_autograd::{Var, div, mean, mul, pow};
use coeus_core::{CpuAddressableStorage, CpuAddressableStorageMut, Float, Scalar};
use coeus_ops::BackendOps;
use coeus_tensor::Tensor;
use core::fmt;
use eunomia::{NumericElement, RealField};
use leto_ops::Scalar as LetoScalar;

use crate::{AutodiffResponseError, DoseConstraint};

/// Build the stabilized generalized equivalent uniform dose law on a Coeus tape.
///
/// The tape scalar `T` varies with the caller's backend precision: `f64`
/// planning tapes and `f32` tapes share this one construction. The returned
/// variable represents
///
/// `s * mean((D / s)^a)^(1/a)`,
///
/// where `s` is the maximum dose for positive `a` and the minimum dose for
/// negative `a`. This is algebraically equal to the Asclepius power mean while
/// reducing avoidable intermediate overflow. `s` is a detached scalar:
/// differentiating while holding it constant is exact because the expression
/// is independent of every positive choice of `s`.
///
/// With `P = mean((D/s)^a)`, the response is `s P^(1/a)`. Differentiating at
/// fixed `s` gives `dR/dD_j = P^(1/a - 1) D_j^(a-1)/N`, which the tape's
/// reverse-mode gradient reproduces; the adapter tests assert both the value
/// and this gradient on each backend.
///
/// The operation borrows the tensor and its CPU-addressable storage. It adds
/// only Coeus graph nodes and a scalar constant; it does not copy the dose
/// observation.
///
/// # Reference
///
/// The differentiated law is Niemierko, "Reporting and analyzing dose
/// distributions: A concept of equivalent uniform dose," *Medical Physics* 24,
/// 103-110 ([DOI 10.1118/1.598063](https://doi.org/10.1118/1.598063)).
///
/// # Errors
///
/// Returns [`AutodiffResponseError::EmptyObservation`] for an empty tensor,
/// [`AutodiffResponseError::InvalidDose`] for non-finite or negative doses, or
/// for a zero dose with a negative volume-effect exponent, and
/// [`AutodiffResponseError::Backend`] when the Coeus backend fails while
/// building the graph.
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
///     Tensor::from_slice([3], &[40.0, 50.0, 60.0])
///         .expect("invariant: example shape matches data"),
///     true,
/// )
/// .expect("invariant: example gradient buffer allocation succeeds");
/// let exponent = VolumeEffect::new(1.0).expect("finite non-zero exponent");
/// let geud = generalized_equivalent_uniform_dose(&doses, exponent)
///     .expect("valid absorbed doses");
///
/// assert_eq!(geud.tensor.as_slice(), &[50.0]);
/// ```
#[must_use = "the returned variable carries the differentiable response"]
pub fn generalized_equivalent_uniform_dose<T, B>(
    doses: &Var<T, B>,
    volume_effect: VolumeEffect<T>,
) -> Result<Var<T, B>, AutodiffResponseError<T, B::Error>>
where
    T: Scalar + Float + RealField + LetoScalar + Copy + fmt::Display,
    B: BackendOps<T> + Default,
    B::DeviceBuffer<T>: CpuAddressableStorage<T> + CpuAddressableStorageMut<T>,
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
            scale.max_scalar(dose)
        } else {
            scale.min_scalar(dose)
        };
    }

    // A positive exponent admits an all-zero observation. Unit scaling avoids
    // division by zero while preserving the exact zero result.
    if scale == <T as NumericElement>::ZERO {
        scale = <T as NumericElement>::ONE;
    }

    let backend = B::default();
    let scale = Var::new(Tensor::from_slice_on([1], &[scale], &backend)?, false)?;
    let normalized = div(doses, &scale)?;
    let powered = pow(&normalized, exponent.to_f64())?;
    let mean_power = mean(&powered)?;
    Ok(mul(&scale, &pow(&mean_power, exponent.recip().to_f64())?)?)
}

fn validate_dose<T, E>(
    index: usize,
    dose: T,
    exponent: T,
) -> Result<(), AutodiffResponseError<T, E>>
where
    T: RealField + Copy,
{
    let constraint = if !dose.is_finite() {
        Some(DoseConstraint::Finite)
    } else if dose < <T as NumericElement>::ZERO {
        Some(DoseConstraint::NonNegative)
    } else if exponent.is_sign_negative() && dose == <T as NumericElement>::ZERO {
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
