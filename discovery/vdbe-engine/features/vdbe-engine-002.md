# vdbe-engine-002 — Mem-cell value semantics (affinity/encoding conversions)

Slice: `vdbe-engine` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Dynamic typing engine: numeric/text/blob conversions, encodings, zeroblob materialization.

## Entrypoints (citations)

- `src/vdbemem.c` (other) — `src/vdbemem.c`

## Inputs / outputs / observables

- typeof() results; CAST results; text<->numeric coercions; encoding conversions UTF-8/16LE/16BE; comparison ordering across types (NULL < numeric < text < blob)

## Behaviour (as implemented)

- Mem cells (src/vdbemem.c) hold dynamic-typed values with flag lattice (Null/Int/Real/Str/Blob + Term/Static/Dyn); applyAffinity attempts lossless numeric conversion of text; sqlite3VdbeMemStringify renders numerics; cross-type comparison follows the fixed type ordering with collation only within text

## Validation rules found in code

- Numeric text must parse fully (trailing junk → stays text under NUMERIC affinity rules)

## Edge cases found in code

- 8-byte integer vs REAL comparison uses exact integer-vs-double logic (no precision loss)
- Zeroblob materializes lazily; text with embedded NULs length-tracked not NUL-terminated

## Dependencies

- utf-util

## Assumptions / unknowns

- Evidence is file-scope (src/vdbemem.c) with behaviour cross-checked from public API docs — confidence kept observed-in-code for the seam, matrix rows to be pinned by Test gen at SQL level
- Characterize via SQL-level typeof()/CAST corpora rather than internals

## Evidence

- `src/vdbemem.c`
