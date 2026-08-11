# json-funcs-003 — JSON validation and typing

Slice: `json-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

json_valid with flags argument distinguishes JSON5/JSONB acceptance.

## Entrypoints (citations)

- `jsonValidFunc()` (other) — `src/json.c:4700`

## Inputs / outputs / observables

- json_valid(X) 0/1; json_valid(X,flags) with bitmask (1=strict RFC-8259, 2=JSON5, 4=JSONB probe, 8=strict JSONB); json_type returns null/true/false/integer/real/text/array/object; json_error_position byte offset

## Behaviour (as implemented)

- jsonValidFunc (src/json.c:4700) validates per flags argument; default flags accept canonical JSON only

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- NULL input → NULL (not 0)
- json_type with path arg returns type at path or NULL if absent

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/json.c:4700`
