# Position in the Stack

Asclepius is the Atlas biological-response and treatment-outcome model
foundation. It owns pure biological-response laws expressed over
[Aequitas](https://github.com/ryancinsight/aequitas) quantities and validated
by the `value` module, so consumers can combine response models without
reimplementing or re-deriving the mathematics.

## Ownership boundary

Asclepius owns:

- validated biological probabilities, damage integrals, response parameters,
  and equivalent-exposure values;
- the generalized equivalent uniform dose, logistic tumour-control
  probability, and Lyman normal-tissue complication probability;
- CEM43 equivalent thermal exposure and first-order Arrhenius damage, over
  borrowed temperatures or exact-size one-pass iterators;
- composition of independent response mechanisms; and
- the static response-model contract and tissue/model composition.

Asclepius does not own dose-volume histograms, medical images, voxel grids,
segmentation, material properties, transport solvers, optimization objectives,
autodiff engines, persistence, or device execution. Those remain with Helios,
Kwavers, RITK, Proteus, Coeus, Consus, and Hephaestus. The sibling
`asclepius-coeus` crate translates Asclepius laws into Coeus graph operations;
it does not move the engine or planning objectives into the law core.

## The static response contract

Every law implements `BiologicalResponse<T>` with a generic associated
observation family. Each evaluation borrows dose samples, a temperature
history, or a single fixed quantity without allocation or cloning, returns a
typed `Output` (`Probability<T>` or a quantity), and reports a typed
`ResponseError<T>` when the observation leaves the law's mathematical domain.
Downstream code can therefore select models statically and stay generic over
any `eunomia::RealField` scalar.

## Quantities and composition

Dose observations are Aequitas `AbsorbedDose` quantities; thermal laws accept
borrowed absolute temperatures or an exact-size one-pass iterator, so
consumers map their storage lazily without an intermediate collection.
Cumulative evaluations write into caller-owned slices, and `Tissue` carries a
`Cow<str>` name so static catalogs borrow while runtime-defined tissues own.
The [biological-values example](examples/biological_values.md), the
[gEUD example](examples/geud.md), and the `treatment_response` example show
the seams end to end: validated values in, a reduction and a control law
composed in a typed pipeline, and thermal exposure evaluated over a
temperature history.
