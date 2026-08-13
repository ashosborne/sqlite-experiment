# engine-btree46-001-C002 — run-11 oneshot characterization

Feature: `engine-btree46-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reopen after the split returns every rowid in order plus low- and high-leaf probes (see harness).

## Observables

- `reopen_count` = `16`
- `reopen_rowids` = `1|2|3|4|10|11|12|13|14|15|16|17|18|19|20|21`
- `reopen_hi` = `vvvv`
- `reopen_low` = `cccc`
