# engine-blob-002-C003 — run-11 oneshot characterization

Feature: `engine-blob-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: read past end / negative offset -> SQL logic error, buffer untouched (see harness).

## Observables

- `past_end` = `rc=1 err=SQL logic error`
- `untouched` = `1`
- `neg_off` = `rc=1 err=SQL logic error`
- `off_plus_n` = `rc=1 err=SQL logic error`
