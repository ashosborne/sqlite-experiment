# mutex-subsystem-001 — Pluggable mutex abstraction

Slice: `mutex-subsystem` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Method-table plugin (config-replaceable); static vs dynamic mutexes; pthread/win/noop backends.

## Entrypoints (citations)

- `sqlite3_mutex_alloc()` (other) — `src/mutex.c:290`, `src/mutex.c:228`, `src/mutex_unix.c:393`

## Inputs / outputs / observables

- Threading modes: single/multi/serialized (compile+start-time+per-connection); mutex counters in debug builds

## Behaviour (as implemented)

- sqlite3MutexInit (src/mutex.c:228) installs compile-selected implementation (pthreads table src/mutex_unix.c:393, noop for single-thread); sqlite3_mutex_alloc (src/mutex.c:290) hands out static (never freed) vs dynamic (recursive) mutexes; config-replaceable via SQLITE_CONFIG_MUTEX

## Validation rules found in code

- Static mutex ids fixed; alloc of unknown id asserts

## Edge cases found in code

- SQLITE_MUTEX_OMIT builds compile all of it away — single-thread only

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Platform-retained; defer recommendation stands (run-1)
- Threading mode of downstream consumers?

## Evidence

- `src/mutex.c:290`
- `src/mutex.c:228`
- `src/mutex_unix.c:393`
