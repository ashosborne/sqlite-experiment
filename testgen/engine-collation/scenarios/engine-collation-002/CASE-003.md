# engine-collation-002-C003 — run-11 oneshot characterization

Feature: `engine-collation-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: unknown collation error shapes (ORDER BY + equality) (see harness).

## Observables

- `order_unknown` = `prep.rc=1 err=no such collation sequence: nope`
- `eq_unknown` = `prep.rc=1 err=no such collation sequence: nope`
