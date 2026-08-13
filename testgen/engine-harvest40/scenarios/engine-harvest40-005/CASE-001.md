# engine-harvest40-005-C001 — run-11 oneshot characterization

Feature: `engine-harvest40-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: json_each full vtab columns: object keys by name, JSONB-offset ids, atom NULL for containers, fullkey/path (see harness).

## Observables

- `each_cols` = `key,value,type,atom,id,parent,fullkey,path`
- `each_full` = `a,1,integer,1,1,~,$.a,$|b,[true,null],array,~,5,~,$.b,$`
- `each_arr` = `0,7,integer,7,1,$[0]|1,x,text,x,3,$[1]|2,~,null,~,5,$[2]`
