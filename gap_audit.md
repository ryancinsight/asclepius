# Asclepius correctness audit

## ASC-COR-022 — Independent probability cancellation [patch]

On main `13bbff1e646a460dcc00f7e44d01b0478c94212d`, singleton
`IndependentInsults` violates `1 - (1 - p) = p`: positive binary32 `2^-30`,
binary64 `2^-60` and each format's minimum subnormal return zero. Binary64
`0.3` also moves one ULP. Five of six standalone native controls fail.
Three committed regression groups fail on unchanged production source.

The repair accumulates the union probability directly. For accumulated `c`,
set `h = max(c, p)` and `l = min(c, p)`. Then
`1 - (h + (1 - h) l) = (1 - c)(1 - p)` proves equivalence by induction.
A native fused multiply-add rounds the product and sum once. The ordered
step is monotone for binary round-to-nearest arithmetic: above `h = 1/2`,
the complement is exact; below it, an adjacent increase in `h` dominates
`l` times any complement rounding decrement because `l <= h`.
Native scalar arithmetic, the public API, static strategy and allocation
behavior are preserved. Multiple inputs still incur floating-point rounding;
the change does not promise exact arbitrary-length probability composition.

The repair excludes PR #110's theorem/adapter changes and protected thermal
work. Final exact-revision review, intact hooks, hosted checks and coherent
provider/package qualification remain required. Inherited dependency,
registry and deployment holds are not cleared by source-level validation.

## Validation checkpoint

The six new regression groups pass on the repair in both all-feature and
no-default-feature core runs. Each full core run records 19 passed, one failed
and zero skipped. The unchanged allocation-instrumentation test reports two
allocations and 112 bytes on both this candidate and exact unchanged main.
That existing gate remains blocking and is neither weakened nor repaired here.
Strict Clippy, formatting, Rust 1.95 all-target checks, all eight doctests and
warning-denied Rustdoc pass. All 212 external lock records remain unchanged.

Current Aequitas/Eunomia default branches have advanced beyond the inherited
locked provider graph. These checks reproduce and validate the consumer's
current graph; they do not qualify a new provider cohort or permit retaining an
old dependency as a release workaround. Provider integration remains owned.

## Review correction: adjacent-value monotonicity

The first candidate `6754a269` used an unsorted union recurrence. Native
binary32 inputs `0x3c89d590` and `0x3c89d591`, paired with `0x3d9d9625`,
produce decreasing outputs `0x3dbd64c3` and `0x3dbd64c2`. A binary64
witness likewise decreases one ULP. The larger-first fused symmetric step fixes
these regression witnesses and retains singleton/subnormal behavior.
The tests check increasing inputs, domination of each input and exact
symmetry for a two-insult composition, across primitive and wrapper scalars.
A shrinking property test covers additional adjacent represented inputs.
