# MORNING BRIEF — engine v16: CHECK on UPDATE (run 26)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–25 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v16-check-on-update — a focused
single-gap loop. MAX_NEW_CASES 24 (used 15). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @16 BOUND — CHECK-on-UPDATE law

versions/16.yaml + ADR 0014, schema-validated. All prior laws carried.

## 2. Semantics (pinned on C, now real)

CHECKs — column-level AND table-level `CHECK(a < b)` constraints (the latter were not
even captured before this run; they are now also enforced on INSERT) — are evaluated
against the **post-update row image** during statement planning:

| Mode | Pinned behaviour |
|---|---|
| plain / OR ABORT | statement-atomic: nothing applied — rows (1,9) stay (1,9) |
| OR FAIL | earlier row changes of the SAME statement kept — (1,9) → (6,9), rc 19 |
| OR IGNORE | violating row skipped, others updated |
| OR ROLLBACK | whole v15 transaction unwound — autocommit→1, COMMIT then errors |

NULL CHECK results pass (three-valued semantics, pinned). NOT NULL is validated on
the post-update image too (unpinned but C-correct direction, noted). File twins pin
durability: a passing UPDATE survives reopen; a failing one is absent after reopen.

## 3. Cases: 15 frozen / 0 deferred

engine-checkupd-001 (8 script: fail/pass, multi-column table CHECK pass+fail,
OR IGNORE mixed rows, NULL semantics, expression CHECK, in-txn commit) ·
engine-checkupd-002 (5 bespoke: row-unchanged proof, OR ABORT-in-txn keeps txn,
OR ROLLBACK kills it, OR FAIL prefix-keep vs ABORT statement-undo) ·
engine-checkupd-003 (2 file twins). Two-run deterministic, delegated HUMAN_ACCEPTED,
all replaying byte-identical.

## 4. dml-codegen-002 verdict: **FULL**

The card's conflict matrix now reads IGNORE / REPLACE / ABORT / FAIL / ROLLBACK with
CHECK / NOT NULL / UNIQUE / FK enforced on **INSERT and UPDATE**, OR ROLLBACK unwinding
real transactions. The CHECK-on-UPDATE gap — the only blocker named since run 20 — is
closed. Flipped to `impl_in_modern: full`, `status: converted`, `parity: UNVERIFIED`.

## 5. Anti-cheat + cargo

anti-cheat **24/24** (new: runtime bound + runtime SET pass/fail with OR IGNORE proof;
runtime OR ROLLBACK unwinding a txn insert). `cargo test` **377/377** (checkupd_compare
8 + engine_checkupd 7 new; all 21 prior suites unchanged). All 324 prior goldens
md5-identical. SCRIPT_TABLE still 0. (Run-26 job-3 commit message said 379 — the true
total is 377; corrected here and in the journal.)

## 6. Scoreboard before → after

| State | run 25 | **run 26** |
|---|---|---|
| full | 71 | **75** (+dml-codegen-002, +engine-checkupd-001/-002/-003) |
| partial | 52 | **51** |
| none | 103 | 103 |
| behaviours | 226 | 229 |

## 7. Explicit honesty line

**SQLite is NOT migrated.** 75 of 229 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified.

## 8. Next call

(a) ddl-schema-002 focused loop: index-driven lookups + multi-column explicit
indexes (the longest-standing named partial), (b) UTF-16 text/prepare APIs
(prepare-001's sole gap), or (c) sqlite3_value / UDF registration surface.
Pack v17 + goldens first.
