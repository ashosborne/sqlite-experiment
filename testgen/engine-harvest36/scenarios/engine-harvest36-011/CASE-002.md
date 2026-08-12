# engine-harvest36-011-C002 — run-11 oneshot characterization

Feature: `engine-harvest36-011` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: WITHOUT ROWID target refused: cannot open table without rowid (see harness).

## Observables

- `worowid_rc` = `1 err=cannot open table without rowid: wr`
