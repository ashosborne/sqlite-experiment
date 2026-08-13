# engine-pager44-002-C002 — run-11 oneshot characterization

Feature: `engine-pager44-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: COMMIT after a multi-row UPDATE persists all pages; a fresh open re-reads them (see harness).

## Observables

- `multi_commit` = `cc|cc|cc|cc`
- `reopen_after_commit` = `1,cc|2,cc|3,cc|4,cc`
