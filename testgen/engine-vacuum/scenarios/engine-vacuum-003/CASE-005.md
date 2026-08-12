# engine-vacuum-003-C005 — run-11 oneshot characterization

Feature: `engine-vacuum-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: invalid target path -> rc 14 unable to open database (see harness).

## Observables

- `bad` = `rc=14 err=unable to open database: /tmp/eftest/no_such_dir/x.db`
