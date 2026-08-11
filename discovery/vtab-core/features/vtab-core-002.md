# vtab-core-002 — declare_vtab and vtab_config negotiation

Slice: `vtab-core` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Column-shape declaration from inside xCreate/xConnect; constraint support flags.

## Entrypoints (citations)

- `sqlite3_declare_vtab()` (api) — `src/vtab.c:815`, `src/vtab.c:1339`

## Inputs / outputs / observables

- declare_vtab schema shape (column names/types/HIDDEN); vtab_config effects (CONSTRAINT_SUPPORT, INNOCUOUS, DIRECTONLY, USES_ALL_SCHEMAS)

## Behaviour (as implemented)

- sqlite3_declare_vtab (src/vtab.c:815) parses a CREATE TABLE statement to fix the vtab's column shape (HIDDEN columns become constraint-only); sqlite3_vtab_config (src/vtab.c:1339) called from xCreate/xConnect negotiates ON CONFLICT support and safety attributes

## Validation rules found in code

- declare_vtab only legal inside xCreate/xConnect (else MISUSE)
- Constraint support off → REPLACE etc. handled by core via delete+insert

## Edge cases found in code

- HIDDEN columns receive constraint values via xBestIndex/xFilter argv, not rows

## Dependencies

- vtab-core-001

## Assumptions / unknowns

- xBestIndex cost contract = deep Phase B topic; carded at seam level, Test gen to pin per-module
- xBestIndex cost contract is a deep Phase B topic

## Evidence

- `src/vtab.c:815`
- `src/vtab.c:1339`
