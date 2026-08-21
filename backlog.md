# Asclepius backlog

| ID | Outcome | Scope / non-goals | Acceptance oracle | Class | Status | Dependencies |
| --- | --- | --- | --- | --- | --- | --- |
| ASC-REL-008 | Publish the Asclepius law core through crates.io Trusted Publishing. | `asclepius`, standalone dependency sources, release workflow, and distribution docs; `asclepius-coeus` remains unpublished. | Repository-side exact-source package dry-run passes and the crates.io package identity is indexed; trusted-publishing-only configuration and the matching GitHub Release remain external release-authority actions. | patch | in-progress | Aequitas and Eunomia registry releases |

Repository-side preflight at provider head `db33cca` passes with `cargo publish
--manifest-path crates/asclepius/Cargo.toml --package asclepius --locked
--dry-run`: 35 files, 95.8 KiB, package verification successful. No registry
publication or account-setting claim is inferred from the dry run.

## ASC-VER-016 — Close CI verification gaps [patch] — in progress

- **Owner:** Codex; branch `ci/asclepius-gates`.
- **Scope:** `.github/workflows/ci.yml`, this backlog, and the provider
  checklist.
- **Non-goals:** law implementation, release publication, and the in-flight
  executable book branch.
- **Acceptance:** every cargo gate uses the committed lockfile; a pinned
  `cargo-semver-checks` action compares against the change base; a job builds
  the workspace at the declared Rust 1.95 MSRV; and Rustdoc runs with warnings
  denied.
- **Evidence:** YAML parses; standalone local verification passes with Rust
  1.95 locked workspace check, Rust 1.97 format and warning-denied Clippy,
  20/20 nextest tests, five doctest cases, and warning-denied Rustdoc. Exact-
  head hosted CI remains required before closure.
| ASC-P1-007 | Consolidate the provider source graph. | Remove revision-qualified Aequitas, Eunomia, and Coeus identities from both Asclepius packages; no API or biological-law changes. | Locked metadata resolves one Aequitas, one Eunomia, and one Coeus package family; format, Clippy, Nextest, doctest, and Rustdoc gates pass. | patch | done | Kwavers Hyperion migration |
| ASC-P1-001 | Establish the biological-response foundation. | Aequitas-typed gEUD, TCP, NTCP, CEM43, Arrhenius, independent insults, tissue identity; no imaging, grids, solvers, catalogs, or clinical parameter claims. | ADR proof obligations; analytical, property, differential, layout, allocation, docs, and full package gates. | minor, arch | done: law core merged at `794f8c3` | Aequitas response quantities merged at `be3a1ac` |
| ASC-P1-002 | Move the Coeus gEUD tape expression into an Asclepius adapter. | Autodiff expression only; Helios planning objectives remain Helios-owned. | Coeus value and analytical-gradient differential tests; standalone consumable dependency graph. | minor, arch | done: adapter passes against merged Coeus `85d1970a` | ASC-P1-001 |
| ASC-P1-003 | Migrate Helios and delete response duplicates. | `helios-analysis` response laws and Coeus gEUD expression; DVH storage and planning objectives remain local. | Helios analysis/planning/simulation focused gates and no residual duplicate formula. | minor, arch | done: Helios `4ce96b1` contains direct adoption after 270/270 focused tests and hosted CI | ASC-P1-001, ASC-P1-002 |
| ASC-P1-006 | Admit zero-allocation consumer temperature streams. | Exact-size lazy observations plus single-step increments; no Celsius unit ownership or consumer grid state. | Borrowed/streamed bitwise equality, single-step/history equality, allocation counter, invalid-domain tests, docs, and full package gates. | minor | done: streamed law core merged at `794f8c3` | ASC-P1-001 |
| ASC-P1-004 | Migrate Kwavers and delete response duplicates. | CEM43, Arrhenius damage, independent insult composition; grids, workflows, and tissue catalogs remain local. | Kwavers physics/therapy/Python value-semantic gates and residue scan. | minor, arch | done: Kwavers PR 301 merged as `1cb01fe` with all first-party CI green | ASC-P1-001 |
| ASC-P1-005 | Register Asclepius in Atlas. | Gitlink, stack map, roadmap graduation, ADR, changelog; no unrelated package changes. | Remote-default OID proof and Atlas structural audit. | patch, arch | done: Atlas `71cdc54` pins public Asclepius and records the provider boundary | ASC-P1-003, ASC-P1-004 |

## Gap audit 2026-08-20

Static evidence audit at head `b660646`. No gate was executed; every item below
cites a file and line.

| ID | Outcome | Scope / non-goals | Acceptance oracle | Class | Status | Dependencies |
| --- | --- | --- | --- | --- | --- | --- |
| ASC-VER-010 | Anchor TCP and NTCP against parameter-sensitive published values. | `tests/radiation_theorems.rs` assertions for `LogisticControlProbability` and `LymanComplicationProbability`; no model or parameter-type change. | A test evaluates each law at a dose away from its midpoint and compares against a value computed from the published closed form with a derived tolerance, such that changing the Niemierko exponent factor `4` (`response/radiation/logistic_control.rs:82`) or dropping the `TD50` factor from the Lyman normalization (`response/radiation/normal_complication.rs:70`) fails the suite. | patch | todo | none |
| ASC-VER-011 | Exercise Arrhenius damage with published kinetic parameters. | One reference case in `tests/thermal_theorems.rs` using Henriques-Moritz or Pearce `A`/`Ea` values at a physiological absolute temperature; no change to the law. | The case asserts the damage integral against the closed-form `A exp(-Ea/(RT)) t` evaluated from the cited parameters, with the citation recorded at the assertion site. | patch | todo | ASC-DOC-012 |
| ASC-VER-013 | Anchor gEUD against an independent published value. | One reference case in `tests/radiation_theorems.rs`; no change to `GeneralizedEquivalentUniformDose`. | The existing differential test (`tests/radiation_theorems.rs:83-97`) recomputes the same power mean and is not an independent oracle; a new case reproduces a worked gEUD value from Niemierko or AAPM TG-166 with a derived tolerance. | patch | todo | ASC-DOC-012 |
| ASC-VER-018 | Cross-check the Coeus gEUD gradient with a finite-difference oracle and document the derivation. | `asclepius-coeus/tests/equivalent_uniform_dose.rs` plus a gradient theorem in ADR 0001 and the adapter Rustdoc; no adapter behaviour change. | A central-difference gradient over the fixture doses agrees with the tape gradient inside a step-size-derived bound, and the closed form currently hard-coded at `tests/equivalent_uniform_dose.rs:44-47` is stated and derived in the adapter Rustdoc. | patch | todo | none |
| ASC-DOC-012 | Place the defining sources beside the invariants they ground. | Rustdoc on the five law types and the adapter; README source list stays. | `grep -rn "doi\|DOI\|PMID" crates` returns a locator for each of gEUD, TCP, NTCP, CEM43, and Arrhenius; it currently returns zero matches while README.md lists six defining sources. | patch | todo | none |
| ASC-DOC-014 | Give the book chapters for the thermal, composition, and tissue boundary bullets. | `docs/book/SUMMARY.md` and new chapters; the in-flight book CI branch owns existing chapter edits. | SUMMARY.md covers all five README boundary bullets; it currently stops at gEUD and TCP/NTCP, and `docs/book/README.md` is a two-line stub. | patch | todo | book CI branch `ci/asclepius-book-test` lands |
| ASC-VER-015 | Make the book code fragments executable so the `mdbook-test` gate is not vacuous. | The six `rust,ignore` fences at `docs/book/geud.md:34`, `docs/book/response_values.md:17,35`, `docs/book/tcp_ntcp.md:27,55`; no prose rewrite. | `mdbook test` compiles every Rust fence in the book; only the two `{{#include}}` example pages compile today, so the `mdbook-test: true` step added in `.github/workflows/book-pages.yml:22` cannot detect API drift in the prose. | patch | todo | book CI branch `ci/asclepius-book-test` lands |
| ASC-VER-016 | Close the CI gate gaps the ADR verification section already claims. | `.github/workflows/ci.yml`; no change to the law crates. | Every cargo invocation carries `--locked`; a `cargo-semver-checks` step exists (ADR 0001 claims a semver gate but no workflow runs one); a job builds at the declared MSRV `1.95` (`Cargo.toml:7`, while `rust-toolchain.toml` pins `1.97.0`); `cargo doc` runs under `RUSTDOCFLAGS=-D warnings`. | patch | todo | none |
| ASC-DOC-019 | Bring the README example under a compiled gate. | `crates/asclepius/src/lib.rs` crate docs and the README example block. | The README example compiles in CI. `grep -rn include_str crates` returns no matches, so the example in README.md is verified by no gate and can rot against the API. | patch | todo | none |
| ASC-ARCH-017 | Decide and record the Coeus adapter scalar dimension. | `asclepius-coeus/src/response/radiation/equivalent_uniform_dose.rs`; an ADR revision if the fix is a documented restriction rather than a generalization. | The adapter is either generic over the scalar the way the law core is, or ADR 0001 records why Coeus fixes it. `generalized_equivalent_uniform_dose` takes `&Var<f64, B>` (`equivalent_uniform_dose.rs:52`) while the core monomorphizes over `T: eunomia::RealField`, so `f32` planning tapes are foreclosed. Confirm what Coeus scalars `Var` supports before choosing. | minor | todo | none |
| ASC-SCOPE-021 | Settle whether fractionation response belongs in Asclepius. | An ADR 0001 revision or a new ADR recording the decision; no speculative implementation. | The linear-quadratic model with repair and repopulation, BED/EQD2, alpha-beta handling, and the Kutcher-Burman effective-volume reduction are each recorded as owned or explicitly out of scope. The README boundary claims none of them, yet it cites the Kutcher-Burman volume reduction as a defining source while `response/radiation/normal_complication.rs` implements only the uniform-dose Lyman sigmoid with no volume parameter. | minor, arch | todo | none |
| ASC-PM-020 | Make the ADR index regeneration instruction resolvable in a standalone checkout. | `docs/adr/README.md` header, or a committed generator; the index rows stay generated. | The regeneration command named in the index header runs from an Asclepius clone. It currently reads `python scripts/adr-index.py generate` while the repository tracks no `scripts/` directory, so the path resolves only from the Atlas meta-repo. | patch | todo | Atlas ADR index tooling owner |
