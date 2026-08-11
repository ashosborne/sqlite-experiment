# prepare-statement-api-006 — Statement introspection (readonly/busy/explain)

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

stmt_readonly/stmt_busy/stmt_explain expose statement state without executing.

## Entrypoints (citations)

- `sqlite3_stmt_readonly()` (api) — `src/vdbeapi.c:2090`, `src/vdbeapi.c:2141`, `src/vdbeapi.c:2105`

## Inputs / outputs / observables

- stmt_readonly boolean, stmt_busy boolean, stmt_explain mode get/set results

## Behaviour (as implemented)

- stmt_readonly (src/vdbeapi.c:2090) true iff the program makes no direct database writes (BEGIN counts as readonly; VACUUM does not)
- stmt_busy (src/vdbeapi.c:2141) true between first step and reset/done
- stmt_explain (src/vdbeapi.c:2105) switches a prepared statement between normal/EXPLAIN/EQP modes, re-preparing if needed

## Validation rules found in code

- stmt_explain returns SQLITE_ERROR if the statement is busy

## Edge cases found in code

- Statements that write only temp/TEMP schema still report readonly=false paths per doc-comments

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeapi.c:2090`
- `src/vdbeapi.c:2141`
- `src/vdbeapi.c:2105`
