# prepare-statement-api-002 — Step execution state machine

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_step drives the VDBE; return codes as implemented on this pin: ROW/DONE (autoreset after DONE); BUSY only where evidenced. Finalized-handle is UAF / not observed — MISUSE is not a recorded contract.

## Entrypoints (citations)

- `sqlite3_step()` (api) — `src/vdbeapi.c:980`, `src/sqlite.h.in:5347`

## Inputs / outputs / observables

- Return codes as implemented on this pin: SQLITE_ROW/SQLITE_DONE (autoreset after DONE); SQLITE_BUSY only where evidenced; legacy (v1) statements report generic SQLITE_ERROR until reset. Finalized-handle behaviour is UAF / not observed (C003 BLOCKED)

## Behaviour (as implemented)

- sqlite3_step (src/vdbeapi.c:980) wraps sqlite3Step with the auto-reprepare retry loop: on SQLITE_SCHEMA from a v2 statement it re-prepares (sqlite3Reprepare src/prepare.c:904) and re-binds, retrying up to SQLITE_MAX_SCHEMA_RETRY times
- First step starts the implicit transaction if none is open; DONE means the statement completed. In this pinned build (OMIT_AUTORESET=off) a further sqlite3_step after DONE auto-resets and returns SQLITE_ROW (100) — recorded as golden C002. The "must call sqlite3_reset before re-stepping" rule is the manual/OMIT_AUTORESET=on contract only. [Patched run 6 — was: "DONE means the statement completed and must be reset before re-stepping"]

## Validation rules found in code

- Stepping a finalized statement: NOT observed and not safely observable — it is use-after-free even with SQLITE_ENABLE_API_ARMOR (armor guards NULL pointers, not freed handles). Characterization case C003 is permanently BLOCKED; no MISUSE claim is made from observation. [Patched run 6 — was: "Stepping a finalized statement → SQLITE_MISUSE"]
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

## Discovery note (run 6)

- Pinned by legacy RECORD run `2026-08-11T1205Z-legacy-record` on the baseline fingerprint in `overnight/BASELINE.md` (OMIT_AUTORESET=off confirmed). Goldens C001/C002 human-accepted 2026-08-11 (Ash Osborne). C003 (step-after-finalize) permanently BLOCKED as UAF — this ID is NOT conversion-ready while C003 is BLOCKED (stamp ≠ PACK ≠ Convert).
