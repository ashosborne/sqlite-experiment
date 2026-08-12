# engine-attach33-002-C006 — run-11 oneshot characterization

Feature: `engine-attach33-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: aux trigger ON main table -> cannot reference objects in database main (see harness).

## Observables

- `xon` = `rc=1 err=trigger trg cannot reference objects in database main`
