# engine-harvest36-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: complete(): line/block comments and open strings gate the terminal semicolon (see harness).

## Observables

- `cmt_eats_semi` = `0`
- `cmt_then_semi` = `1`
- `block_cmt_semi` = `1`
- `open_string` = `0`
