# TCP and NTCP

Asclepius owns the two classic radiation-response probability laws over
validated parameters: the Niemierko logistic tumour-control probability and
the Lyman normal-tissue complication probability. Both take a validated
midpoint dose, but their distinct `Gamma50` and `LymanSlope` types preserve the
difference between Niemierko `gamma50` and Lyman `m`. Both implement the
statically dispatched `BiologicalResponse<T>` contract with the borrowed
observation family `AbsorbedDose<T>`.

## Logistic tumour-control probability

`LogisticControlProbability<T>` evaluates

```
TCP = 1 / (1 + (D50 / D)^(4 gamma50))
```

for a positive midpoint `D50` and a positive slope `gamma50`. The midpoint and
bounds are proven at construction: a positive `D50` and positive `gamma50`
make the denominator at least one, so `0 <= TCP <= 1`, and at `D = D50` the
ratio is one so `TCP = 1/2`. The law is monotone increasing in dose for
`D > 0`.

The following is a focused, non-standalone API fragment:

```rust,ignore
use aequitas::systems::si::{quantities::AbsorbedDose, units::Gray};
use asclepius::response::radiation::LogisticControlProbability;
use asclepius::{BiologicalResponse, Gamma50};

let tcp = LogisticControlProbability::new(
    AbsorbedDose::from_unit::<Gray>(50.0),
    Gamma50::new(2.0)?,
)?
.evaluate(AbsorbedDose::from_unit::<Gray>(60.0))?;
assert!(tcp.get() > 0.5); // above the midpoint dose
```

## Lyman normal-tissue complication probability

`LymanComplicationProbability<T>` evaluates the Lyman model at uniform dose
through the standard-normal CDF:

```
t = (D - TD50) / (m TD50)
NTCP = Phi(t)
```

where `TD50` is the 50% complication dose and `m` is the normalized slope. At
`D = TD50` the argument is zero and `NTCP = 1/2`; because `m TD50 > 0` and the
normal density is positive, NTCP is monotone increasing in dose and its CDF
range proves `0 <= NTCP <= 1`.

```rust,ignore
use aequitas::systems::si::{quantities::AbsorbedDose, units::Gray};
use asclepius::response::radiation::LymanComplicationProbability;
use asclepius::{BiologicalResponse, LymanSlope};

let ntcp = LymanComplicationProbability::new(
    AbsorbedDose::from_unit::<Gray>(45.0),
    LymanSlope::new(0.15)?,
)?
.evaluate(AbsorbedDose::from_unit::<Gray>(40.0))?;
assert!(ntcp.get() < 0.5); // below the midpoint dose
```

## Composition

Both laws share the validation surface: the midpoint must be finite and
positive; `Gamma50` and `LymanSlope` each validate their own positive
parameter, and each evaluated dose is revalidated as finite and non-negative.
Composing a reduction (such as gEUD) with a control
law is a plain typed pipeline — the `treatment_response` example evaluates
`gEUD` over a dose sample and feeds the equivalent dose straight into
`LogisticControlProbability`, and the `IndependentInsults` strategy composes
independent kill probabilities multiplicatively
(`p = 1 - product_i (1 - p_i)`) across mechanisms.
