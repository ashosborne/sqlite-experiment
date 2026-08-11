# tokenizer-002 — sqlite3_complete statement detection

Slice: `tokenizer` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Public API state machine deciding whether SQL text ends in a complete statement (trigger-aware).

## Entrypoints (citations)

- `sqlite3_complete()` (api) — `src/complete.c:340`

## Inputs / outputs / observables

- sqlite3_complete 0/1 per SQL text; trigger-aware (CREATE TRIGGER ... END; counts as one statement)

## Behaviour (as implemented)

- sqlite3_complete (src/complete.c:340) runs a token-level state machine requiring a terminal semicolon outside any TRIGGER body / EXPLAIN prefix

## Validation rules found in code

- Comments and whitespace after the semicolon still → complete

## Edge cases found in code

- Statements ending inside a string/comment → incomplete; empty string → 0

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/complete.c:340`
