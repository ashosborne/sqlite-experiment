# prepare-statement-api-004 — Column result access (typed, with coercions)

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

column_blob/bytes/double/int/int64/text accessors with implicit type conversion rules.

## Entrypoints (citations)

- `sqlite3_column_text()` (api) — `src/vdbeapi.c:1448`, `src/vdbeapi.c:1468`, `src/vdbeapi.c:1333`

## Inputs / outputs / observables

- Typed values returned per column; sqlite3_column_type before vs after a coercing accessor (type is UNSTABLE after conversion)
- column_bytes reflects the encoding requested

## Behaviour (as implemented)

- Accessors (src/vdbeapi.c:1448-1478) fetch Mem cells from the current row and apply the documented coercion table (e.g. column_int on a text value parses the prefix; column_text on a blob returns the bytes as text)
- column_count (src/vdbeapi.c:1333) is static per statement; column_type is per-row dynamic

## Validation rules found in code

- Out-of-range column index returns NULL/0 semantics (guarded, no crash) — misuse but tolerated

## Edge cases found in code

- Calling column_text then column_bytes16 (or vice versa) can invalidate prior pointers — documented pointer-invalidation rules
- Accessing columns when statement is not on a row (before first step / after DONE) yields NULLs/0 rather than error

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Coercion matrix is card-worthy at Test-gen granularity; this card pins the seam, matrix rows enumerable from the doc-comment table (src/vdbeapi.c:1419-1431)
- Coercion matrix is behaviour-rich; likely several cards at Phase B

## Evidence

- `src/vdbeapi.c:1448`
- `src/vdbeapi.c:1468`
- `src/vdbeapi.c:1333`
