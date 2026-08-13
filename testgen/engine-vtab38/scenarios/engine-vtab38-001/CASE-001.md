# engine-vtab38-001-C001 — run-11 oneshot characterization

Feature: `engine-vtab38-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: file db: first CREATE VIRTUAL TABLE runs xCreate (not xConnect); close runs xDisconnect (not xDestroy) (see harness).

## Observables

- `cvt` = `rc=0 err=-`
- `create_calls` = `1`
- `connect_calls` = `0`
- `scan` = `1|2|3|4`
- `disc_at_close` = `1`
- `destroy_at_close` = `0`
