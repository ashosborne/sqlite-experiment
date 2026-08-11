# engine-files-001-C002 — durable file round-trip (run 15)

Assert mode: RECORDED. Two-run determinism gate passed. Frozen on the pinned C library.

## Observables

- `write.rc` = `0`
- `reopen.rc` = `0`
- `row0.col0` = `1`
- `row0.col1` = `x`
- `row1.col0` = `2`
- `row1.col1` = `y`
- `read.rc` = `0`
- `cb.rows` = `2`
