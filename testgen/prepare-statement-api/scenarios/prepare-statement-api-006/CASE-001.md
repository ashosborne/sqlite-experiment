# prepare-statement-api-006-C001 — run-11 oneshot characterization

Feature: `prepare-statement-api-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stmt_readonly + stmt_busy before/after step (see harness).

## Observables

- `stmt_readonly` = `1`
- `stmt_busy.before` = `0`
- `stmt_busy.after_step` = `1`
