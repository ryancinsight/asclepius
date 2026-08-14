# Changelog

All externally observable changes are recorded here.

## Unreleased

### Changed

- Separate the Niemierko `Gamma50<T>` and Lyman `LymanSlope<T>` parameters;
  the former `ResponseSlope<T>` surface could silently interchange distinct
  model quantities, so all in-repository callers now use the domain-specific
  types.

- Enable crates.io publication for the Asclepius law core and replace the
  integration crate's sibling paths with versioned Git sources so standalone
  checkouts resolve the complete workspace.

- CEM43's canonical Kelvin conversion now carries an explicit Eunomia
  `UnitScalar` bound, matching Aequitas' provider-owned quantity-conversion
  contract without widening `RealField`'s public supertraits.

- Consolidate Aequitas, Eunomia, and Coeus consumption onto their versioned
  Git contracts. `Cargo.lock` remains the reproducible revision pin, while
  consumers no longer instantiate revision-qualified provider identities in
  parallel with the canonical source graph.

### Added

- Document the radiation-parameter separation in both directions on the
  constructors themselves: each law's `new` now carries a working example
  using its own parameter alongside a `compile_fail` example that differs
  only in the second argument's type, so the pair evidences the type
  distinction rather than a shared setup error.

- Add an environment-gated crates.io Trusted Publishing workflow with
  package/version validation and short-lived OIDC credentials.

### Fixed

- Scope the README's scalar-genericity claim to the law core. The
  `asclepius-coeus` gEUD adapter varies over the Coeus backend but is pinned
  to `f64`, which the previous unqualified wording did not distinguish.

- Publish Asclepius as a public Atlas provider. Helios and Kwavers now consume
  reviewed Git revisions directly, without sibling-directory source patches.

- Add an allocation-free uniform-temperature observation seam. Borrowed
  `TemperatureHistory` and exact-size `TemperatureSamples<I, T>` streams feed
  one monomorphized CEM43/Arrhenius kernel, while public CEM43 rate and
  single-step increment methods let spatial solvers reuse the same law without
  constructing histories.

- Typed, allocation-free generalized equivalent uniform dose, logistic tumour
  control probability, and Lyman normal-tissue complication probability.
- Typed CEM43 equivalent exposure and Arrhenius damage integration over borrowed
  temperature histories.
- Const-generic independent-insult composition, GAT response contracts, and
  borrowed-or-owned tissue identity.
- Backend-generic Coeus gEUD tape construction with zero-copy validation,
  stabilized forward evaluation, and analytical-gradient conformance.
