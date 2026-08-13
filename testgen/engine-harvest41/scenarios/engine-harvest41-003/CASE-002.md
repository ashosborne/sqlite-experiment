# engine-harvest41-003-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ANALYZE DENY blocks with rc 23 (see harness).

## Observables

- `an_deny` = `rc=23 err=not authorized LOG=[28|t|~|main|~]`
