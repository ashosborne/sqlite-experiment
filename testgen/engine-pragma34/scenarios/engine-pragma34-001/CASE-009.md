# engine-pragma34-001-C009 — run-11 oneshot characterization

Feature: `engine-pragma34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: table_xinfo / index_info / index_xinfo row shapes (see harness).

## Observables

- `txinfo` = `0,a,,0,~,0,0|1,b,,0,~,0,0`
- `iinfo` = `0,0,a`
- `ixinfo` = `0,0,a,0,BINARY,1|1,-1,~,0,BINARY,0`
