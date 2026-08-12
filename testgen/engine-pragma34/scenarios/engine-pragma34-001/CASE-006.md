# engine-pragma34-001-C006 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ignore_check_constraints enforced on/off around CHECK (see harness).

## Observables

- `chk_violate` = `rc=19 err=CHECK constraint failed: c > 0`
- `icc_on` = `rc=0 err=-`
- `chk_pass` = `rc=0 err=-`
- `u_rows` = `-5`
- `icc_off` = `rc=0 err=-`
- `chk_violate2` = `rc=19 err=CHECK constraint failed: c > 0`
