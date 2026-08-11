# global-init-config-001 — Library init/shutdown lifecycle

Slice: `global-init-config` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Once-init of malloc/mutex/pcache/OS layers; shutdown ordering; auto-init from open paths.

## Entrypoints (citations)

- `sqlite3_initialize()` (api) — `src/main.c:360`, `src/main.c:389`, `src/sqlite.h.in:1689`

## Inputs / outputs / observables

- sqlite3_initialize idempotent SQLITE_OK; shutdown ordering requirements; auto-init from sqlite3_open/malloc paths

## Behaviour (as implemented)

- sqlite3_initialize (src/main.c:360) once-guards: mutex system → malloc system → pcache → OS layer (sqlite3_os_init); re-entrant calls cheap no-ops; sqlite3_shutdown (src/main.c:389) reverses, legal only when all connections closed

## Validation rules found in code

- Most sqlite3_config calls after initialize → SQLITE_MISUSE (mask exceptions src/main.c:453)

## Edge cases found in code

- SQLITE_OMIT_AUTOINIT builds require explicit initialize (gate documented)

## Dependencies

- malloc-subsystem
- mutex-subsystem

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:360`
- `src/main.c:389`
- `src/sqlite.h.in:1689`
