# exec-convenience-api-001 — sqlite3_exec callback loop

Slice: `exec-convenience-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Runs zero-or-more semicolon-separated statements; per-row callback; nonzero callback return aborts with SQLITE_ABORT; errmsg ownership rules.

## Entrypoints (citations)

- `sqlite3_exec()` (api) — `src/legacy.c:30`, `src/sqlite.h.in:430`

## Inputs / outputs / observables

- Return code of first failure; per-row callback invocations (argc, argv text values, column names); *pzErrMsg allocation caller must sqlite3_free

## Behaviour (as implemented)

- sqlite3_exec (src/legacy.c:30) loops prepare/step/finalize over semicolon-separated statements; callback invoked per row with all values as UTF-8 text (NULL → NULL pointer)
- Non-zero callback return aborts with SQLITE_ABORT without executing remaining statements
- NULL callback = statements executed for side effects only

## Validation rules found in code

- Errors stop processing at the failing statement; already-executed statements are NOT rolled back (no implicit transaction wrapper)

## Edge cases found in code

- Column names computed once per statement (first row) and reused
- Empty/whitespace SQL → SQLITE_OK with no callback

## Dependencies

- prepare-statement-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/legacy.c:30`
- `src/sqlite.h.in:430`
