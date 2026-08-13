# engine-harvest41-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SQLITE_FUNCTION fires at compile time once per occurrence (s1 NULL, s2 = name), outer-then-inner for nested calls; count(*) reads the table with an EMPTY column name and NULL schema (see harness).

## Observables

- `f_abs` = `rows=1|2|3 LOG=[21|~|~|~|~][31|~|abs|~|~][20|t|a|main|~]`
- `f_min` = `rows=-3 LOG=[21|~|~|~|~][31|~|min|~|~][20|t|a|main|~]`
- `f_count` = `rows=3 LOG=[21|~|~|~|~][31|~|count|~|~][20|t|{}|~|~]`
- `f_coal` = `rows=-3 LOG=[21|~|~|~|~][31|~|coalesce|~|~][31|~|min|~|~][20|t|a|main|~]`
