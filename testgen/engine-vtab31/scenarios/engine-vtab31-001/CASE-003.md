# engine-vtab31-001-C003 — run-11 oneshot characterization

Feature: `engine-vtab31-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: xCreate failure surfaces constructor pzErr; no schema entry (see harness).

## Observables

- `badarg` = `rc=1 err=intseries: bad limit 'bogus'`
- `master_count` = `0`
