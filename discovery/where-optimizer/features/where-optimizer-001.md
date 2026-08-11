# where-optimizer-001 — WHERE-loop generation boundary

Slice: `where-optimizer` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

WhereBegin/WhereEnd bracket loop codegen for SELECT/UPDATE/DELETE.

## Entrypoints (citations)

- `sqlite3WhereBegin()` (other) — `src/where.c:6826`, `src/where.c:7529`

## Inputs / outputs / observables

- Loop structure bracketed by WhereBegin/WhereEnd; EQP rows describing scan/search choices

## Behaviour (as implemented)

- sqlite3WhereBegin (src/where.c:6826) analyzes WHERE terms (whereexpr.c), builds WhereLoops, runs the solver, emits loop-opening code; sqlite3WhereEnd (src/where.c:7529) closes loops and resolves deferred seeks
- Handles: index range scans, equality probes, IN-operator loops, OR-clause multi-index, skip-scan, covering-index-only scans, LEFT JOIN ordering constraints

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- ORDER BY elimination when an index delivers the requested order; LIMIT-aware min/max optimization

## Dependencies

- btree
- expr-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/where.c:6826`
- `src/where.c:7529`
