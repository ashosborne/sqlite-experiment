# engine-harvest37-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: multi-page backup: pagecount>1, remaining tracks step quanta exactly, DONE at 0, dest content verified (see harness).

## Observables

- `multipage` = `1`
- `step1_rc` = `0`
- `rem1_is_pc_minus_1` = `1`
- `step2_rc` = `0`
- `rem2_is_pc_minus_3` = `1`
- `pc_stable` = `1`
- `stepall_rc` = `101`
- `rem_final` = `0`
- `finish_rc` = `0`
- `dst_count` = `200`
- `dst_small` = `42`
