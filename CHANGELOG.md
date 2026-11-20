# Changelog

All externally observable changes are recorded here.

## 0.4.0

### Breaking

- Use registry Aequitas 0.4 for public dose, time, and temperature types
  and Eunomia 0.12.1 for scalar contracts. Update direct Aequitas
  requirements to 0.4. The Coeus adapter takes Coeus 0.13 tensors.

### Fixed

- Measure foreign-thread allocator activity with an allocation-free
  handshake. The test still requires the worker to allocate and release
  1024 bytes while the parent reports zero allocations.

## Unreleased

### Changed

- Admit Leto 0.50.0 in the lock. Asclepius reaches Leto through Coeus 0.12.0
  and Apollo 0.29.5, both of which still demand `<0.49.0`, so `Cargo.lock`
  moves three first-party rows onto their first default-branch revisions whose
  Leto bounds admit 0.50: Coeus `7e1ded59` (from `5033ec53`), Apollo
  `b7bffe1c` (from `0c651487`) and Leto `6483eac3` (from `39d65120`), with
  `leto` and `leto-ops` at 0.50.0. Leto 0.50 removes the `VecStorage::uninit`
  constructor so it is a breaking change for the provider, but Asclepius calls
  no `VecStorage` method, owns no Leto or Apollo type in its public surface,
  and its Coeus adapter imports only `coeus-autograd`, `coeus-core`,
  `coeus-ops` and `coeus-tensor` items, so no source change is needed. `cargo metadata --locked` resolves the
  export outside the Atlas dependency overlay, which leaves the lock
  byte-identical and its git source rows intact.

- Advance the Moirai provider contract from 0.11.4 to 0.12.0 and refresh every
  first-party lock row onto its repository's current default head. The lock is
  regenerated outside the Atlas dependency overlay, so the `source = "git+..."`
  rows survive and the registry rows are unchanged. Moirai 0.12.0 removes the
  required `StateWord::get_mut` method, which Asclepius neither implements nor
  calls, and Asclepius re-exports no Moirai or Eunomia type, so the public
  Asclepius surface is unchanged.

### Fixed

- Ship the full Apache License 2.0 text in `LICENSE-APACHE`. The manifest
  declares `MIT OR Apache-2.0`, but the file held only the short-form notice,
  so published packages did not include the license terms they offered.

- Scope the README's scalar-genericity claim to the law core. The
  `asclepius-coeus` gEUD adapter varies over the Coeus backend but is pinned
  to `f64`, which the previous unqualified wording did not distinguish.

- De-flake the allocation-instrumentation tests: the two zero-allocation
  windows measured through the process-global `stats_alloc` allocator inside
  one parallel libtest binary, so concurrent tests' bookkeeping allocations
  leaked into the measured regions and the assertions failed
  non-deterministically (roughly every other run under plain `cargo test`).
  The windows now live in a dedicated `allocation_instrument` integration
  test binary whose single test runs both windows sequentially in an
  otherwise-allocating-free process; `composition_and_layout` keeps the
  value, composition, and representation invariants. The nextest CI gate,
  which isolates every test into its own process, was never affected.

- Close the continuous-integration verification gap: locked workspace gates,
  warning-denied Rustdoc, a Rust 1.95 MSRV check, and a pinned SemVer check
  now run on pull requests.

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
