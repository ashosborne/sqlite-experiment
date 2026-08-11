# MORNING BRIEF — sqlite-experiment run 7: Test execution RECORD (six prepare-statement cases)

Run: 2026-08-11 · `2026-08-11T1310Z-legacy-record-six` · from `44ac80fba` · committed as `sqlite-testexec-record-six`
MODE=RECORD · TARGET=legacy only. (Run-5 brief preserved as `MORNING_BRIEF-2026-08-11-run5.md`.)

## 1. Build + fingerprint vs run 5

Run-5 out-of-tree build **reused** (`/tmp/sqlite-build`); `src/`/`ext/` verified zero-diff since the
run-5 build base, and the fingerprint re-verified live: **sqlite 3.54.0, ENABLE_API_ARMOR=0,
OMIT_AUTORESET=0 — exact match to the BASELINE pin. No rebuild, no second pin, BASELINE dump untouched.**
Harness compiled fresh against that amalgamation (public C API only; no path steps a finalized handle).

## 2. Six goldens + replay

| Case | RECORD actuals | Replay |
| --- | --- | --- |
| prepare-statement-api-001-C001 | prepare=0, stmt non-NULL, pzTail consumed | **REPLAY_GREEN** |
| prepare-statement-api-001-C002 | whitespace/comment-only SQL → OK + NULL stmt | **REPLAY_GREEN** |
| prepare-statement-api-003-C001 | bind=0 → ROW(100) → column=7 | **REPLAY_GREEN** |
| prepare-statement-api-003-C002 | bind index OOR (same stmt after reset) → **25 (SQLITE_RANGE)** | **REPLAY_GREEN** |
| prepare-statement-api-005-C001 | explicit reset-from-DONE: reset=0, re-step ROW, **bound 42 preserved** | **REPLAY_GREEN** |
| prepare-statement-api-005-C002 | finalize live stmt → 0 (contract = finalize.rc only; setup rcs frozen as extras, not promoted) | **REPLAY_GREEN** |

All replays byte-matched. Goldens: `tests/characterization/prepare-statement-api/cases/<FEATURE_ID>/C00n.approved.txt`;
run SoT: `runs/2026-08-11T1310Z-legacy-record-six/{results.json,REPORT.md,actuals,logs}`.
The four HUMAN_ACCEPTED goldens verified **byte-identical** (md5) — untouched, unrenamed.

## 3. Hygiene (done first, words not goldens)

- 002-C002 TRACE title: dropped "(card: must reset before re-stepping)" → autoreset wording.
- 002-C003 TRACE title + CASE-003.md: retitled **"BLOCKED — UAF, do not RECORD"**; the old "try RECORD if API_ARMOR" advice removed (armor guards NULL, not freed handles); step(NULL) noted as a future C004, not this ID.
- error-status-001-C002 TRACE title: dropped "MISUSE-safe" → SQLITE_NOMEM / 'out of memory' wording.
- 003-C002 spec now states the bind is on the same stmt after `sqlite3_reset` (matches harness).
- 001-C002 spec observable renamed `pzTail.rest` (matches harness).
- 005-C001 spec notes it is explicit **reset-from-DONE** on the autoreset pin (distinct from 002-C002).

## 4. Flags kept honest

- **legacy_green NOT flipped** on 001/003/005 — still exactly the two stamped IDs. These six goldens are `PENDING_HUMAN` (recorded in TRACEABILITY beside the run-6 stamp, which covers only the 002 pair).
- C003 still BLOCKED. Nothing captured for it.

## 5. What this run did NOT do

No re-RECORD of the four stamped cases, no COMPARE/modern, no Conversion, no PACK.yaml, no Discovery
seeding, no `test/` TCL/testfixture/sqllogictest, no `src/`/`ext/` edits, no generated files committed.

## 6. completeness: incomplete

Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes).

## 7. Closing ask

**Approve these six goldens?** On stamp, the whole prepare-statement spine (001/002/003/005) plus
error-status-001 would be characterization-pinned — ready for the next batch decision (error-status-002/003,
exec-convenience, connection-lifecycle) or the first Architecture PACK conversation. Still not Conversion-ready:
stamp ≠ PACK ≠ Convert, and 002-C003 stays blocked.
