# ADR 0013 — engine v15: transactions (pack v15)

Date: 2026-08-12 · Status: BOUND (supersedes pack v14; v1–v14 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 25)

## Context
No transactions meant no OR ROLLBACK, no savepoints, no honest multi-statement
atomicity — dml-codegen-002's biggest remaining gap and a missing spine for every
app-realistic write path.

## Decision — how undo works (plainly)
**Snapshot model, not a journal.** BEGIN (and each SAVEPOINT) captures a full snapshot
of the store's logical state — tables, rows, catalog, views, triggers, indexes, fk
flag. ROLLBACK/ROLLBACK TO restore a snapshot (ROLLBACK TO keeps the named savepoint,
discarding inner ones); COMMIT/RELEASE discard undo state (releasing the outermost
implicit savepoint commits). Counters (total_changes) are not rolled back, matching C.
This is explicitly NOT a pager journal and NOT WAL — no crash-safety or
multi-connection locking claims. For file connections the committed logical state is
what the shared v12 writer persists at close; an open transaction at close
auto-rolls-back exactly as C does (pinned).

## Semantics pinned on C (27 cases, two-run gate, delegated stamp)
BEGIN/COMMIT/ROLLBACK happy paths; nested BEGIN error; COMMIT/ROLLBACK with no txn
errors; multi-statement atomicity; DDL inside txn rolled back (table vanishes from
sqlite_schema); DEFERRED/IMMEDIATE accepted; savepoint partial undo, release-commits,
nesting, rollback-to-then-continue, unknown-savepoint errors, release-then-rollback-to
error; OR ROLLBACK kills the txn (autocommit→1, COMMIT then errors, rows gone) vs
OR ABORT which keeps it; FK orphan + explicit ROLLBACK; OR ROLLBACK in autocommit;
**DELETE OR ROLLBACK is a syntax error** (C grammar: DELETE has no conflict clause —
pinned rather than invented); sqlite3_get_autocommit across BEGIN/COMMIT; file reopen
after COMMIT / ROLLBACK / uncommitted-close. RAISE(ROLLBACK) now unwinds the real txn.

## Consequences
cargo 360/360; anti-cheat 22/22 (runtime commit/rollback keys, savepoint partial undo,
OR ROLLBACK autocommit flip, file reopen). Honestly still out: crash recovery, WAL,
concurrent connections/locking, BEGIN IMMEDIATE/EXCLUSIVE lock semantics (accepted,
no observable locking), CHECK-on-UPDATE. NOT migrated.
