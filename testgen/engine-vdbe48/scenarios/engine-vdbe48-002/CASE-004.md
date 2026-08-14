# engine-vdbe48-002-C004 — run-11 oneshot characterization

Feature: `engine-vdbe48-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: the scan sees a later INSERT (fresh cells on the next step), no stale program state (see harness).

## Observables

- `pre` = `10/20/30`
- `post` = `10/20/30/40`
- `post_ab` = `10|ten/20|twenty/30|thirty/40|forty`
