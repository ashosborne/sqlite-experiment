# engine-harvest43-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest43-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pragma_foreign_key_list projects id/seq/table/from/to/on_update/on_delete/match in reverse declaration order; composite keys share an id across seq 0/1; defaults NO ACTION/NONE (see harness).

## Observables

- `fk_cols` = `id,seq,table,from,to,on_update,on_delete,match`
- `fk_rows` = `rows=0,0,t,y,a,NO ACTION,NO ACTION,NONE|1,0,p,x,id,SET NULL,CASCADE,NONE`
- `fk_pragma` = `rows=0,0,t,y,a,NO ACTION,NO ACTION,NONE|1,0,p,x,id,SET NULL,CASCADE,NONE`
- `fk_comp` = `rows=0,0,t,m,a,NO ACTION,NO ACTION,NONE|0,1,t,n,b,NO ACTION,NO ACTION,NONE`
- `fk_none` = `rows=0`
