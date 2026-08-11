# misc-uuid-001 — uuid generation/conversion functions

Slice: `misc-uuid` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

RFC-4122 v4 generation + text/blob conversions

## Entrypoints (citations)

- `sqlite3_uuid_init()` (other) — `ext/misc/uuid.c:213`

## Inputs / outputs / observables

- uuid() → v4 text; uuid_str(X) canonical text form; uuid_blob(X) 16-byte blob; accepts text/blob inputs in either direction

## Behaviour (as implemented)

- init ext/misc/uuid.c:213: RFC-4122 v4 from sqlite3_randomness; converters validate + normalize case/hyphens

## Validation rules found in code

- Invalid uuid text/blob → NULL

## Edge cases found in code

- Accepts braces/urn: prefixes on parse (liberal input)

## Dependencies

- loadext-api
- util-primitives

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/uuid.c:213`
