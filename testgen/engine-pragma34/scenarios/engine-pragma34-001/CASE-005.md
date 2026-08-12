# engine-pragma34-001-C005 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: query_only enforced: write -> readonly rc 8, off -> ok (see harness).

## Observables

- `qo0` = `0`
- `qo_on` = `rc=0 err=-`
- `write_blocked` = `rc=8 err=attempt to write a readonly database`
- `qo_off` = `rc=0 err=-`
- `write_ok` = `rc=0 err=-`
