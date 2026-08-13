# engine-harvest41-001-C003 — run-11 oneshot characterization

Feature: `engine-harvest41-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: FUNCTION IGNORE makes the function yield NULL per row (an ignored aggregate stops aggregating: 3 NULL rows; coalesce fallback 999 per row) and the column READ degrades to the empty-column form (see harness).

## Observables

- `f_ignore_min` = `rows=~|~|~ LOG=[21|~|~|~|~][31|~|min|~|~][20|t|{}|~|~]`
- `f_ignore_coal` = `rows=999|999|999 LOG=[21|~|~|~|~][31|~|coalesce|~|~][31|~|min|~|~][20|t|{}|~|~]`
