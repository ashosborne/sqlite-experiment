# engine-blob-001-C003 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reopen repositions to another rowid; bytes/content follow (see harness).

## Observables

- `bytes1` = `2`
- `reopen` = `rc=0 err=not an error`
- `bytes2` = `4`
- `read` = `rc=0 err=not an error`
- `hex` = `BBBBBBBB`
- `close` = `0`
