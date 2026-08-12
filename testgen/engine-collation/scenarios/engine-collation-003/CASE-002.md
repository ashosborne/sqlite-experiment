# engine-collation-003-C002 — run-11 oneshot characterization

Feature: `engine-collation-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: factory declines -> error stands (factory still called) (see harness).

## Observables

- `eq` = `prep.rc=1 err=no such collation sequence: lazy2`
- `factory_called` = `1`
- `factory_name` = `lazy2`
