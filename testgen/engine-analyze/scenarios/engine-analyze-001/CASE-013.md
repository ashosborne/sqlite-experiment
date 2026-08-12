# engine-analyze-001-C013 — run-11 oneshot characterization

Feature: `engine-analyze-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: durable file: reopen keeps sqlite_stat1 (integrity ok) (see harness).

## Observables

- `analyze` = `rc=0 err=-`
- `reopen` = `t,ia,3 1`
- `integ` = `ok`
