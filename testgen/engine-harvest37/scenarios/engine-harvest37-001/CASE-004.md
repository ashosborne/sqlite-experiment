# engine-harvest37-001-C004 — run-11 oneshot characterization

Feature: `engine-harvest37-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: open_v2 flag matrix: READONLY missing 14 / existing write 8; RW-without-CREATE 14; MEMORY flag; zero flags MISUSE (see harness).

## Observables

- `ro_missing_rc` = `14`
- `ro_existing_open_rc` = `0`
- `ro_existing_write` = `rc=8 err=attempt to write a readonly database`
- `rw_nocreate_missing_rc` = `14`
- `memory_flag_rc` = `0`
- `memflag_no_disk` = `1`
- `zero_flags_rc` = `21`
