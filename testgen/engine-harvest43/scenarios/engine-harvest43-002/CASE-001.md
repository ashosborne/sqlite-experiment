# engine-harvest43-002-C001 — run-11 oneshot characterization

Feature: `engine-harvest43-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pragma_index_list projects seq/name/unique/origin/partial in reverse creation order with the autoindex last (origin u); IPK tables report zero rows; PRAGMA and TVF forms agree (see harness).

## Observables

- `il_cols` = `seq,name,unique,origin,partial`
- `il_rows` = `rows=0,tpart,0,c,1|1,tb,0,c,0|2,sqlite_autoindex_t_1,1,u,0`
- `il_pragma` = `rows=0,tpart,0,c,1|1,tb,0,c,0|2,sqlite_autoindex_t_1,1,u,0`
- `il_none` = `rows=0`
