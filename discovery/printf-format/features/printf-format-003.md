# printf-format-003 — sqlite3_str string-builder API

Slice: `printf-format` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Dynamic string accumulator with error-state and precision/width caps.

## Entrypoints (citations)

- `sqlite3_str_new()` (api) — `src/printf.c:1447`, `src/printf.c:192`

## Inputs / outputs / observables

- sqlite3_str_* accumulator API: append/appendf/reset/finish/errcode/length/value

## Behaviour (as implemented)

- sqlite3_str_new (src/printf.c:1447) allocates an accumulator bound to a db's limits; errors are sticky (SQLITE_NOMEM/TOOBIG) and reported at finish

## Validation rules found in code

- Exceeding SQLITE_LIMIT_LENGTH → sticky SQLITE_TOOBIG, output discarded

## Edge cases found in code

- str_finish on error state returns NULL but still frees

## Dependencies

- malloc-subsystem

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/printf.c:1447`
- `src/printf.c:192`
