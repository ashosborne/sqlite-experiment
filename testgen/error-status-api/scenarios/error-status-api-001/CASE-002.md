# error-status-api-001-C002 — errcode/errmsg on NULL db handle

Feature: `error-status-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code
Citations: card Validation section ("Calling on NULL db → 'out of memory'/MISUSE-safe static answers (guarded)"), `src/main.c:2743`

## Preconditions / fixtures
- None. No connection opened. Library init state: whatever a fresh process gives (record it).

## Inputs
- `db = NULL`

## Boundary invoke
1. `ec = sqlite3_errcode(NULL)`
2. `msg = sqlite3_errmsg(NULL)`

## Observables to capture
- `errcode(NULL).value` (integer, as implemented)
- `errmsg(NULL).text` (static string, as implemented — captured verbatim)

## Scrub
- None.

## Notes
- The card documents guarded static answers; the harness must not dereference beyond the returned
  pointer (defensive capture). Whatever the implementation returns is the pin — no invented expects.
