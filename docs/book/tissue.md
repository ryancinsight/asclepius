# Tissue Boundary

`Tissue<'name, Model>` pairs a runtime identity (a borrowed or owned name,
`Cow` so catalog lookups do not allocate) with one statically dispatched
response law. The tissue owns no dose storage and no grid: it names the
biology, the caller supplies the observation, and evaluation delegates to the
model with its typed error preserved:

```rust
extern crate asclepius;

use asclepius::response::composition::IndependentInsults;
use asclepius::{Probability, Tissue};

let cord = Tissue::borrowed("spinal-cord", IndependentInsults::<2>);
assert_eq!(cord.name(), "spinal-cord");
let kill = cord
    .evaluate(&[
        Probability::<f64>::new(0.3).expect("valid probability"),
        Probability::<f64>::new(0.5).expect("valid probability"),
    ])
    .expect("valid insults");
assert!((kill.get() - 0.65).abs() < 1e-15);
```

## What lives here — and what does not

Tissue is the narrow seam between a planning system's catalog (names,
fractionation schedules, organ-at-risk lists) and the law crates. Imaging,
dose-volume storage, voxel grids, treatment planning, and device execution
stay with consumers: see ADR 0001 for the boundary and the rejected
alternatives (god package, dynamic response traits, raw-scalar quantities).
