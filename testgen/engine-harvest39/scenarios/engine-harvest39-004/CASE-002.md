# engine-harvest39-004-C002 — run-11 oneshot characterization

Feature: `engine-harvest39-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: test_control PRNG SAVE/RESTORE replays the stream; re-seeding with the same seed replays (within-run predicates, no raw bytes) (see harness).

## Observables

- `save_rc` = `0`
- `restore_rc` = `0`
- `restore_replays` = `1`
- `next_differs` = `1`
- `seed_rc` = `0`
- `seed_rc2` = `0`
- `seed_replays` = `1`
