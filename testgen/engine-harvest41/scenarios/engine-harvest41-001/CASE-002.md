# engine-harvest41-001-C002 — run-11 oneshot characterization

Feature: `engine-harvest41-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: FUNCTION DENY fails prepare rc 1 with C's per-name message: not authorized to use function: NAME (also for a nested call) (see harness).

## Observables

- `f_deny` = `prep.rc=1 err=not authorized to use function: abs LOG=[21|~|~|~|~][31|~|abs|~|~]`
- `f_deny_inner` = `prep.rc=1 err=not authorized to use function: min LOG=[21|~|~|~|~][31|~|coalesce|~|~][31|~|min|~|~]`
