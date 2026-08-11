# mutex-subsystem-001-C001 — run-11 oneshot characterization

Feature: `mutex-subsystem-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: mutex_alloc FAST nonnull, enter/leave/free (see harness).

## Observables

- `alloc.nonnull` = `1`
- `enter_leave_free.ok` = `1`
