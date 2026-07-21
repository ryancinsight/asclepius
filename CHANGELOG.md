# Changelog

All externally observable changes are recorded here.

## Unreleased

### Added

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
