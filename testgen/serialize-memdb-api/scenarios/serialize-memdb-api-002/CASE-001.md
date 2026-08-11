# serialize-memdb-api-002-C001 — run-11 oneshot characterization

Feature: `serialize-memdb-api-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: deserialize the empty-db image (no URI) then re-serialize (see harness).

## Observables

- `deserialize.rc` = `0`
- `reserialize.size` = `4096`
- `roundtrip.same_size` = `1`
