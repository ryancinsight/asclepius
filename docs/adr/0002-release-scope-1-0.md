# ADR 0002: Asclepius release scope for 1.0

- Status: Accepted
- Change class: minor
- Date: 2026-11-12

## Context

Asclepius (`asclepius` law core + `asclepius-coeus` adapter, workspace
0.3.0) is the Atlas stack's biological-response provider: gEUD, TCP,
NTCP, CEM43, Arrhenius damage, and independent-insult composition over
Aequitas/Eunomia/Coeus quantities. Fourteen open entries remain: law
oracle anchors, source citations, the Coeus scalar-dimension and
fractionation scope decisions, CI/book/README gates, and the crates.io
publication task. No milestone ladder exists to 1.0.

## Decision

Adopt a three-milestone ladder to 1.0 recorded in `backlog/milestones.md`:

- **M-law-oracles** (0.4.0): the laws stand on independent oracles —
  TCP/NTCP, Arrhenius, and gEUD reference anchors, the finite-difference
  gradient cross-check, defining sources cited at the law sites, and the
  scalar-dimension and fractionation scope decisions recorded.
- **M-gates-docs** (0.5.0): the gates stop being vacuous — CI lockfile /
  semver / MSRV / Rustdoc gaps closed, book chapters written and
  executable, README example compiled, ADR index instruction resolvable.
- **M-1.0** (1.0.0): stable law release (authority-gated) with the
  Trusted Publishing closure completed under release authority.

In scope: law oracles, citations, scope decisions, CI/book/README gates,
the repository-side publish dry run. Out of scope: law implementation
changes beyond oracle anchoring (non-goal of the gate items); the
in-flight executable book branch owned work (dependency, not ladder
content); any registry publication or account-setting claim without
release authority (veto point: M-1.0 never forces publication).

## Rejected alternatives

- Plan 1.0 as a single release: rejected because oracle anchoring and
  gate closure are independently verifiable slices with an order between
  them (anchors cite sources; gates execute anchors).
- Keep roadmap implicit in backlog rows: rejected because stack policy
  requires a runnable capability-oriented milestone ladder.

## Consequences

- Open entries carry `milestone:` assignments and name the exit check
  they serve.
- Milestones close only when exit checks pass on fetched default commits;
  1.0 release/publication remains authority-gated.
- This ADR is rewritten in place as scope evidence changes.
