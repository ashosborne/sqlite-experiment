# pragma-surface-002 — pragma_* eponymous virtual tables

Slice: `pragma-surface` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Result-returning pragmas exposed as pragma_<name>() table-valued functions.

## Entrypoints (citations)

- `pragmaVtabModule` (other) — `src/pragma.c:2814`, `src/pragma.c:3059`, `src/pragma.c:3092`

## Inputs / outputs / observables

- SELECT * FROM pragma_table_info('t') etc. — result-returning pragmas as TVFs with WHERE/JOIN support

## Behaviour (as implemented)

- pragmaVtabConnect/Module (src/pragma.c:2814,3059) wrap any result-flagged pragma; sqlite3PragmaVtabRegister (src/pragma.c:3092) resolves pragma_<name> at CREATE-less eponymous lookup; pragma args become hidden columns

## Validation rules found in code

- Only pragmas flagged Result0/Result1 get vtab projection

## Edge cases found in code

- Schema qualifier passed via second hidden column (e.g. pragma_table_info('t','main'))

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/pragma.c:2814`
- `src/pragma.c:3059`
- `src/pragma.c:3092`
