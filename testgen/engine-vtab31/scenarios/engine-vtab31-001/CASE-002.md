# engine-vtab31-001-C002 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unknown module -> 'no such module: nosuch' exact; no schema entry (see harness).

## Observables

- `unknown` = `rc=1 err=no such module: nosuch`
- `master_count` = `0`
