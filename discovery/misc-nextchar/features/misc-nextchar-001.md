# misc-nextchar-001 — next_char() incremental completion function

Slice: `misc-nextchar` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Returns the set of next characters extending a prefix within an indexed column

## Entrypoints (citations)

- `sqlite3_nextchar_init()` (other) — `ext/misc/nextchar.c:296`

## Inputs / outputs / observables

- next_char(W, 'table.column' or subquery spec[, collation...]) → string of possible next characters extending prefix W

## Behaviour (as implemented)

- init ext/misc/nextchar.c:296: probes an indexed column with range queries per candidate continuation, aggregating distinct next characters

## Validation rules found in code

- Needs an index on the target column for sane cost (documented)

## Edge cases found in code

- Multi-byte UTF-8 continuations returned whole

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/nextchar.c:296`
