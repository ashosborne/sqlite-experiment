# engine-vtab38-004-C003 — run-11 oneshot characterization

Feature: `engine-vtab38-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: module SQLITE_CONSTRAINT surfaces rc 19 'constraint failed'; argv shape log pinned (see harness).

## Observables

- `constraint_ins` = `rc=19 err=constraint failed`
- `count_after_constraint` = `1`
- `uplog` = `[INS argc=4 a0=N a1=N rid=1][INS argc=4 a0=N a1=I rid=7][UPD argc=4 old=1 new=1][DEL argc=1 rid=7]`
