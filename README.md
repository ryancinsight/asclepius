# Asclepius

Asclepius is the Atlas biological-response and treatment-outcome model
foundation. It owns mathematical response laws that are shared by radiation
therapy and therapeutic-ultrasound consumers.

The name refers to Asclepius, the Greek god of medicine and healing.

## Distribution

Asclepius is developed as a
[public GitHub repository](https://github.com/ryancinsight/asclepius) and
published as the [`asclepius`](https://crates.io/crates/asclepius) crate.
Atlas source consumers use reviewed Git revisions directly; no
sibling-directory patch or private source is part of the consumer contract.

The `Crates.io Release` workflow validates a named workspace package on manual
dispatch. After the first release establishes the crate and its trusted
publisher, a GitHub Release tagged `crate-<package>-v<version>` publishes the
matching Cargo version through the protected `crates-io` environment with a
short-lived OIDC token.

## Boundary

Asclepius owns:

- validated biological probabilities, damage integrals, response parameters,
  and equivalent-exposure values;
- generalized equivalent uniform dose, logistic tumour-control probability,
  and Lyman normal-tissue complication probability;
- CEM43 equivalent thermal exposure and first-order Arrhenius damage;
- composition of independent response mechanisms; and
- the static response-model contract and tissue/model composition.

Asclepius does not own dose-volume histograms, medical images, voxel grids,
segmentation, material properties, transport solvers, optimization objectives,
autodiff engines, persistence, or device execution. Those remain with Helios,
Kwavers, RITK, Proteus, Coeus, Consus, and Hephaestus. The sibling
`asclepius-coeus` crate translates Asclepius laws into Coeus graph operations;
it does not move the engine or planning objectives into the law core.

## Example

```rust
use aequitas::systems::si::{
    quantities::AbsorbedDose,
    units::Gray,
};
use asclepius::{
    BiologicalResponse, Gamma50, VolumeEffect,
    response::radiation::{
        GeneralizedEquivalentUniformDose, LogisticControlProbability,
    },
};

let doses = [40.0_f64, 50.0, 60.0].map(AbsorbedDose::from_unit::<Gray>);
let geud_model = GeneralizedEquivalentUniformDose::new(
    VolumeEffect::new(2.0).expect("finite non-zero exponent"),
);
let geud = geud_model.evaluate(&doses).expect("valid dose sample");

let tcp_model = LogisticControlProbability::new(
    AbsorbedDose::from_unit::<Gray>(50.0),
    Gamma50::new(2.0).expect("positive gamma50"),
)
.expect("positive midpoint dose");
let tcp = tcp_model.evaluate(geud).expect("valid equivalent dose");

assert!(tcp.get() > 0.5);
```

Dose observations are borrowed. Thermal laws accept either borrowed absolute
temperatures or an exact-size one-pass iterator, so consumers can lazily map
their storage into Aequitas quantities without an intermediate collection.
Cumulative evaluations write into caller-owned slices, and tissue names use
`Cow<str>` so static catalogs borrow while runtime-defined tissues own.

## Architecture

```text
crates/
├── asclepius/src/
│   ├── contract/
│   │   └── response.rs             # GAT borrowed-observation seam
│   ├── response/
│   │   ├── radiation/
│   │   │   ├── equivalent_uniform_dose.rs
│   │   │   ├── logistic_control.rs
│   │   │   └── normal_complication.rs
│   │   ├── thermal/
│   │   │   ├── history.rs          # borrowed and streamed observations
│   │   │   ├── arrhenius.rs
│   │   │   └── cem.rs
│   │   └── composition/
│   │       └── independent.rs      # const-generic ZST strategy
│   ├── tissue/
│   │   └── model.rs                # Cow identity plus static model
│   └── value/
│       ├── probability.rs
│       ├── damage.rs
│       ├── exposure.rs
│       ├── parameter.rs
│       └── error.rs
└── asclepius-coeus/src/
    ├── response/radiation/
    │   └── equivalent_uniform_dose.rs
    └── value/
        └── response_error.rs
```

Every `lib.rs` and `mod.rs` is a manifest. Model families live in one canonical
leaf, dependencies point inward, and the core crate is `no_std + alloc`.
`BiologicalResponse<T>` uses a GAT for borrowed observations and associated
output/error types. Models monomorphize over `T: eunomia::RealField`; there is
no vtable, scalar widening, unit metadata, or hidden allocation.
`UniformTemperatureObservation<T>` seals the two supported observation shapes:
borrowed `TemperatureHistory` and generic `TemperatureSamples<I, T>`.
The iterator stays inline and monomorphizes into the same integration kernel.
The validated CEM43 rate and single-step CEM43 and Arrhenius methods reuse the
identical increment laws for spatial solvers.

`asclepius-coeus` depends outward on Coeus while core remains independent. Its
gEUD operation is monomorphized over the Coeus backend, validates borrowed
CPU-addressable dose storage without copying it, uses a detached normalization
scale to reduce overflow, and preserves the exact analytical gradient because
the normalized power mean is independent of every positive scale choice.

## Mathematical specification

The equations, domains, theorems, and proofs live beside their implementations
in Rustdoc. The architectural proof obligations and evidence map are in
[ADR 0001](docs/adr/0001-biological-response-boundary.md).

The defining sources are:

- Niemierko, “Reporting and analyzing dose distributions: A concept of
  equivalent uniform dose,” *Medical Physics* 24, 103–110,
  [DOI 10.1118/1.598063](https://doi.org/10.1118/1.598063).
- Lyman, “Complication probability as assessed from dose-volume histograms,”
  *Radiation Research Supplement* 8, S13–S19,
  [PMID 3867079](https://pubmed.ncbi.nlm.nih.gov/3867079/), with the
  Kutcher–Burman non-uniform-volume reduction described in
  [DOI 10.1016/0360-3016(89)90972-3](https://doi.org/10.1016/0360-3016(89)90972-3).
- AAPM Task Group 166, *The Use and QA of Biologically Related Models for
  Treatment Planning*, sections II.F and IV,
  [Report 166](https://www.aapm.org/pubs/reports/rpt_166.pdf).
- Sapareto and Dewey, “Thermal dose determination in cancer therapy,”
  *International Journal of Radiation Oncology Biology Physics* 10, 787–800,
  [PMID 6547421](https://pubmed.ncbi.nlm.nih.gov/6547421/).
- Henriques and Moritz, “Studies of Thermal Injury: I,” *American Journal of
  Pathology* 23, 530–549,
  [PMID 19970945](https://pubmed.ncbi.nlm.nih.gov/19970945/).
- Pearce, “Comparative analysis of mathematical models of cell death and
  thermal damage processes,” *International Journal of Hyperthermia* 29,
  262–280, [DOI 10.3109/02656736.2013.786140](https://doi.org/10.3109/02656736.2013.786140).

These are mathematical model implementations, not clinical validation or
parameter recommendations. Tissue parameters and clinical applicability
remain consumer-owned and require endpoint-specific validation. In particular,
Pearce documents limits of irreversible Arrhenius injury models in the
43–50 °C hyperthermia range, and AAPM TG-166 cautions against treating absolute
TCP/NTCP estimates as the sole plan-quality criterion.

## Verification

The committed gates are:

```sh
cargo fmt --check
cargo check --workspace --all-features
cargo check -p asclepius --no-default-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
cargo test --doc --workspace --all-features
cargo doc --workspace --no-deps --all-features
cargo deny check
```

Executable evidence includes generalized-mean bounds and homogeneity,
TCP/NTCP midpoint and monotonicity, CEM43 reference cases, Arrhenius
non-decreasing damage and survival identity, const-generic composition bounds,
`f32`/`f64` instantiation, transparent layout, ZST routing, allocation-free
borrowed and streamed evaluation, streamed/borrowed bitwise equivalence,
single-step/history equivalence, and differential comparison with the
pre-extraction consumer formulas. The Coeus adapter adds core-value,
closed-form-gradient, large-finite-dose, invalid-domain, and
Sequential/Moirai backend contracts.

### Pre-push hook


Install the hooks once per clone:

```sh
git config core.hooksPath .githooks
```

Git never applies tracked hooks on its own, so this is a one-time step per
clone. The `pre-push` hook runs `scripts/lockfile.py --check`, which is the
same check CI runs. It matters most when working inside the Atlas stack: the
stack's `[patch]` overlay makes cargo resolve first-party dependencies to
local paths and write a `Cargo.lock` with every `source = "git+..."` line
stripped. That lock resolves fine under the overlay and fails every
`--locked` job in CI, so without the hook the corruption is invisible until a
runner reports it. Repair with `python3 scripts/lockfile.py --regenerate`.
## License

Licensed under either the MIT License or Apache License 2.0.
