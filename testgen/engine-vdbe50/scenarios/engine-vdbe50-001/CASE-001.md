# engine-vdbe50-001-C001 — run-11 oneshot characterization

Feature: `engine-vdbe50-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: C EXPLAIN of INSERT INTO t(a,b) VALUES(1,one): Init(0,8), OpenWrite(0,root,0,p4=2), Integer->r2, String8->r3, NewRowid->r1, MakeRecord(2,2,4,p4=DB affinity string), Insert(0,4,1,p4=t,p5=57), Halt, Transaction(0,1,cookie,p4=0,p5=1 - WRITE txn p2=1), Goto(0,1) (see harness).

## Observables

- `x_ins_cols` = `0|Init|0|8|0|~|0|~/1|OpenWrite|0|2|0|2|0|~/2|Integer|1|2|0|~|0|~/3|String8|0|3|0|one|0|~/4|NewRowid|0|1|0|~|0|~/5|MakeRecord|2|2|4|DB|0|~/6|Insert|0|4|1|t|57|~/7|Halt|0|0|0|~|0|~/8|Transaction|0|1|1|0|1|~/9|Goto|0|1|0|~|0|~`
