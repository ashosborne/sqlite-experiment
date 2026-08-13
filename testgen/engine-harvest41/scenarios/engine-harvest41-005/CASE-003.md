# engine-harvest41-005-C003 — run-11 oneshot characterization

Feature: `engine-harvest41-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: vtab code DENY is rc 23 for both 29 and 30 (see harness).

## Observables

- `cv_deny` = `rc=23 err=not authorized LOG=[29|vt3|ser|main|~]`
- `dv_deny` = `rc=23 err=not authorized LOG=[30|vt2|ser|main|~]`
