# engine-btree46-001-C003 — run-11 oneshot characterization

Feature: `engine-btree46-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: a second INSERT after the split lands and survives reopen; the root stays interior (see harness).

## Observables

- `post_split_row` = `post-split`
- `post_split_count` = `17`
- `post_split_reopen` = `post-split`
- `root_still_interior` = `5`
