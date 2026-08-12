# engine-collation-002-C005 — run-11 oneshot characterization

Feature: `engine-collation-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: RTRIM / NOCASE equality + order interaction (see harness).

## Observables

- `rtrim_cnt` = `1`
- `nocase_cnt` = `1`
- `nocase_order` = `AA|aa |ab`
