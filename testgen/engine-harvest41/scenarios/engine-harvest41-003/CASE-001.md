# engine-harvest41-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ANALYZE fires code 28 per analyzed table (s1 = table, s3 = main) - outer event only, the sqlite_stat1 tail is deliberately not frozen (see harness).

## Observables

- `an_all` = `rc=0 err=- LOG=[28|t|~|main|~]`
- `an_t` = `rc=0 err=- LOG=[28|t|~|main|~]`
