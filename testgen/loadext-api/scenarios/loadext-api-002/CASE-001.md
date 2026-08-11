# loadext-api-002-C001 — run-11 oneshot characterization

Feature: `loadext-api-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: auto_extension in-process callback invoked on open; cancel stops it (see harness).

## Observables

- `register.rc` = `0`
- `calls.after_open` = `1`
- `cancel.rc` = `1`
- `calls.after_cancel_open` = `1`
