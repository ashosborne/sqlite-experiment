# engine-conn-001-C005 — run-11 oneshot characterization

Feature: `engine-conn-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: stepped/reset statement still blocks close until finalize (see harness).

## Observables

- `close` = `rc=5 err=unable to close due to unfinalized statements or unfinished backups`
- `close_after_reset` = `rc=5 err=unable to close due to unfinalized statements or unfinished backups`
- `finalize` = `0`
- `close2` = `0`
