# malloc-subsystem-002 — Lookaside per-connection allocator

Slice: `malloc-subsystem` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Two-size lookaside slots; db_config knobs; OOM fallback to general allocator.

## Entrypoints (citations)

- `lookaside (src/malloc.c)` (other) — `src/malloc.c:330`, `src/malloc.c:348`

## Inputs / outputs / observables

- DBSTATUS_LOOKASIDE_USED/HIT/MISS counters; db_config lookaside resizing results

## Behaviour (as implemented)

- Lookaside (src/malloc.c:330-348 predicates) serves small per-connection allocations from preallocated two-size slot pools (regular + mini); falls back to general allocator when exhausted; disabled while statements active

## Validation rules found in code

- db_config(LOOKASIDE) only when no allocations outstanding → SQLITE_BUSY otherwise

## Edge cases found in code

- Lookaside pointers never realloc'd — code paths distinguish lookaside-vs-heap on free

## Dependencies

- malloc-subsystem-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/malloc.c:330`
- `src/malloc.c:348`
