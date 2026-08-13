# engine-btree46-002-C001 — run-11 oneshot characterization

Feature: `engine-btree46-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: THE PINNED C AMALGAMATION (this harness binary) opens the MODERN-written split file: all rowids, the leaf probe, PRAGMA integrity_check ok, root type 5 (see harness).

## Observables

- `modern_count` = `16`
- `modern_rowids` = `1|2|3|4|10|11|12|13|14|15|16|17|18|19|20|21`
- `modern_probe` = `vvvv`
- `modern_integrity` = `ok`
- `modern_root_type` = `5`
