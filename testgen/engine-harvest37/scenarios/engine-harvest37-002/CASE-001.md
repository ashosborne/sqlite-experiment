# engine-harvest37-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer argument strings for INSERT/UPDATE(per-column)/DELETE/SELECT+READ (see harness).

## Observables

- `ins` = `[18|t|~|main|~]`
- `upd` = `[23|t|a|main|~][20|t|b|main|~]`
- `del` = `[9|t|~|main|~][20|t|a|main|~]`
- `sel` = `[21|~|~|~|~][20|t|a|main|~][20|t|b|main|~]`
