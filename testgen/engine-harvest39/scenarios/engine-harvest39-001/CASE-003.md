# engine-harvest39-001-C003 — run-11 oneshot characterization

Feature: `engine-harvest39-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: prepared statement fires STMT at execution and PROFILE at completion (no semicolon) (see harness).

## Observables

- `nstmt` = `1`
- `nprof` = `1`
- `log` = `[STMT:SELECT 3][PROF:SELECT 3]`
