# engine-harvest23-002-C003 — run-11 oneshot characterization

Feature: `engine-harvest23-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: highwater reset returns prior and re-arms (see harness).

## Observables

- `prior_ge` = `1`
- `after_le_prior` = `1`
