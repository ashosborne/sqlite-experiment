# engine-harvest37-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest37-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: URI mode=ro forces readonly (write rc 8); unknown vfs param errors no such vfs (see harness).

## Observables

- `mode_ro_open_rc` = `0`
- `mode_ro_write` = `rc=8 err=attempt to write a readonly database`
- `bad_vfs` = `rc=1 msg=no such vfs: nosuchvfs`
