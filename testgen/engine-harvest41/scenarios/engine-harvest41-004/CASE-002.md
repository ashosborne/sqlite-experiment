# engine-harvest41-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ALTER IGNORE silently no-ops the rename (rc 0, old name survives); DENY is rc 23 (see harness).

## Observables

- `alter_ignore` = `rc=0 err=- LOG=[26|main|u|~|~]`
- `after_ignore` = `rows=u LOG=`
- `alter_deny` = `rc=23 err=not authorized LOG=[26|main|u|~|~]`
