# engine-harvest37-001-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: USE_URI off: plain open treats file: as a literal path (14); OPEN_URI parses and creates the real path (see harness).

## Observables

- `plain_open_literal_rc` = `14`
- `v2_uri_open_rc` = `0`
- `real_path_created` = `1`
