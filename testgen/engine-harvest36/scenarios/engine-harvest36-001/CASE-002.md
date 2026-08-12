# engine-harvest36-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: complete(): quoted END + nested CASE END in trigger bodies; bare ; complete, whitespace not (see harness).

## Observables

- `quoted_end_in_trigger` = `1`
- `nested_case_end` = `1`
- `bare_semi` = `1`
- `whitespace_only` = `0`
