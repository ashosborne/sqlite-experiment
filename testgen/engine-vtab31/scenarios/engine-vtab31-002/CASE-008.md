# engine-vtab31-002-C008 — run-11 oneshot characterization

Feature: `engine-vtab31-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: declared types via table_info; runtime typeof int/text (see harness).

## Observables

- `types` = `integer,text|integer,text`
- `tinfo` = `a,INTEGER|b,TEXT`
