# Asclepius release audit

## 0.4.0 Coeus adapter

- Fresh standalone resolution selects one registry Aequitas 0.4.0, Eunomia
  0.12.1, Leto 0.51.0, and Coeus 0.13.0. The 21 tests include analytical and
  central-difference gradient checks on the sequential and Moirai backends.
- The lockfile guard accepts this registry-only graph. A lock generated with
  the actual Eunomia source overlaid as a path fails both the working and
  staged checks.
- The indexed adapter archive records merge 211b5b9ee3b7 and matches the
  locally verified archive checksum. A registry-only consumer starting with
  no lockfile checks the response RMS of 3 Gy and 4 Gy and both partial
  derivatives. [Release](https://github.com/ryancinsight/asclepius/releases/tag/asclepius-coeus-v0.4.0)
