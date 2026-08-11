# error-status-api-001-C001 — errmsg + errcode after failed prepare (invalid SQL)

Feature: `error-status-api-001` (discovery/error-status-api/features/error-status-api-001.md)
Kind: characterization · Assert mode: **TO_BE_RECORDED** · Evidence gate: observed-in-code
Citations: `src/main.c:2743` (errmsg), `src/main.c:2851` (errcode), `src/main.c:2866` (extended_errcode)

## Preconditions / fixtures
- Baseline build per `overnight/BASELINE.md` (default Unix amalgamation).
- Fresh connection: `sqlite3_open(":memory:", &db)` → rc captured (expected-shape SQLITE_OK, recorded not asserted).

## Inputs
- Invalid SQL text: `"SELECTT 1"` (deterministic syntax error; no fixtures/PII).

## Boundary invoke
1. `rc1 = sqlite3_prepare_v2(db, "SELECTT 1", -1, &stmt, NULL)`
2. `ec = sqlite3_errcode(db)`; `xec = sqlite3_extended_errcode(db)`
3. `msg = sqlite3_errmsg(db)`

## Observables to capture (RECORD stage freezes these)
- `prepare.rc` (integer)
- `errcode.value`, `extended_errcode.value` (integers)
- `errmsg.text` — captured verbatim for the golden; the *exact English wording* is
  operator-DEFERRED as a pinned contract (see DEFERRED.md): RECORD stores it, but wording-drift
  classification is a human call at execution review.

## Scrub
- None (no secrets, no PII, static inputs).

## Notes
- Error state is per-connection and overwritten by subsequent calls (card edge case) — harness
  captures observables immediately after the failing call, before any other API use.
