# Validated Response Values

Asclepius validates every biological-response quantity at construction time,
so downstream laws can trust their inputs without re-checking domains at each
evaluation. The `value` module defines transparent newtypes over a
`eunomia::RealField` scalar, each with one documented mathematical domain, and
a typed failure surface that names the offending value and the constraint it
violated.

## Probability

`Probability<T>` is a validated value in the closed interval `[0, 1]`. The
bounds are inclusive: a perfectly deterministic outcome (`0.0` or `1.0`) is a
valid probability. Non-finite values and values outside the interval are
rejected with `InvalidValue`:

```rust,ignore
let tcp = Probability::<f64>::new(0.75)?;
assert!(Probability::<f64>::new(0.0).is_ok());
assert!(Probability::<f64>::new(1.0).is_ok());
assert!(Probability::<f64>::new(1.1).is_err());
```

## Volume effect and response parameters

`VolumeEffect<T>` is a finite non-zero generalized-mean exponent. The sign
carries the tissue model: positive exponents describe parallel architectures
where gEUD approaches the mean, while negative exponents describe serial
architectures where gEUD approaches the minimum dose. `Gamma50<T>` and
`LymanSlope<T>` are distinct finite positive parameters for the TCP and NTCP
models. The type distinction prevents passing Niemierko `gamma50` where Lyman
`m` is required. Both reject their non-positive degenerate cases at
construction:

```rust,ignore
let a_parallel = VolumeEffect::<f64>::new(1.0)?;   // parallel (mean-like)
let a_serial = VolumeEffect::<f64>::new(-16.0)?;   // serial (min-like)
let gamma50 = Gamma50::<f64>::new(4.0)?;
let m = LymanSlope::<f64>::new(0.15)?;
assert!(VolumeEffect::<f64>::new(0.0).is_err());
assert!(Gamma50::<f64>::new(-1.0).is_err());
assert!(LymanSlope::<f64>::new(-1.0).is_err());
```

## Thermal quantities

`DamageIntegral<T>` is the validated non-negative dimensionless Arrhenius
damage integral; `EquivalentExposure<T>` is the validated non-negative
cumulative equivalent exposure, transparent over an Aequitas `Time` quantity
so the time dimension survives while the newtype separates biological
equivalent exposure from wall time. `CompensationFactor<T>` is the CEM
temperature-compensation factor in the half-open interval `(0, 1]`.

## Failure surface

Rejections are reported as `InvalidValue<T>` carrying the `ValueKind` (which
biological value role was checked), the rejected scalar, and the
`ValueConstraint` that was violated (`FiniteNonNegative`, `FinitePositive`,
`FiniteNonZero`, `UnitInterval`, or `PositiveUnitInterval`). Law evaluations
lift that into `ResponseError<T>`, which also covers sample-dependent failures
(`InvalidObservation` with a zero-based index), an empty observation, an
over-long observation, a mismatched caller output buffer, and a non-finite
result from otherwise valid inputs.

The runnable
[biological-values example](examples/biological_values.md) exercises this
complete validation surface: boundary values accepted, out-of-range and
non-finite scalars rejected, and each value type's constraint asserted.
