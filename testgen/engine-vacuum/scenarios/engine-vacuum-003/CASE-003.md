# engine-vacuum-003-C003 — run-11 oneshot characterization

Feature: `engine-vacuum-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: existing target -> output file already exists (see harness).

## Observables

- `again` = `rc=1 err=output file already exists`
