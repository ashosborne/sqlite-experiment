# engine-harvest41-006-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: trigger-body events carry s4 = trigger name: the body INSERT and the new.column READ (see harness).

## Observables

- `trig_ins` = `rc=0 err=- LOG=[18|t|~|main|~][18|log|~|main|tr][20|t|a|main|tr]`
- `log_rows` = `rows=5 LOG=`
