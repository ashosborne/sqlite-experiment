# engine-harvest39-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest39-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: two-statement exec fires STMT and PROFILE per statement with the SQL C reports (trailing semicolons) (see harness).

## Observables

- `nstmt` = `2`
- `nprof` = `2`
- `log` = `[STMT:SELECT 1;][PROF:SELECT 1;][STMT:SELECT 2;][PROF:SELECT 2;]`
