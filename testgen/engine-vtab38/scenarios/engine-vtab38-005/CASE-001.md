# engine-vtab38-005-C001 — run-11 oneshot characterization

Feature: `engine-vtab38-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: eponymous-only module (xCreate NULL): SELECT without CREATE runs xConnect; CREATE VIRTUAL TABLE refuses no such module; no master row (see harness).

## Observables

- `epo_select` = `1|2|3`
- `epo_create_calls` = `0`
- `epo_connect_calls` = `1`
- `epo_cvt` = `rc=1 err=no such module: epo`
- `epo_master` = `0`
