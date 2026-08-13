# engine-harvest43-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest43-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: JSON array-index paths: set/replace hit existing elements, out-of-range indexes are NO-OPS for set/insert/replace; insert never overwrites; inserting $[0] into an empty array appends (see harness).

## Observables

- `set_idx` = `rows={"a":[1,99,3]}`
- `set_oob` = `rows={"a":[1,2]}`
- `set_top` = `rows=[10,20,5]`
- `ins_idx` = `rows={"a":[1,2,3]}`
- `ins_oob` = `rows={"a":[1,2]}`
- `ins_empty` = `rows=[7]`
- `repl_idx` = `rows={"a":[1,99,3]}`
- `repl_oob` = `rows={"a":[1,2]}`
