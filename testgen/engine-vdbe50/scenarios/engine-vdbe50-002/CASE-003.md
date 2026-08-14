# engine-vdbe50-002-C003 — run-11 oneshot characterization

Feature: `engine-vdbe50-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reset+step on the same INSERT statement inserts again with C next rowids (3 then 4) (see harness).

## Observables

- `ins_twice_rc` = `101 101 last 4`
- `final` = `1|1/2|2/3|9/4|9`
