# engine-harvest36-005-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: DROP parent with child rows -> 19; child-first then parent OK (see harness).

## Observables

- `drop_parent_blocked` = `rc=19 err=FOREIGN KEY constraint failed`
- `drop_child_first` = `rc=0 err=-`
- `then_parent_ok` = `rc=0 err=-`
