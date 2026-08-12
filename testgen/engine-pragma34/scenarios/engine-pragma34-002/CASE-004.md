# engine-pragma34-002-C004 — run-11 oneshot characterization

Feature: `engine-pragma34-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: pragma_compile_options TVF = v32 38-entry fingerprint (see harness).

## Observables

- `tvf_copt_count` = `38`
- `tvf_copt_first` = `ATOMIC_INTRINSICS=1|COMPILER=gcc-13.3.0`
