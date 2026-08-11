# json-funcs-001 — JSON path extraction (json_extract, ->, ->>)

Slice: `json-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Path-expression extraction; text vs SQL-value return differs between -> and ->>.

## Entrypoints (citations)

- `jsonExtractFunc()` (other) — `src/json.c:4071`, `src/json.c:5660`

## Inputs / outputs / observables

- json_extract returns SQL values for terminal scalars, JSON text for objects/arrays; -> always JSON text; ->> always SQL value/text

## Behaviour (as implemented)

- jsonExtractFunc (src/json.c:4071) parses path expressions ($.a.b[N], $."quoted"); single-path extract unwraps, multi-path returns JSON array; -> / ->> operators map to 2-arg extract variants with distinct result modes

## Validation rules found in code

- Malformed JSON → 'malformed JSON' error; malformed path → error with path text

## Edge cases found in code

- JSON5 accepted on input (unquoted keys, trailing commas, hex numbers) since json.c rewrite; output always canonical JSON
- jsonb_extract returns JSONB blobs for structured results

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/json.c:4071`
- `src/json.c:5660`
