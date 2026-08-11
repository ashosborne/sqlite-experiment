# pcache-002 — Default pcache1 (LRU, purgeable pages, memory pressure)

Slice: `pcache` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Default cache with group LRU, soft/hard limits, release-memory hooks.

## Entrypoints (citations)

- `pcache1Create()` (other) — `src/pcache1.c:764`, `src/pcache1.c:1197`, `src/pcache1.c:1198`

## Inputs / outputs / observables

- PRAGMA cache_size effects; memory usage under SQLITE_CONFIG_PAGECACHE; sqlite3_release_memory reclaiming

## Behaviour (as implemented)

- pcache1 (pcache1Create src/pcache1.c:764; installed by sqlite3PCacheSetDefault src/pcache1.c:1197): group-LRU across caches, page recycling, optional preallocated page pool, memory-pressure eviction hooks

## Validation rules found in code

- cache_size negative → KB-based sizing; positive → page count

## Edge cases found in code

- Under soft-heap-limit pressure unpinned pages are reclaimed across ALL connections in the group

## Dependencies

- malloc-subsystem

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/pcache1.c:764`
- `src/pcache1.c:1197`
- `src/pcache1.c:1198`
