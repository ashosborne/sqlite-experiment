# malloc-subsystem-001 — Public malloc API and memory accounting

Slice: `malloc-subsystem` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

malloc/realloc/free family with soft-heap-limit accounting; alternative backends mem0..mem5 compile-time selected.

## Entrypoints (citations)

- `sqlite3_malloc64()` (api) — `src/malloc.c:322`, `src/malloc.c:296`, `src/malloc.c:197`, `src/malloc.c:159`

## Inputs / outputs / observables

- memory_used/highwater; soft/hard heap limit effects (SQLITE_NOMEM vs releasing caches); malloc failure propagation as SQLITE_NOMEM

## Behaviour (as implemented)

- sqlite3Malloc (src/malloc.c:296) routes through configured methods with size accounting under STATUS mutex; public sqlite3_malloc64 (src/malloc.c:322); release_memory (src/malloc.c:23) frees reclaimable cache; hard heap limit turns allocations beyond budget into failures
- OOM sets db->mallocFailed → deferred SQLITE_NOMEM at API boundary (apiExit pattern)

## Validation rules found in code

- Allocation size >2GB-ish guarded (SQLITE_MAX_ALLOCATION_SIZE style caps)

## Edge cases found in code

- mem0..mem5 alternates (debug/system) selected at compile — census stays in this card

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/malloc.c:322`
- `src/malloc.c:296`
- `src/malloc.c:197`
- `src/malloc.c:159`
