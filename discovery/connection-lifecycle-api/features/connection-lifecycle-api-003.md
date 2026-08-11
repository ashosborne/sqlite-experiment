# connection-lifecycle-api-003 — Busy handler / busy timeout

Slice: `connection-lifecycle-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-connection lock-contention callback; busy_timeout installs default sleeping handler.

## Entrypoints (citations)

- `sqlite3_busy_timeout()` (api) — `src/main.c:1816`, `src/main.c:1873`

## Inputs / outputs / observables

- Blocking behaviour of statements hitting SQLITE_BUSY; busy_timeout total sleep duration; handler invocation count argument

## Behaviour (as implemented)

- sqlite3_busy_handler (src/main.c:1816) installs per-connection callback invoked with retry count when a lock cannot be obtained; returning 0 → SQLITE_BUSY propagates, non-zero → retry
- sqlite3_busy_timeout (src/main.c:1873) installs the built-in sqliteDefaultBusyCallback with exponential-ish delay table up to the ms budget; ms<=0 clears the handler

## Validation rules found in code

- Setting a busy handler clears any previous one (and vice versa with timeout)

## Edge cases found in code

- Busy handler is NOT invoked for deadlock-certain cases (e.g. upgrading a shared lock while another writer waits) — returns SQLITE_BUSY immediately
- In shared-cache mode SQLITE_LOCKED (not BUSY) paths bypass this handler (see unlock-notify-api-001)

## Dependencies

- btree
- pager

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:1816`
- `src/main.c:1873`
