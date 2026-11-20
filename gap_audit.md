# Asclepius release audit

## 0.4.0 Coeus adapter

- Fresh standalone resolution selects one registry Aequitas 0.4.0, Eunomia
  0.12.1, Leto 0.51.0, and Coeus 0.13.0. The 21 tests include analytical and
  central-difference gradient checks on the sequential and Moirai backends.
- The lockfile guard accepts this registry-only graph. A lock generated with
  the actual Eunomia source overlaid as a path fails both the working and
  staged checks.
- The normalized adapter package builds against the published response-law
  and Coeus packages. Close publication when crates.io verifies the archive
  and an independent consumer compiles from an empty lockfile state.
