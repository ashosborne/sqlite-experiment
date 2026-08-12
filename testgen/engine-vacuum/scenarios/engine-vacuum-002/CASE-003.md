# engine-vacuum-002-C003 — run-11 oneshot characterization

Feature: `engine-vacuum-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WAL db: VACUUM works, mode stays wal, durable (see harness).

## Observables

- `vac` = `rc=0 err=-`
- `mode` = `wal`
- `data` = `83`
- `reopen` = `83`
