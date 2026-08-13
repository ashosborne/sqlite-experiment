# engine-btree45-002-C001 — run-11 oneshot characterization

Feature: `engine-btree45-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: table cursor: literal INSERTs persist; reopen SELECTs all rows by rowid and seeks a single rowid (see harness).

## Observables

- `reopen_all` = `10,ten|20,twenty|30,thirty|40,forty`
- `seek_30` = `thirty`
