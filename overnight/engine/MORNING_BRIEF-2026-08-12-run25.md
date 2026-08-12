# MORNING BRIEF — engine v15: transactions (run 25)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–24 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v15-transactions. MAX_NEW_CASES 40 (used 27).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @15 BOUND — transaction / savepoint / OR ROLLBACK laws

versions/15.yaml + ADR 0013, schema-validated. WAL stays forbidden; no locking claims.

## 2. How undo works — plainly

**Snapshot model, not a journal.** BEGIN (and every SAVEPOINT) captures a full snapshot
of the store's logical state (tables, rows, catalog, views, triggers, indexes, FK flag).
ROLLBACK / ROLLBACK TO restore a snapshot — ROLLBACK TO keeps the named savepoint alive,
exactly like C; COMMIT / RELEASE drop undo state; releasing the outermost implicit
savepoint commits. total_changes is deliberately NOT rolled back (matches C). This is
not a pager journal, not WAL, and makes no crash-safety or multi-connection claims.
File connections persist the committed logical state through the shared v12 writer;
closing with an open transaction auto-rolls-back — pinned against C.

## 3. Cases: 27 frozen / 0 deferred (+ file reopen results)

engine-txn-001 (8: commit/rollback/nested-begin error/no-txn errors/multi-stmt
atomicity/DDL-in-txn rollback/DEFERRED+IMMEDIATE) · engine-txn-002 (OR IGNORE in txn) ·
engine-savepoint-001 (8: partial undo, release-commits, nesting both directions,
continue-after-rollback-to, unknown-name + released-name errors) ·
engine-orrollback-001 (6, bespoke: OR ROLLBACK kills the txn — autocommit→1, COMMIT
errors, rows gone; OR ABORT keeps it; FK orphan + explicit ROLLBACK;
**DELETE OR ROLLBACK is a C syntax error** — pinned, not invented; get_autocommit) ·
engine-txnfile-001 (4: reopen after COMMIT sees rows; after ROLLBACK doesn't; open txn
at close auto-rolls-back; committed marker 777001 survives). All two-run deterministic,
delegated HUMAN_ACCEPTED, all replaying byte-identical through exec + statement API.

## 4. dml-codegen-002 verdict

**Stays partial — honestly.** OR ROLLBACK is now real (the last conflict-mode hole),
so the matrix reads IGNORE/REPLACE/ABORT/FAIL/ROLLBACK + CHECK/NOT NULL/UNIQUE/FK on
insert — but **CHECK-on-UPDATE is still absent**, and that keeps the card partial.
Also aligned: triggers-002's RAISE(ROLLBACK) now unwinds the real transaction (the
run-24 note caveat is resolved rather than papered over).

## 5. Anti-cheat + cargo

anti-cheat **22/22** (new: runtime commit-visible/rollback-gone keys; savepoint
partial undo with runtime values; OR ROLLBACK flips autocommit and voids COMMIT;
file reopen commit+uncommitted-close). `cargo test` **360/360** (txn_compare 17,
engine_txn 10, all 19 prior suites unchanged green). All 297 prior goldens
md5-identical. SCRIPT_TABLE still 0.

## 6. Scoreboard before → after

| State | run 24 | **run 25** |
|---|---|---|
| full | 66 | **71** (+engine-txn-001/-002, savepoint, orrollback, txnfile) |
| partial | 52 | 52 (dml-002 + triggers-002 notes updated) |
| none | 103 | 103 |
| behaviours | 221 | 226 |

## 7. Explicit honesty line

**SQLite is NOT migrated.** 71 of 226 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified. No WAL, no crash recovery, no
concurrent-connection locking, no VDBE, no planner.

## 8. Next call

(a) CHECK-on-UPDATE + UPDATE OR-conflict clauses (finishes dml-codegen-002),
(b) UTF-16 text/prepare APIs (prepare-001 + column UTF-16), or (c) index-driven
lookups + multi-column explicit indexes (ddl-schema-002 focused loop).
Pack v16 + goldens first.
