# engine-compile32-003-C002 — run-11 oneshot characterization

Feature: `engine-compile32-003` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: ENABLE_STAT4/API_ARMOR/UNLOCK_NOTIFY/SESSION = 0 (armor per charter; unlock-notify presence check) (see harness).

## Observables

- `stat4` = `used=0`
- `api_armor` = `used=0`
- `unlock_notify` = `used=0`
- `session` = `used=0`
