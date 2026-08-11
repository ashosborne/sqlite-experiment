# MORNING BRIEF — sqlite-experiment run 6: golden stamp + card truth + next spine batch

Run: 2026-08-11 · from `b989b9dd` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-stamp-and-next-spine`
(Run-4 brief preserved as `overnight/testgen/MORNING_BRIEF-2026-08-11-run4.md`.)

## 1. Stamp (Job 1a)

- **HUMAN_ACCEPTED** (Ash Osborne, 2026-08-11 Europe/London) written to all four TRACEABILITY files
  (testgen ×2, tests/characterization ×2) for: error-status-api-001-C001/C002, prepare-statement-api-002-C001/C002 — all stay REPLAY_GREEN.
- Goldens verified **byte-identical** before/after (md5 diff clean). No renames.
- **C003 still BLOCKED** (UAF; golden_path null). No `sqlite3_step(NULL)` under this ID — that would be a new C004, out of this run.

## 2. C001 wording class (Job 1b)

- `errmsg.text` tagged **wording_deferred / shape-only** for any future COMPARE; recorded line kept in the golden.
- Contract observables: the integers — `prepare.rc=1`, `errcode.value=1`, `extended_errcode.value=1`.
- C002's `out of memory` (sqlite3ErrStr / SQLITE_NOMEM) left as a **contract string** (Terry).

## 3. Card patches (Job 1c) — quoted

- error-status-api-001, Validation:
  - was: `Calling on NULL db → 'out of memory'/MISUSE-safe static answers (guarded)`
  - now: `Calling on NULL db returns errcode=7 (SQLITE_NOMEM) and errmsg 'out of memory' (static guarded answers via sqlite3ErrStr). This is not MISUSE.`
- prepare-statement-api-002, Behaviour:
  - was: `DONE means the statement completed and must be reset before re-stepping`
  - now: `... In this pinned build (OMIT_AUTORESET=off) a further sqlite3_step after DONE auto-resets and returns SQLITE_ROW (100) — recorded as golden C002. The "must call sqlite3_reset before re-stepping" rule is the manual/OMIT_AUTORESET=on contract only.`
- prepare-statement-api-002, Validation:
  - was: `Stepping a finalized statement → SQLITE_MISUSE`
  - now: `Stepping a finalized statement: NOT observed and not safely observable — it is use-after-free even with SQLITE_ENABLE_API_ARMOR ... Case C003 is permanently BLOCKED; no MISUSE claim is made from observation.`
- Discovery notes appended to both cards citing run `2026-08-11T1205Z-legacy-record` + the BASELINE fingerprint.
- legacy_green: exactly the two stamped IDs (now legitimate); parity_green false everywhere; prepare-statement-api-002 notes: **not conversion-ready while C003 is BLOCKED (stamp ≠ PACK ≠ Convert)**. COVERAGE regenerated. No other card flipped; no PACK authored.

## 4. New cases written (Job 2) — TO_BE_RECORDED, matrix approved by this paste

| Case ID | Spec |
| --- | --- |
| prepare-statement-api-001-C001 (prepare valid stmt: rc, stmt non-NULL, pzTail consumed) | testgen/prepare-statement-api/scenarios/prepare-statement-api-001/CASE-001.md |
| prepare-statement-api-001-C002 (whitespace/comment-only SQL) | .../prepare-statement-api-001/CASE-002.md |
| prepare-statement-api-003-C001 (bind_int → ROW → column) | .../prepare-statement-api-003/CASE-001.md |
| prepare-statement-api-003-C002 (bind index out of range) | .../prepare-statement-api-003/CASE-002.md |
| prepare-statement-api-005-C001 (reset preserves bindings) | .../prepare-statement-api-005/CASE-001.md |
| prepare-statement-api-005-C002 (finalize live stmt — finalize.rc only; handle never touched after) | .../prepare-statement-api-005/CASE-002.md |

Harness: `testgen/prepare-statement-api/harness/prepare_bind_reset_harness.c` — public C API only,
**compile-checked** against the pinned run-5 amalgamation, **not executed** (execution = RECORD, forbidden this run).
It never steps/reads a finalized handle. TRACEABILITY rows appended; the 002 rows and goldens untouched.

## 5. Deferred / blocked

- Operator DEFERs recorded per feature in DEFERRED.md (UTF-16 twins, prepFlags, busy/TOOBIG/destructors, clear_bindings-vs-reset, reprepare matrix, finalize-in-txn, stmt_status).
- Still blocked: prepare-statement-api-002-C003 (UAF). No new blocks this run.

## 6. What this run did NOT do

No RECORD of the new cases (no new `*.approved.*`), no COMPARE, no Conversion, no PACK, no Discovery
seeding, no other CARD_IDS, no product-source edits. Frozen goldens byte-identical.

## 7. completeness: incomplete

Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes).

## 8. Closing ask

**Run Test execution RECORD on the six new TO_BE_RECORDED cases?** The pinned build recipe and
fingerprint are already in BASELINE.md; the new harness is compile-checked and capture-ready.
