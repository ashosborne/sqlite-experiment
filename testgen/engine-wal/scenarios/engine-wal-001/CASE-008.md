# engine-wal-001-C008 — run-11 oneshot characterization

Feature: `engine-wal-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multi-page dataset (60 text rows) durable through WAL (see harness).

## Observables

- `agg` = `60,1830,row-0001-0003-0007,row-0060-0180-0420`
