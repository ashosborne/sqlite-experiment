# engine-harvest39-003-C001 — run-11 oneshot characterization

Feature: `engine-harvest39-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: PER-ROW expiry: writing a different row leaves the handle readable and writable; writing its own row expires it (rc 4, bytes 0) (see harness).

## Observables

- `open_rc` = `0`
- `read_after_other_row` = `rc=0 bytes=5 b0=1`
- `write_after_other_row_rc` = `0`
- `read_after_own_row` = `rc=4 bytes=0`
