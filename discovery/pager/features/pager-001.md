# pager-001 — Transaction lifecycle and two-phase commit

Slice: `pager` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Begin/commit-phase-one/two/rollback with rollback-journal crash safety.

## Entrypoints (citations)

- `sqlite3PagerBegin()` (other) — `src/pager.c:5976`, `src/pager.c:6519`, `src/pager.c:6822`, `src/pager.c:4789`

## Inputs / outputs / observables

- ACID effects observable via crash-free API level: BEGIN/COMMIT/ROLLBACK; journal files appearing/disappearing; SQLITE_BUSY on lock conflicts

## Behaviour (as implemented)

- sqlite3PagerBegin (src/pager.c:5976) acquires RESERVED→EXCLUSIVE as needed; CommitPhaseOne (src/pager.c:6519) flushes pages + syncs journal (or WAL frames); PhaseTwo finalizes; Rollback (src/pager.c:6822) replays the rollback journal; PagerGet (src/pager.c:5780) fetches pages through the cache with journalling-before-write discipline

## Validation rules found in code

- Hot-journal detection at open replays uncommitted transactions from crashes
- synchronous pragma levels change sync points (OFF/NORMAL/FULL/EXTRA)

## Edge cases found in code

- Statement journals for partial-statement rollback (nested savepoint machinery)
- In-memory and temp dbs skip journalling per pager flags

## Dependencies

- vfs-os-abstraction
- pcache

## Assumptions / unknowns

- Crash-safety characterization needs fault-injection VFS — upstream harness out of scope; recorded blind spot stands (run-1/2)
- Crash-safety characterization needs fault-injection VFS — note for test design

## Evidence

- `src/pager.c:5976`
- `src/pager.c:6519`
- `src/pager.c:6822`
- `src/pager.c:4789`
