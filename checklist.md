# Asclepius execution checklist

## Stable release checklist

Basis `49794f842d79`, representative manifest `asclepius 0.5.0`. Target 1.0.0. [Backlog item](backlog.md#asclepius-stable-qualification).

- [ ] [patch] Run the law suite on the current provider graph. Check the gEUD value and gradient independently, including the equal-dose case and nonuniform inputs, through the real adapter.
- [ ] [arch] Resolve ASC-ARCH-017 against the scalar types supported by Coeus and document the decision before changing the adapter API. Close the law-source and scope decisions in M-law-oracles.
- [ ] [patch] Replay the lock, MSRV, SemVer and executable book gates in M-gates-docs. Exercise the resulting core and adapter archives together in a fresh consumer.
- [ ] [patch] **0.5.1.** Evaluate uniform and unequal dose arrays through core and the Coeus adapter. Check the uniform identity and directional derivative independently, including the adapter’s declared scalar boundary. [Source and baseline](gap_audit.md#incremental-release-baseline).
- [ ] [arch] **0.6.0.** The adapter boundary changed in #130 (Coeus 0.15 types; [changelog](CHANGELOG.md)), so 0.6.0 is the next release of `asclepius-coeus`; the bump awaits release authority (ASC-REL-009 in the backlog). Obtain the ASC-ARCH-017 decision. Check the documented scalar contract and migrate the Coeus caller if it changes. If the current API suffices, take the verified patch line to beta.
- [ ] [major] **1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0.** Freeze the supported surface at beta. The same law suite and executable documentation pass against packaged core and adapter with the supported compiler. Repeat the applicable exits on the exact RC before stable promotion. Run the [release scope](docs/adr/0002-release-scope-1-0.md) exits and attach reviewed, exact-revision evidence for this member’s [Atlas qualification criteria](https://github.com/ryancinsight/atlas/blob/17b0124a7548ad81e817284afc5cb115c1d1bc78/docs/parity/repositories.json).

## Current target 0.4.0

- [x] [major] Verify and publish the Asclepius response-law crate.
      [Release](https://github.com/ryancinsight/asclepius/releases/tag/crate-asclepius-v0.4.0)
- [x] [minor] Resolve the registry Coeus 0.13 graph and verify the adapter.
- [x] [minor] Publish the adapter and verify a fresh crates.io consumer.
      [Release](https://github.com/ryancinsight/asclepius/releases/tag/asclepius-coeus-v0.4.0)

## ASC-VER-016 — Close CI verification gaps

- [x] Add `--locked` to every cargo gate that resolves the workspace.
- [x] Add pinned SemVer and Rust 1.95 MSRV jobs; make Rustdoc warning-denied.
- [x] Validate the workflow and focused local gates. YAML parses; Rust 1.95
      check, Rust 1.97 format, warning-denied Clippy, 20/20 nextest tests,
      five doctest cases, and warning-denied Rustdoc pass from the standalone
      provider context using the shared Atlas target.
- [ ] Push the branch and record exact-head hosted results.

## Codex

- [x] Validate the `asclepius` standalone exact-source archive with
      `cargo publish --manifest-path crates/asclepius/Cargo.toml --package
      asclepius --locked --dry-run` at provider head `db33cca`. Cargo confirms
      the existing crates.io package identity, packages 35 files (95.8 KiB),
      and verifies the package successfully.
- [x] Verify the 0.4.0 trusted-publisher workflow and release tag.
      [Workflow](https://github.com/ryancinsight/asclepius/actions/runs/44652256475)
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
- [ ] ASC-DOC-012: add a resolvable locator (DOI, PMID, or report section) to
      the Rustdoc of `GeneralizedEquivalentUniformDose`,
      `LogisticControlProbability`, `LymanComplicationProbability`, `Cem43`,
      `ArrheniusDamage`, and the Coeus adapter. The sources exist only in
      README.md today.
- [ ] ASC-VER-013: add a Niemierko or TG-166 worked gEUD value as an
      independent oracle beside the existing same-formula differential test.
- [ ] ASC-VER-011: add an Arrhenius case with published kinetic parameters at
      a physiological absolute temperature; the present oracle uses
      `A = 2`, `Ea = 1`, `R = 1` at `T = 1 K`.
- [ ] ASC-VER-018: add a central-difference gradient check to the Coeus
      adapter tests and write the gradient derivation into the adapter Rustdoc
      and an ADR 0001 theorem.
- [ ] ASC-VER-016: add `--locked`, a `cargo-semver-checks` step, an MSRV job at
      `1.95`, and `RUSTDOCFLAGS=-D warnings` to `.github/workflows/ci.yml`.
- [ ] ASC-DOC-019: bring the README example under a compiled gate.
- [ ] ASC-ARCH-017: check what scalars Coeus `Var` supports, then either
      generalize the adapter or record the restriction in ADR 0001.
- [ ] ASC-SCOPE-021: draft the ADR revision settling LQ, BED/EQD2, alpha-beta,
      fractionation, and the Kutcher-Burman volume reduction as owned or
      out of scope.
- [ ] ASC-DOC-014 and ASC-VER-015: after the `ci/asclepius-book-test` branch
      merges, add the thermal, composition, and tissue chapters and make the
      six `rust,ignore` prose fences executable.
- [ ] ASC-PM-020: coordinate with the Atlas ADR index tooling owner so the
      regeneration command in `docs/adr/README.md` resolves from a standalone
      clone.
