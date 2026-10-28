# Thermal Dose and Damage

Hyperthermia response has two validated laws: CEM43 accumulates
time-at-temperature into an equivalent exposure, and the Arrhenius model turns
a temperature history into cell kill through a damage integral.

## CEM43 equivalent exposure

`Cem43` implements the Sapareto-Dewey thermal dose. Each step contributes
`R^(43 − T) · dt` with `R = 0.25` below 43 °C and `R = 0.5` at or above it,
so one minute at 43 °C contributes exactly 60 s of equivalent exposure. The
reference temperature and both compensation factors are typed
(`ThermodynamicTemperature`, `ReciprocalTemperature`, dimensionless), and the
accumulator accepts absolute temperatures — the `R^(T_ref − T)` exponent is
dimensionless by construction:

```rust
extern crate aequitas;
extern crate asclepius;

use aequitas::systems::si::quantities::ThermodynamicTemperature;
use asclepius::response::thermal::Cem43;

let rate = Cem43::canonical()
    .rate(ThermodynamicTemperature::from_base(317.15_f64))
    .expect("valid temperature");
assert_eq!(rate.into_base(), 2.0); // 44 C doubles the 43 C rate
```

## Arrhenius damage and survival

`ArrheniusDamage` evaluates `k = A·exp(−Ea/(R·T))` with molar energy,
molar heat capacity, and absolute temperature quantities, so the exponent is
checked dimensionless. The damage integral `Ω = k·dt` is a validated
`DamageIntegral`; `survival` and `kill_probability` are complements through
`S = exp(−Ω)`:

```rust
extern crate aequitas;
extern crate asclepius;

use aequitas::systems::si::quantities::{
    MolarEnergy, MolarHeatCapacity, ReciprocalTime, ThermodynamicTemperature, Time,
};
use aequitas::systems::si::units::{JoulePerMole, JoulePerMoleKelvin, PerSecond};
use asclepius::response::thermal::ArrheniusDamage;

let law = ArrheniusDamage::new(
    ReciprocalTime::from_unit::<PerSecond>(2.0),
    MolarEnergy::from_unit::<JoulePerMole>(1.0),
    MolarHeatCapacity::from_unit::<JoulePerMoleKelvin>(1.0),
)
.expect("positive parameters");
let kill = ArrheniusDamage::kill_probability(
    law.increment(
        ThermodynamicTemperature::from_base(1.0),
        Time::from_base(1.0),
    )
    .expect("valid step"),
);
assert!(kill.get() > 0.0);
```
