# Asclepius execution checklist

## ASC-VER-016 — Close CI verification gaps

- [x] Add `--locked` to every cargo gate that resolves the workspace.
- [x] Add pinned SemVer and Rust 1.95 MSRV jobs; make Rustdoc warning-denied.
- [x] Validate the workflow and focused local gates. YAML parses; Rust 1.95
      check, Rust 1.97 format, warning-denied Clippy, 20/20 nextest tests,
      five doctest cases, and warning-denied Rustdoc pass from the standalone
      provider context using the shared Atlas target.
- [ ] Push the branch and record exact-head hosted results. Note (2026-10-28):
      a local `--locked` run is not a validity signal here: the
      `D:/atlas/.cargo/config.toml` stack overlay redirects git sources to
      local trees and documents that local lock churn is a never-commit
      development artifact, with `--locked` verified from a clean checkout
      instead. Do not "fix" `--locked` by committing a locally regenerated
      lock.

## Codex

- [x] Validate the `asclepius` standalone exact-source archive with
      `cargo publish --manifest-path crates/asclepius/Cargo.toml --package
      asclepius --locked --dry-run` at provider head `db33cca`. Cargo confirms
      the existing crates.io package identity, packages 35 files (95.8 KiB),
      and verifies the package successfully.
- [ ] Configure and verify the crates.io Trusted Publisher, enable
      trusted-publishing-only mode, and create the matching GitHub Release.
      This remains an external release-authority action; repository-side
      packaging is complete.
- [x] Audit current Helios and Kwavers response ownership.
- [x] Verify primary mathematical sources and model limitations.
- [x] Add the required Aequitas dimensional vocabulary.
- [x] Define the boundary and proof obligations in ADR 0001.
- [x] Complete core implementation, tests, documentation, and package gates.
- [x] Resolve the Coeus standalone dependency blocker and add the adapter.
- [x] Add exact-size streamed thermal observations and single-step increments
      so consumers need no temporary quantity collection.
- [x] Migrate Helios and delete superseded response formulas; Helios
      `4ce96b1` contains direct adoption and exact-head hosted verification.
- [x] Migrate Kwavers and delete superseded response formulas; Kwavers PR 301
      merged as `1cb01fe` after all first-party CI checks passed.
- [x] Register the public repository and consumer implementation revisions in
      Atlas; the final parent pin sweep advances this PM-complete head.
- [x] Consolidate Aequitas, Eunomia, and Coeus onto versioned Git source
      identities; verify both packages and publish the consumer-compatible head.

## gap-audit-2026-08-20 (owner: atlas-gap-audit)

Ordered so that the citation work lands before the reference cases that depend
on it, and so that book items wait for the in-flight book CI branch.

- [x] Audit declared scope against the tree at head `b660646` by static
      evidence only; file the findings as backlog items ASC-VER-010 through
      ASC-PM-020.
- [x] ASC-DOC-012: add a resolvable locator (DOI, PMID, or report section) to
      the Rustdoc of `GeneralizedEquivalentUniformDose`,
      `LogisticControlProbability`, `LymanComplicationProbability`, `Cem43`,
      `ArrheniusDamage`, and the Coeus adapter. The sources exist only in
      README.md today. Each type now carries a `# Reference` section reusing
      the README locators (Niemierko 1997 DOI, Gay-Niemierko 2007 DOI,
      Lyman PMID, Kutcher-Burman DOI, Sapareto-Dewey PMID, Henriques-Moritz
      PMID, Pearce DOI, TG-166 report); warning-denied Rustdoc and doctests
      pass.
- [x] ASC-VER-010: add off-midpoint TCP and NTCP reference cases. Confirm the
      new assertions fail when the Niemierko `4` factor or the Lyman `TD50`
      normalization is perturbed, since every current assertion holds for any
      positive parameter value. `radiation_theorems.rs` now asserts
      TCP(40/60) and NTCP(40/60) at `D50 = TD50 = 50`, `gamma50 = 2`,
      `m = 0.2` against CPython-evaluated oracles; halving the `4` factor
      (0.6746/0.2906) and dropping the `TD50` normalization (1.0/0.0) both
      fail the 64-ulp bound as confirmed by mutation runs.
- [x] ASC-VER-013: add a Niemierko or TG-166 worked gEUD value as an
      independent oracle beside the existing same-formula differential test.
      Delivered as a precision-independent oracle instead: no citable worked
      number surfaced in search, so `radiation_theorems.rs` pins the
      `[2, 4, 8, 16]`, `a = 2.5` case against a 50-digit `Decimal`
      evaluation (`9.92931284656492885177...`) within 64 ulps. A literature
      worked value remains welcome if one is sourced.
- [x] ASC-VER-011: add an Arrhenius case with published kinetic parameters at
      a physiological absolute temperature; the present oracle uses
      `A = 2`, `Ea = 1`, `R = 1` at `T = 1 K`. `thermal_theorems.rs` now
      asserts the Henriques-Moritz rate (`A = 3.1e98 s^-1`,
      `Ea = 627.6 kJ/mol`) at `T = 317.15 K` against a CPython-evaluated
      `1.3414998998367949e-05 s^-1` within a derived `1e-12` relative bound.
- [x] ASC-VER-018: add a central-difference gradient check to the Coeus
      adapter tests and write the gradient derivation into the adapter Rustdoc
      and an ADR 0001 theorem. The per-backend central-difference checks and
      analytic-gradient assertions already pass; the derivation now also lives
      in the adapter Rustdoc and ADR 0001 ("gEUD reverse-mode gradient").
- [x] ASC-VER-016: add `--locked`, a `cargo-semver-checks` step, an MSRV job at
      `1.95`, and `RUSTDOCFLAGS=-D warnings` to `.github/workflows/ci.yml`.
      Verified already present: `--locked` on every cargo invocation, MSRV
      `1.95.0` job, `RUSTDOCFLAGS=-D warnings` doc step, SemVer via the
      shared `semver-gate.yml` workflow, plus a lockfile-integrity guard job.
- [x] ASC-DOC-019: bring the README example under a compiled gate.
      `crates/asclepius/src/lib.rs` doc-includes the repository README, so
      the usage example compiles under `cargo test --doc` (7 doctests pass,
      README fence included).
- [x] ASC-ARCH-017: check what scalars Coeus `Var` supports, then either
      generalize the adapter or record the restriction in ADR 0001.
      Generalized: `Var<T: Scalar>` plus generic `div`/`mean`/`mul`/`pow`
      cover `f32`, and the adapter plus `AutodiffResponseError` are now
      scalar-generic over the Coeus/eunomia/leto bound bridge, with `f32`
      value-and-gradient contract tests on both backends green.
- [x] ASC-SCOPE-021: draft the ADR revision settling LQ, BED/EQD2, alpha-beta,
      fractionation, and the Kutcher-Burman volume reduction as owned or
      out of scope. ADR 0001 now records single-course laws owned and
      LQ/BED/EQD2/alpha-beta plus the K-B effective-volume reduction out of
      scope, grounded in a repository-wide no-implementation search; "LKB"
      call sites documented as EUD-fed Lyman only.
- [x] ASC-DOC-014 and ASC-VER-015: after the `ci/asclepius-book-test` branch
      merges, add the thermal, composition, and tissue chapters and make the
      six `rust,ignore` prose fences executable. The branch merged (PR #22),
      so `thermal.md`, `composition.md`, and `tissue.md` are added and
      registered, all five `rust,ignore` fences are executable `rust` fences
      with `extern crate` lines, and every prose fence passes under the
      CI-equivalent staged `mdbook test` plus direct `rustdoc --test` runs.
- [ ] ASC-PM-020: coordinate with the Atlas ADR index tooling owner so the
      regeneration command in `docs/adr/README.md` resolves from a standalone
      clone.
