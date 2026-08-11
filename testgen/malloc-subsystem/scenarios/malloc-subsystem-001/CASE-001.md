# malloc-subsystem-001-C001 — run-11 oneshot characterization

Feature: `malloc-subsystem-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: malloc64 nonnull + msize>=64 + free (see harness).

## Observables

- `malloc.nonnull` = `1`
- `msize.ge64` = `1`
