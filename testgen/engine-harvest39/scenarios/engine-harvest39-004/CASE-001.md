# engine-harvest39-004-C001 — run-11 oneshot characterization

Feature: `engine-harvest39-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: randomness fills exactly N bytes (tail untouched), draws differ, N=0 writes nothing and the stream continues (see harness).

## Observables

- `tail_untouched` = `1`
- `draws_differ` = `1`
- `n0_no_write` = `1`
- `post_n0_differ` = `1`
