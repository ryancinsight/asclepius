# Asclepius execution checklist

## Codex

- [ ] Publish `asclepius` from a standalone exact-source archive, register its
      Trusted Publisher, enforce trusted-publishing-only mode, and create the
      matching GitHub Release.
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
