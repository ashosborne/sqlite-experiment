# engine-harvest42-004-C001 — run-11 oneshot characterization

Feature: `engine-harvest42-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: SQLITE_RECURSIVE 33 is SCAN-GATED: unused WITH RECURSIVE fires nothing beyond the top-level 21; a scanned CTE fires [21|c][33|~|~|~|c][21|c][21|c]; a non-recursive WITH fires only the s4-context 21 (see harness).

## Observables

- `a_unused` = `rows=42 LOG=[21|~|~|~|~]`
- `a_scanned` = `rows=1|2|3 LOG=[21|~|~|~|~][21|~|~|~|c][33|~|~|~|c][21|~|~|~|c][21|~|~|~|c]`
- `a_nonrec` = `rows=1 LOG=[21|~|~|~|~][21|~|~|~|x]`
