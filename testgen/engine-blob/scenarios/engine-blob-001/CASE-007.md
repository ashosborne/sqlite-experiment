# engine-blob-001-C007 — run-11 oneshot characterization

Feature: `engine-blob-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: text cell opened as blob: bytes + readable content (see harness).

## Observables

- `open_text` = `rc=0 err=not an error`
- `bytes` = `5`
- `read` = `rc=0 err=not an error`
- `text` = `texty`
- `close` = `0`
