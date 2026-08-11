# tokenizer-002-C001 — run-11 oneshot characterization

Feature: `tokenizer-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_complete 3 inputs (see harness).

## Observables

- `complete.terminated` = `1`
- `complete.unterminated` = `0`
- `complete.trigger_open` = `0`
