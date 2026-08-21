# Asclepius execution checklist

## ASC-VER-016 — Close CI verification gaps

- [ ] Add `--locked` to every cargo gate that resolves the workspace.
- [ ] Add pinned SemVer and Rust 1.95 MSRV jobs; make Rustdoc warning-denied.
- [ ] Validate the workflow and focused local gates.
- [ ] Push the branch and record exact-head hosted results.

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
