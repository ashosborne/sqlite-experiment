# engine-pager44-002-C002 — run-11 oneshot characterization

Feature: `engine-pager44-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: COMMIT after a multi-row UPDATE persists all pages; a fresh open re-reads them (see harness).

## Observables

- `multi_commit` = `C1|C2|C3|C4`
- `reopen_after_commit` = `1,C1|2,C2|3,C3|4,C4`
