# prepare-statement-api-001 — Prepare family (v1/v2/v3, UTF-8/16, prepFlags)

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Six public prepare variants funnel to sqlite3Prepare via sqlite3LockAndPrepare (schema-lock retry).

## Entrypoints (citations)

- `sqlite3_prepare_v2()` (api) — `src/prepare.c:955`, `src/prepare.c:700`, `src/prepare.c:854`, `src/sqlite.h.in:4637`

## Inputs / outputs / observables

- Return code + sqlite3_stmt* out-param (NULL for whitespace/comment-only SQL with SQLITE_OK)
- pzTail points at first unconsumed byte (multi-statement input)

## Behaviour (as implemented)

- Six variants (UTF-8/16 × v1/v2/v3, src/prepare.c:943-1095) funnel to sqlite3Prepare (src/prepare.c:700) via sqlite3LockAndPrepare (src/prepare.c:854) which retries on SQLITE_SCHEMA after reloading the schema
- v2/v3 produce statements that auto-recompile on schema change and remember the SQL text; v3 adds prepFlags (PERSISTENT/NORMALIZE/NO_VTAB)
- Compiles only the FIRST statement; tail returned for the rest

## Validation rules found in code

- nByte semantics: negative → read to NUL; exact-length inputs must include room or be NUL-terminated (documented perf caveat)
- Syntax errors → SQLITE_ERROR with sqlite3_errmsg detail and error offset

## Edge cases found in code

- Schema-lock contention during prepare invokes the busy handler through LockAndPrepare
- Preparing against a zombie/closed db → SQLITE_MISUSE
- v1 statements return SQLITE_SCHEMA at step time instead of auto-reprepare

## Dependencies

- tokenizer
- parser-grammar

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/prepare.c:955`
- `src/prepare.c:700`
- `src/prepare.c:854`
- `src/sqlite.h.in:4637`
