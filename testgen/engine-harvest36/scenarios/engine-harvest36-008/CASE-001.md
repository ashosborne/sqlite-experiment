# engine-harvest36-008-C001 — run-11 oneshot characterization

Feature: `engine-harvest36-008` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stmt_status: FULLSCAN_STEP rows-1 per run and accumulates; RUN per execution; VM_STEP/MEMUSED positive (see harness).

## Observables

- `fullscan_after_run` = `4`
- `run_count` = `1`
- `vm_step_pos` = `1`
- `memused_pos` = `1`
- `run_count2` = `2`
- `fullscan_accumulates` = `8`
