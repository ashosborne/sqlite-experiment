# engine-vtab38-004-C002 — run-11 oneshot characterization

Feature: `engine-vtab38-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: xUpdate UPDATE (old=new rowid, values via argv) and DELETE (argc=1 rowid); module storage drives the scans (see harness).

## Observables

- `upd` = `rc=0 err=-`
- `scan2` = `1,99,x|7,20,y`
- `del` = `rc=0 err=-`
- `scan3` = `1,99,x`
