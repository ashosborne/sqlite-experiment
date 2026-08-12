# engine-conn-002-C004 — run-11 oneshot characterization

Feature: `engine-conn-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: busy_timeout replaces a handler (handler never fires); still BUSY after wait (see harness).

## Observables

- `timeout_rc` = `0`
- `blocked` = `rc=5 err=database is locked`
- `handler_calls` = `0`
