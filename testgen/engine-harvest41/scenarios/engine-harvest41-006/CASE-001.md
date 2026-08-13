# engine-harvest41-006-C001 — run-11 oneshot characterization

Feature: `engine-harvest41-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SELECT through a view: base-table READs carry s4 = view name (the WHOLE view body, regardless of outer projection), then view-column READs with s4 NULL, then a nested SELECT consult with s4 = view (see harness).

## Observables

- `view_sel` = `rows=1,2 LOG=[21|~|~|~|~][20|t|a|main|v][20|t|b|main|v][20|v|a|main|~][20|v|b|main|~][21|~|~|~|v]`
- `view_one` = `rows=1 LOG=[21|~|~|~|~][20|t|a|main|v][20|t|b|main|v][20|v|a|main|~][21|~|~|~|v]`
