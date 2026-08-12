# engine-conn-003-C006 — run-11 oneshot characterization

Feature: `engine-conn-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: trace_v2 STMT captures SQL text (see harness).

## Observables

- `log` = `{CREATE TABLE t(a);} {INSERT INTO t VALUES(42);}`
