# misc-closure-001 — transitive_closure vtab

Slice: `misc-closure` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Transitive closure over parent-pointer tables with idcolumn/parentcolumn args

## Entrypoints (citations)

- `sqlite3_closure_init()` (other) — `ext/misc/closure.c:985`

## Inputs / outputs / observables

- SELECT id FROM closure WHERE root=? AND tablename=? ... — transitive closure rows with depth; idcolumn/parentcolumn params

## Behaviour (as implemented)

- init ext/misc/closure.c:985: breadth-first walk over a parent-pointer table using an internal queue + hash de-dup; depth limit param

## Validation rules found in code

- Requires usable index on idcolumn for sane performance (documented)

## Edge cases found in code

- Cycles terminated by visited-set de-dup

## Dependencies

- vtab-core

## Assumptions / unknowns

- Recursive-CTE-supersession question stands
- Superseded by recursive CTEs downstream?

## Evidence

- `ext/misc/closure.c:985`
