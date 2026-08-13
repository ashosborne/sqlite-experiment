# engine-btree45-002-C002 — run-11 oneshot characterization

Feature: `engine-btree45-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: cursor DELETE removes a rowid cell and literal UPDATE replaces a cell; reopen reflects both; deleted rowid seek is empty; integrity_check ok (see harness).

## Observables

- `after_del_upd` = `10,TEN|30,thirty|40,forty`
- `seek_deleted` = ``
- `integrity` = `ok`
