# engine-harvest36-004-C005 — run-11 oneshot characterization

Feature: `engine-harvest36-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: flags 0 / 16 -> FLAGS parameter to json_valid() must be between 1 and 15 (see harness).

## Observables

- `flag0` = `rc=1 err=FLAGS parameter to json_valid() must be between 1 and 15`
- `flag16` = `rc=1 err=FLAGS parameter to json_valid() must be between 1 and 15`
