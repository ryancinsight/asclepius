# ADR 0001: Own shared biological response in Asclepius

- Status: accepted
- Change class: minor, architectural
- Date: 2026-07-20

## Context

Helios owns scalar generalized equivalent uniform dose, logistic tumour-control
probability, and Lyman normal-tissue complication probability in
`helios-analysis`. Helios planning independently reconstructs gEUD on a Coeus
autodiff tape.

Kwavers owns repeated CEM43 calculations in analytical safety, HIFU planning,
transcranial safety, thermal diffusion, and therapy tracking. It also owns two
Arrhenius damage implementations and an independent thermal/mechanical
cell-kill combiner. The formulas are biological response, not acoustic
propagation, radiation transport, image processing, or material law.

The overlap satisfies the Atlas promotion gate: two independent consumers need
the capability, no provider owns this bounded context, and real computation
already exists in incorrect dependency layers.

## Decision

Create the Asclepius workspace with a pure `asclepius` law crate.

- Aequitas owns absorbed dose, molar energy, molar heat capacity, reciprocal
  time, thermodynamic temperature, and time dimensions.
- Transparent validating newtypes own probabilities, damage integrals,
  equivalent exposure, and dimensionless response parameters.
- `BiologicalResponse<T>` uses a GAT for the borrowed observation family and
  associated output/error types.
- Radiation and thermal laws are generic over `T: RealField` and statically
  dispatched.
- `TemperatureHistory` borrows a quantity slice. Cumulative evaluation writes
  into caller-owned storage through monomorphized internal sinks; final-only
  evaluation uses a zero-sized discard sink.
- `IndependentInsults<const N: usize>` is a const-generic ZST that specializes
  the real fixed mechanism count without runtime dispatch.
- `Tissue<'a, Model>` uses `Cow<'a, str>` for borrowed catalogs and owned
  runtime identities.
- Coeus tape construction belongs in a sibling infrastructure adapter, not in
  the `no_std` law crate. The adapter is a workspace-promotion trigger and may
  depend on Coeus; core never depends outward on autodiff infrastructure.

## Theorems and proofs

### Generalized-mean bounds

For positive doses and finite `a != 0`, the generalized-mean inequality gives
`min(D) <= ((sum D_i^a)/N)^(1/a) <= max(D)`. Scaling every dose by the relevant
extremum and factoring the scale outside the power mean preserves the result
and reduces intermediate overflow. Property tests exercise the bounds over
generated positive samples.

### Positive homogeneity

For `c >= 0`,

`M_a(cD) = ((sum (cD_i)^a)/N)^(1/a)
          = (c^a (sum D_i^a)/N)^(1/a)
          = c M_a(D)`.

Uniform-dose and differential tests exercise the identity. IEEE overflow and
underflow remain bounded by native `T`; non-finite outputs are typed failures.

### Probability midpoint and monotonicity

The logistic control denominator is at least one for positive parameters. At
`D = D50`, its ratio term equals one and the result is `1/2`; differentiation
for `D > 0` is positive.

For Lyman NTCP, `t = (D - TD50)/(m TD50)`. Positive `m` and `TD50` make `t`
strictly increasing in `D`; composition with the standard normal CDF preserves
monotonicity, and `Phi(0) = 1/2`.

### Thermal accumulation

Arrhenius rate `A exp(-Ea/(RT))` is positive for positive finite `A`, `Ea`,
`R`, and absolute `T`. Multiplication by positive `dt` makes every rectangle
increment positive. CEM43 uses positive `dt` and `0 < R <= 1`, so its
exponential factor is also positive. Both cumulative outputs are therefore
non-negative and non-decreasing.

### Arrhenius survival

The first-order loss equation `dS/dt = -k(t)S` separates to
`dS/S = -k(t)dt`. Integrating yields
`ln(S(t)/S(0)) = -integral k(t)dt = -Omega`, hence
`S(t)/S(0) = exp(-Omega)` and kill probability is `1 - exp(-Omega)`.

### Independent-insult composition

For probabilities `p_i in [0,1]`, every survival factor `1-p_i` lies in
`[0,1]`. Their product lies in `[0,1]`, so
`1-product(1-p_i)` is a probability. Its partial derivative with respect to
any `p_j` is the non-negative product of the remaining survival factors.

## Rejected alternatives

- Keep consumer-local functions: rejected because formulas and validation
  already repeat across repositories.
- Move DVHs, images, tissue catalogs, or treatment objectives into Asclepius:
  rejected because these have distinct owners and would create a god package.
- Use raw Celsius, seconds, gray, or joules-per-mole scalars: rejected because
  Aequitas can make dimensional mistakes compile-time errors.
- Return cumulative `Vec`s: rejected because consumers already own storage and
  caller-provided slices avoid allocation and copies.
- Dynamic response traits: rejected because model types are known at operation
  boundaries and static dispatch preserves monomorphization.
- Put Coeus in the core crate: rejected because infrastructure would leak into
  the mathematical domain and break `no_std` isolation.

## Migration

1. Add the Aequitas response dimensions and coherent units.
2. Land the Asclepius laws with independent analytical and differential tests.
3. Replace Helios scalar response functions and its Coeus gEUD tape expression
   with Asclepius-owned APIs; delete superseded code.
4. Replace Kwavers CEM43, Arrhenius, and independent-insult arithmetic while
   retaining grids, therapy workflows, and tissue parameter catalogs.
5. Register the repository in Atlas and run cross-package contract tests.

No forwarding wrappers, compatibility aliases, or fallback implementations
remain after each consumer migration.

## Verification

- positive, negative, zero, NaN, infinity, and empty-observation boundaries;
- generic instantiation at `f32` and `f64`;
- generalized-mean property and legacy-formula differential tests;
- published CEM43 42/43/44 °C reference cases;
- Arrhenius analytical constant-temperature oracle and survival identity;
- midpoint, range, and monotonicity for TCP and NTCP;
- const-generic independent-insult composition;
- GAT borrowing, `Cow` pointer identity, transparent layouts, and ZST sizes;
- allocation-free borrowed evaluation;
- no-default-features, Clippy, Nextest, doctests, rustdoc, examples,
  supply-chain, and semver gates;
- consumer package tests after direct integration.
