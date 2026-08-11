# global-init-config-003-C001 — run-11 oneshot characterization

Feature: `global-init-config-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: db_config ENABLE_FKEY query/set/query (see harness).

## Observables

- `fkey.default` = `0`
- `fkey.after_set` = `1`
