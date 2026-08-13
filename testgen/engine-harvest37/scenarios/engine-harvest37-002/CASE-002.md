# engine-harvest37-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest37-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer args for PRAGMA get/set, TRANSACTION BEGIN/COMMIT, ATTACH, DETACH (see harness).

## Observables

- `prg` = `[19|user_version|~|~|~]`
- `prgset` = `[19|user_version|3|~|~]`
- `txn` = `[22|BEGIN|~|~|~][22|COMMIT|~|~|~]`
- `attach` = `[24|:memory:|~|~|~]`
- `detach` = `[25|aux|~|~|~]`
