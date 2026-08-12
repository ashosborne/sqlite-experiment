# engine-compile32-001-C012 — run-11 oneshot characterization

Feature: `engine-compile32-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: bare-gate =value mismatch -> 0; pinned COMPILER row (gcc-13.3.0, ADR 0030 pin decision) (see harness).

## Observables

- `bare_gate` = `used=1`
- `bare_gate_valued` = `used=0`
- `compiler_row` = `used=1`
- `compiler_bare` = `used=1`
