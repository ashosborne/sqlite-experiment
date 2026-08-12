# engine-harvest36-010-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-010` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: close(src) BUSY with the unfinished-backups errmsg; close(dst) defers; step/finish error; late closes settle (see harness).

## Observables

- `init_ok` = `1`
- `close_src_rc` = `5`
- `close_src_err` = `unable to close due to unfinalized statements or unfinished backups`
- `close_dst_rc` = `0`
- `step_rc` = `1`
- `finish_rc` = `1`
- `close_src2_rc` = `0`
- `close_dst2_rc` = `21`
