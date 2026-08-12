# engine-conn-001-C006 — run-11 oneshot characterization

Feature: `engine-conn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: open blob handle blocks close like a statement (see harness).

## Observables

- `close` = `rc=5 err=unable to close due to unfinalized statements or unfinished backups`
- `blob_close` = `0`
- `close2` = `0`
