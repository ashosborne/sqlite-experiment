# engine-status34-001-C003 — run-11 oneshot characterization

Feature: `engine-status34-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: MALLOC_SIZE/PAGECACHE_SIZE cur==0 hi>0; MALLOC_COUNT positive with hi>=cur (see harness).

## Observables

- `malloc_size_cur0` = `1`
- `malloc_size_hi_pos` = `1`
- `pagecache_size_cur0` = `1`
- `malloc_count_pos` = `1`
- `malloc_count_hi_ge_cur` = `1`
