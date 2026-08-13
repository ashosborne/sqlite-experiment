# engine-pager44-001-C002 — run-11 oneshot characterization

Feature: `engine-pager44-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reopen after COMMIT sees the committed rows (durable db file) (see harness).

## Observables

- `reopen_rows` = `1,one|2,two|3,three|4,four`
