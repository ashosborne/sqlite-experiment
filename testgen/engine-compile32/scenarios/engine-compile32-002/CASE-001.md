# engine-compile32-002-C001 — run-11 oneshot characterization

Feature: `engine-compile32-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: OMIT census: LOAD_EXTENSION/WAL/VIRTUALTABLE/TRIGGER gates inactive on the pin (see harness).

## Observables

- `load_ext` = `used=0`
- `wal` = `used=0`
- `vtab` = `used=0`
- `trigger` = `used=0`
