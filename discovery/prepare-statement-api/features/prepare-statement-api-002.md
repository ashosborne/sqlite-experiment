# prepare-statement-api-002 — Step execution state machine

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_step drives the VDBE; SQLITE_ROW/DONE/BUSY/MISUSE contract.

## Entrypoints (citations)

- `sqlite3_step()` (api) — `src/vdbeapi.c:980`, `src/sqlite.h.in:5347`

## Inputs / outputs / observables

- Return codes SQLITE_ROW/SQLITE_DONE/SQLITE_BUSY/SQLITE_MISUSE/error codes; legacy (v1) statements report generic SQLITE_ERROR until reset

## Behaviour (as implemented)

- sqlite3_step (src/vdbeapi.c:980) wraps sqlite3Step with the auto-reprepare retry loop: on SQLITE_SCHEMA from a v2 statement it re-prepares (sqlite3Reprepare src/prepare.c:904) and re-binds, retrying up to SQLITE_MAX_SCHEMA_RETRY times
- First step starts the implicit transaction if none is open; DONE means the statement completed and must be reset before re-stepping

## Validation rules found in code

- Stepping a finalized statement → SQLITE_MISUSE
- Interrupt (sqlite3_interrupt) surfaces as SQLITE_INTERRUPT at the next opcode boundary

## Edge cases found in code

- SQLITE_BUSY on COMMIT is recoverable (statement stays live, retry step); busy during a nested write is not
- Expired statement after too many schema retries → SQLITE_SCHEMA even for v2

## Dependencies

- vdbe-engine

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeapi.c:980`
- `src/sqlite.h.in:5347`
