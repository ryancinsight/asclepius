# gEUD and Power Mean

`GeneralizedEquivalentUniformDose<T>` evaluates the generalized equivalent
uniform dose for a sample of absorbed doses `D_i >= 0` and a finite non-zero
volume-effect exponent `a`:

```
gEUD = ((1/N) sum_i D_i^a)^(1/a)
```

The law implements `BiologicalResponse<T>` with the borrowed observation
family `&[AbsorbedDose<T>]`, so consumers hand over their dose slice directly
without allocation or cloning.

## Generalized-mean bounds

For positive observations and finite `a != 0`, the generalized-mean inequality
gives `min(D) <= gEUD <= max(D)`. A uniform observation therefore returns that
dose exactly, up to native floating-point rounding. The exponent chooses the
limit the mean approaches: `a = 1` reproduces the arithmetic mean, while large
positive `a` drives gEUD toward the maximum dose and negative `a` toward the
minimum.

## Positive homogeneity

For `c >= 0`, substituting `c D_i` and factoring `c^a` out of the mean gives
`gEUD(c D) = c gEUD(D)`. The implementation performs this factorization
explicitly — normalizing by the maximum dose for positive `a` and the minimum
dose for negative `a` — which reduces overflow and underflow relative to
evaluating `D_i^a` directly.

The following is a focused, non-standalone API fragment:

```rust,ignore
use aequitas::systems::si::quantities::AbsorbedDose;
use asclepius::response::radiation::GeneralizedEquivalentUniformDose;
use asclepius::{BiologicalResponse, VolumeEffect};

let doses = [1.0_f64, 2.0, 3.0, 4.0].map(AbsorbedDose::from_base); // Gy
let geud = GeneralizedEquivalentUniformDose::new(VolumeEffect::new(1.0)?)
    .evaluate(doses.as_slice())?;
assert!((*geud.as_base() - 2.5).abs() < 1e-10); // the mean
```

## Validation

Each dose is revalidated as finite and non-negative before use. For negative
exponents, zero doses are rejected because `0^a` is not defined for `a < 0`
(that condition is reported as an `InvalidObservation` naming the index and
the `FinitePositive` constraint). An empty slice returns
`ResponseError::EmptyObservation`, and a non-finite result from otherwise
valid inputs returns `ResponseError::NonFiniteResult`.

The runnable [gEUD example](examples/geud.md) demonstrates the three anchor
behaviors: uniform dose returns the dose for any exponent, `a = 1` returns the
mean of a mixed distribution, and large positive `a` approaches the maximum.
