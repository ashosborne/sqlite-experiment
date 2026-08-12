# engine-status34-002-C006 — run-11 oneshot characterization

Feature: `engine-status34-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: cold reopen: CACHE_MISS positive on first read; hi 0 (see harness).

## Observables

- `probe` = `4`
- `cache_miss_pos` = `1`
- `miss_hi_zero` = `1`
