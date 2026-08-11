# printf-format-001 — SQL printf()/format() functions

Slice: `printf-format` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Formatter with SQL-argument marshalling (SQLITE_PRINTF_SQLFUNC); %q/%Q/%w quoting directives.

## Entrypoints (citations)

- `printfFunc()` (other) — `src/func.c:3369`, `src/func.c:315`, `src/printf.c:244`

## Inputs / outputs / observables

- printf()/format() SQL results incl. SQLite extensions %q (quote-escape '), %Q (quote or NULL), %w (double-quote identifier escape), %z ignored in SQL mode

## Behaviour (as implemented)

- printfFunc (src/func.c:315) marshals SQL args via SQLITE_PRINTF_SQLFUNC mode (src/printf.c:244) — %s of NULL yields empty string, %d of text applies numeric coercion

## Validation rules found in code

- Too-few args → missing conversions render as empty/zero (no error)
- Width/precision capped by SQLITE_PRINTF_PRECISION_LIMIT when defined (src/printf.c:192)

## Edge cases found in code

- %q/%Q are the injection-safety directives (run-1 flag): %q doubles single quotes; %Q also wraps in quotes and renders SQL NULL

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/func.c:3369`
- `src/func.c:315`
- `src/printf.c:244`
