# engine-blob-001-C004 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: nonexistent rowid on open and reopen -> no such rowid (see harness).

## Observables

- `open99` = `rc=1 err=no such rowid: 99`
- `handle_null` = `1`
- `reopen99` = `rc=1 err=no such rowid: 99`
- `close` = `0`
