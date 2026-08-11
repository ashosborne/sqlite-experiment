# global-init-config-002-C001 — run-11 oneshot characterization

Feature: `global-init-config-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: config(MULTITHREAD) after init -> MISUSE (see harness).

## Observables

- `config_after_init.rc` = `21`
