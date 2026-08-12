# engine-lookaside35-001-C003 — run-11 oneshot characterization

Feature: `engine-lookaside35-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: negative size / negative count / huge size normalize with rc 0; USED zeros after (see harness).

## Observables

- `neg_sz_rc` = `0`
- `used_after_negsz_zero` = `1`
- `neg_cnt_rc` = `0`
- `used_after_negcnt_zero` = `1`
- `huge_sz_rc` = `0`
