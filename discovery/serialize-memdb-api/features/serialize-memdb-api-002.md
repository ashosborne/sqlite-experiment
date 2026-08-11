# serialize-memdb-api-002 — memdb in-memory VFS

Slice: `serialize-memdb-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Registered VFS 'memdb' provides byte-array file semantics incl. shared named images.

## Entrypoints (citations)

- `memdb_vfs` (other) — `src/memdb.c:137`, `src/memdb.c:542`

## Inputs / outputs / observables

- VFS name 'memdb' registered; file:/name?vfs=memdb URIs share one image per name within a process

## Behaviour (as implemented)

- memdb_vfs (src/memdb.c:137) implements byte-array files (memdbOpen src/memdb.c:542); named images are refcounted process-global; unnamed are per-connection

## Validation rules found in code

- Journal files are no-ops (in-memory rollback via image copy semantics)

## Edge cases found in code

- Shared named image concurrency uses its own mutex, not file locks

## Dependencies

- vfs-os-abstraction

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/memdb.c:137`
- `src/memdb.c:542`
