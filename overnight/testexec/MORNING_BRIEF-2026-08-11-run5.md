# MORNING BRIEF — sqlite-experiment run 5: Test execution RECORD (legacy)

Run: 2026-08-11 · `2026-08-11T1205Z-legacy-record` · from `23b6a1ed1` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-testexec-record`
MODE=RECORD · TARGET=legacy only. No COMPARE, no modern, no Conversion.

## 1. Build + fingerprint

- Pinned baseline built out-of-tree (`/tmp/sqlite-build`; generated amalgamation/binaries **not** committed): `/workspace/configure && make sqlite3 sqlite3.c`, default flags. `sqlite_version()` = **3.54.0**.
- Fingerprint **captured** and appended to `overnight/BASELINE.md` (closes the run-4 "not captured" line):
  - **ENABLE_API_ARMOR = off** (as expected) · **OMIT_AUTORESET = off** (as expected)
  - Notable bare-configure defaults recorded: FTS3/FTS4, RTREE, MATH_FUNCTIONS, PERCENTILE, STMTVTAB, DBSTAT/DBPAGE/BYTECODE vtabs enabled; **DQS=0**. These refine the compile-options census cards.

## 2. Goldens + replay (the four approved cases only)

| Case | RECORD result | Immediate replay |
| --- | --- | --- |
| error-status-api-001-C001 | errcode=1, extended=1, errmsg=`near "SELECTT": syntax error` | **REPLAY_GREEN** (byte-match) |
| error-status-api-001-C002 | errcode(NULL)=7 (SQLITE_NOMEM), errmsg(NULL)=`out of memory` | **REPLAY_GREEN** |
| prepare-statement-api-002-C001 | prepare=0 → step=100 (ROW) → col=1 → step=101 (DONE) | **REPLAY_GREEN** |
| prepare-statement-api-002-C002 | third step after DONE = **100 (ROW)** — autoreset, not MISUSE (charter predicted this) | **REPLAY_GREEN** |

Goldens: `tests/characterization/<slice>/cases/.../C00n.approved.txt` · run SoT: `runs/2026-08-11T1205Z-legacy-record/{results.json,REPORT.md,actuals,logs}` · scrub profile: none (deterministic static inputs). `legacy_green` set **only** on `error-status-api-001` and `prepare-statement-api-002`; the other 183 behaviours untouched.

## 3. C003 BLOCKED

`prepare-statement-api-002-C003` (step a finalized statement) — **BLOCKED, not captured**: use-after-free even with API_ARMOR (armor guards NULL, not freed handles). The capture call was **removed from the harness**; TRACEABILITY (both testgen and tests/characterization) carries the reason. No UB frozen. Optional future replacement (`sqlite3_step(NULL)`) deliberately NOT done this run.

## 4. Flags for humans

- **errmsg wording**: frozen verbatim in the C001 golden but not a pass/fail contract — needs a human look at golden approval.
- **Card refinement suggested** (Discovery note, no golden rewrite): prepare-statement-api-002's "must be reset before re-stepping" sentence describes the manual contract; the pinned build auto-resets (OMIT_AUTORESET off) and returns ROW.

## 5. What this run did NOT do

No COMPARE / modern runs, no Conversion, no PACK, no Discovery seeding, no `test/` TCL / testfixture / sqllogictest, no product-source edits (verified zero diffs), no goldens for any other feature ID.

## 6. completeness: incomplete

Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes). 2 of 185 behaviours now legacy-green-flagged pending golden approval.

## 7. Closing ask

**Approve these four goldens?** (`tests/characterization/*/cases/**.approved.txt` + run reports.) Approval unlocks the next stage decisions (more RECORD batches, or Architecture PACK authoring toward Conversion). C003 stays blocked until you choose the `sqlite3_step(NULL)` replacement case or drop it.
