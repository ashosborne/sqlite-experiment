# engine-harvest43-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest43-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: json_remove of an array index shifts the remainder; nested $.a[N].b paths mutate and remove; $.a[#] appends and $.a[#-1] replaces the last element; json_extract reads by index (see harness).

## Observables

- `rm_idx` = `rows={"a":[1,3]}`
- `rm_top` = `rows=[20,30]`
- `rm_nested` = `rows={"a":[{},{"b":2}]}`
- `nested_set` = `rows={"a":[{"b":1},{"b":42}]}`
- `extract_idx` = `rows=7`
- `hash_append` = `rows={"a":[1,2,9]}`
- `hash_minus` = `rows={"a":[1,2,9]}`
