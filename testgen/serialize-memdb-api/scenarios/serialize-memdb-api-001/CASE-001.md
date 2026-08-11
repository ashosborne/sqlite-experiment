# serialize-memdb-api-001-C001 — run-11 oneshot characterization

Feature: `serialize-memdb-api-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: serialize empty :memory: size + nonnull (see harness).

## Observables

- `ptr.nonnull` = `1`
- `size` = `4096`
