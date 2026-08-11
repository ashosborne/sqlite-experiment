# pager-002 — Journal-mode state machine

Slice: `pager` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Six journal modes; transition legality (e.g. WAL exit requires exclusive access).

## Entrypoints (citations)

- `sqlite3PagerSetJournalMode()` (other) — `src/pager.c:7415`

## Inputs / outputs / observables

- PRAGMA journal_mode results (echoes resulting mode); file suffixes -journal vs -wal/-shm; mode-transition failures

## Behaviour (as implemented)

- sqlite3PagerSetJournalMode (src/pager.c:7415) implements the 6-mode machine (DELETE/TRUNCATE/PERSIST/MEMORY/WAL/OFF); entering WAL requires no other connections mid-transaction; leaving WAL requires exclusive access; MEMORY/OFF trade durability for speed (rollback impossible in OFF)

## Validation rules found in code

- journal_mode on :memory: dbs limited to MEMORY/OFF
- WAL persists in the db header (survives reopen)

## Edge cases found in code

- PERSIST leaves journal bytes zeroed-header; TRUNCATE keeps the file at 0 bytes — different fs behaviour profiles

## Dependencies

- wal

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/pager.c:7415`
