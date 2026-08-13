# engine-harvest40-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest40-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_profile REPLACES the legacy trace (same slot): only P events fire, with the statement text (see harness).

## Observables

- `nlt` = `0`
- `nlp` = `1`
- `log` = `[P:SELECT count(*) FROM t;]`
