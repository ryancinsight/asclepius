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
- **Evidence plan:** YAML parse, focused locked local gates, and exact-head
  hosted CI before closure.
| ASC-P1-007 | Consolidate the provider source graph. | Remove revision-qualified Aequitas, Eunomia, and Coeus identities from both Asclepius packages; no API or biological-law changes. | Locked metadata resolves one Aequitas, one Eunomia, and one Coeus package family; format, Clippy, Nextest, doctest, and Rustdoc gates pass. | patch | done | Kwavers Hyperion migration |
| ASC-P1-001 | Establish the biological-response foundation. | Aequitas-typed gEUD, TCP, NTCP, CEM43, Arrhenius, independent insults, tissue identity; no imaging, grids, solvers, catalogs, or clinical parameter claims. | ADR proof obligations; analytical, property, differential, layout, allocation, docs, and full package gates. | minor, arch | done: law core merged at `794f8c3` | Aequitas response quantities merged at `be3a1ac` |
| ASC-P1-002 | Move the Coeus gEUD tape expression into an Asclepius adapter. | Autodiff expression only; Helios planning objectives remain Helios-owned. | Coeus value and analytical-gradient differential tests; standalone consumable dependency graph. | minor, arch | done: adapter passes against merged Coeus `85d1970a` | ASC-P1-001 |
| ASC-P1-003 | Migrate Helios and delete response duplicates. | `helios-analysis` response laws and Coeus gEUD expression; DVH storage and planning objectives remain local. | Helios analysis/planning/simulation focused gates and no residual duplicate formula. | minor, arch | done: Helios `4ce96b1` contains direct adoption after 270/270 focused tests and hosted CI | ASC-P1-001, ASC-P1-002 |
| ASC-P1-006 | Admit zero-allocation consumer temperature streams. | Exact-size lazy observations plus single-step increments; no Celsius unit ownership or consumer grid state. | Borrowed/streamed bitwise equality, single-step/history equality, allocation counter, invalid-domain tests, docs, and full package gates. | minor | done: streamed law core merged at `794f8c3` | ASC-P1-001 |
| ASC-P1-004 | Migrate Kwavers and delete response duplicates. | CEM43, Arrhenius damage, independent insult composition; grids, workflows, and tissue catalogs remain local. | Kwavers physics/therapy/Python value-semantic gates and residue scan. | minor, arch | done: Kwavers PR 301 merged as `1cb01fe` with all first-party CI green | ASC-P1-001 |
| ASC-P1-005 | Register Asclepius in Atlas. | Gitlink, stack map, roadmap graduation, ADR, changelog; no unrelated package changes. | Remote-default OID proof and Atlas structural audit. | patch, arch | done: Atlas `71cdc54` pins public Asclepius and records the provider boundary | ASC-P1-003, ASC-P1-004 |
