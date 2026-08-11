# rtree-001 — R-tree virtual table module

Slice: `rtree` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

rtree/rtree_i32 modules, 1-5 dimensions, node storage in shadow tables.

## Entrypoints (citations)

- `sqlite3RtreeInit()` (other) — `ext/rtree/rtree.c:4325`, `ext/rtree/rtree.c:3379`, `ext/rtree/rtree.c:4485`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE USING rtree(id, x0,x1, y0,y1,...); range-query results; shadow tables _node/_rowid/_parent; integrity via rtreecheck()

## Behaviour (as implemented)

- sqlite3RtreeInit (ext/rtree/rtree.c:4325) registers rtree + rtree_i32 modules (rtreeModule ext/rtree/rtree.c:3379); 1-5 dimensions stored as float32 (or int32) pairs; queries decompose constraints into R-tree traversal with guaranteed-containment semantics (float32 rounding expands boxes outward)

## Validation rules found in code

- Odd column counts / >5 dims → error at create; coordinates coerced to float32 (documented precision loss)

## Edge cases found in code

- Rounding: stored ranges may be slightly larger than inserted (never smaller) — queries can return false positives, never false negatives; caller re-filters

## Dependencies

- vtab-core

## Assumptions / unknowns

- Shadow-table format is the migration-sensitive part (run-1 flag)

## Evidence

- `ext/rtree/rtree.c:4325`
- `ext/rtree/rtree.c:3379`
- `ext/rtree/rtree.c:4485`
