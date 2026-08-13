# engine-harvest39-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest39-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: three-statement exec: three STMT + three PROFILE in statement order (see harness).

## Observables

- `nstmt` = `3`
- `nprof` = `3`
- `log` = `[STMT:INSERT INTO t VALUES(1);][PROF:INSERT INTO t VALUES(1);][STMT:INSERT INTO t VALUES(2);][PROF:INSERT INTO t VALUES(2);][STMT:SELECT a FROM t;][PROF:SELECT a FROM t;]`
