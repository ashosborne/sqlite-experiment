# engine-harvest23-004-C006 — run-11 oneshot characterization

Feature: `engine-harvest23-004` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: cume_dist + named-window combo (see harness).

## Observables

- `cume_dist` = `5,0.2|10,0.4|15,0.6|20,0.8|30,1.0`
- `combo` = `a,10,10,0.33333333333333332|a,20,10,0.66666666666666663|a,30,10,1.0|b,5,5,0.5|b,15,5,1.0`
