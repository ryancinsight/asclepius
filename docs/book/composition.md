# Independent-Insult Composition

A treatment course is rarely one insult. `IndependentInsults<const N: usize>`
composes statistically independent cell-kill mechanisms — two drugs, heat plus
radiation, or two fractions with independent kill — through the survival
product:

```text
p_combined = 1 − Πᵢ (1 − pᵢ)
```

The type is a zero-sized static strategy parameterized by the real mechanism
count: no allocation, no dispatch, and the count mismatch is a compile-time
array-length error rather than a runtime length check.

## Bounds and monotonicity

Every survival factor `1 − pᵢ` lies in `[0, 1]`, so their product lies in
`[0, 1]` and the combined kill is a probability. Its partial derivative with
respect to any `pⱼ` is the non-negative product of the remaining survival
factors, proving that adding or increasing an insult cannot reduce combined
cell kill:

```rust
extern crate asclepius;

use asclepius::response::composition::IndependentInsults;
use asclepius::{BiologicalResponse, Probability};

let insults = [
    Probability::<f64>::new(0.3).expect("valid probability"),
    Probability::<f64>::new(0.5).expect("valid probability"),
];
let combined = IndependentInsults::<2>
    .evaluate(&insults)
    .expect("valid insults");
assert!((combined.get() - 0.65).abs() < 1e-15); // 1 − 0.7·0.5
```

## Scope note

Independence is a modelling assumption, not a default: correlated insults
need their own joint law. Fractionated courses with repair and repopulation
(the linear-quadratic family) are out of scope for this combinator — see ADR
0001 — while repeated identical thermal steps compose here exactly when each
step's kill is assessed independently.
