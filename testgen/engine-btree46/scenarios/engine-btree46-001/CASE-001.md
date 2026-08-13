# engine-btree46-001-C001 — run-11 oneshot characterization

Feature: `engine-btree46-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: overflow forces the first split: page_count 2->4, root 0x0d -> interior 0x05, pages 3/4 are 0x0d leaves, all 16 rows and min/max intact (see harness).

## Observables

- `pages_small` = `2`
- `root_type_small` = `13`
- `pages_after_overflow` = `4`
- `root_type_after` = `5`
- `p3_type` = `13`
- `p4_type` = `13`
- `count_all` = `16`
- `min_max` = `1,21`
