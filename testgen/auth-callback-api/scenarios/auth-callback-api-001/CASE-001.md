# auth-callback-api-001-C001 — run-11 oneshot characterization

Feature: `auth-callback-api-001` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: authorizer DENY on SQLITE_SELECT -> prepare rc (see harness).

## Observables

- `prepare.rc` = `23`
- `cb.called` = `1`
- `errmsg.class` = `SQLITE_AUTH`
