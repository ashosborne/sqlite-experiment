# engine-harvest23-005-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: set returns prior; over-max clamps (COLUMN/ATTACHED) (see harness).

## Observables

- `prior_col` = `2000`
- `get_col` = `50`
- `prior_clamp` = `50`
- `get_clamped` = `2000`
- `att_clamp_prior` = `10`
- `att_clamped` = `10`
