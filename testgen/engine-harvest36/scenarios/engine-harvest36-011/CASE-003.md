# engine-harvest36-011-C003 — run-11 oneshot characterization

Feature: `engine-harvest36-011` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: blob write inside an open transaction commits durably (see harness).

## Observables

- `txn_open_rc` = `0`
- `txn_write_rc` = `0`
- `after_commit` = `00AABB00`
