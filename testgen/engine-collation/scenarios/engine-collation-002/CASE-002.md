# engine-collation-002-C002 — run-11 oneshot characterization

Feature: `engine-collation-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: per-connection: file reopen must re-register (see harness).

## Observables

- `first_conn` = `A|b`
- `no_register` = `prep.rc=1 err=no such collation sequence: caseless`
- `re_registered` = `A|b`
