# engine-vtab38-001-C002 — run-11 oneshot characterization

Feature: `engine-vtab38-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: reopen: unregistered module errors exactly; master row persists (rootpage 0 + sql); re-register -> SELECT runs xConnect; DROP runs xDestroy (see harness).

## Observables

- `noreg_select` = `prep.rc=1 err=no such module: ser`
- `master` = `table,vt,0,CREATE VIRTUAL TABLE vt USING ser(4)`
- `rereg_select` = `1|2|3|4`
- `reopen_create_calls` = `0`
- `reopen_connect_calls` = `1`
- `drop` = `rc=0 err=-`
- `destroy_after_drop` = `1`
