# engine-harvest23-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest23-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: memory_highwater latches the peak (sticky) (see harness).

## Observables

- `peak_ge` = `1`
- `sticky` = `1`
