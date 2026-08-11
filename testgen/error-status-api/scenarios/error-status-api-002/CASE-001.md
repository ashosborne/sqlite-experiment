# error-status-api-002-C001 — run-11 oneshot characterization

Feature: `error-status-api-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_limit prior-value protocol on VARIABLE_NUMBER (see harness).

## Observables

- `limit.query` = `32766`
- `limit.set999_returns_prior` = `32766`
- `limit.query_after` = `999`
