# ADR 0024 — engine v26: connection lifecycle (close, busy, hooks, trace)

Status: accepted (pack v26 BOUND, Ash Osborne, delegated autonomy run 36)

## Context (plain language)

A connection should refuse to close while statements are still alive; close_v2
should say OK and quietly wait until the last statement is finalized; a busy
handler should get called (and can sleep/retry) when another connection holds the
write lock; and applications should get callbacks when rows change, when
transactions commit, and when statements run. All three lifecycle cards were none.

## What modern now does

1. **close / close_v2.** Live prepared statements AND open blob handles are tracked
   per connection. `sqlite3_close` refuses with rc 5 and C's exact message while any
   remain (reset does not unblock — pinned); finalize/blob_close unblock. `close_v2`
   returns OK immediately and zombies: the statement stays usable (pinned read after
   close_v2), and the REAL teardown runs when the last handle goes. NULL closes are
   no-ops. Closing with an open transaction rolls back (pinned via file reopen).
2. **Busy handling over a real (in-process) write lock.** BEGIN IMMEDIATE takes a
   per-file lock; a second connection's write consults its busy handler with an
   increasing retry count (pinned 1-call and 3-call shapes), or sleeps under
   busy_timeout, then fails rc 5 "database is locked". Handler and timeout replace
   each other (pinned: handler never fires after timeout installed). COMMIT releases
   the lock and the blocked connection succeeds afterwards (pinned).
3. **Cross-connection visibility.** To make the busy pins honest, committed state
   now flushes to the file at COMMIT (C's durability point) and sibling connections
   on the same path reload it before their next statement (only when they have no
   local writes and no open transaction). **Residual: this is a single-process
   model — C's cross-process file locking and page cache are NOT claimed.**
4. **Hooks.** `commit_hook` fires once per committed transaction — autocommit
   writes included (pinned count 3 = 2 autocommits + 1 explicit) — and a non-zero
   return turns the commit into a rollback (rc 19 "constraint failed", autocommit
   restored, data unchanged; autocommit aborts use a pre-statement snapshot).
   `update_hook` reports (op 18/23/9, "main", table, rowid) with the rowid aliased
   to the INTEGER PRIMARY KEY (pinned log). Both return the previous user argument
   on replacement and stop firing when unset (pinned).
5. **trace_v2.** STMT sees the statement text; ROW fires per delivered row; CLOSE
   fires once at teardown; PROFILE fires per completed statement (count pinned);
   mask 0 unsets. Residual: STMT/PROFILE fire per exec call, not per prepared
   statement inside multi-statement scripts.

## Deliberate residuals (not claimed)

Cross-process locking / shared cache / unlock-notify; busy handlers on non-write
paths; deferred (non-IMMEDIATE) lock acquisition timing beyond pins; unfinished
sqlite3_backup close coupling; WITHOUT ROWID update_hook suppression; TRUNCATE
fast-path delete hook suppression; legacy sqlite3_trace/profile; per-prepared-
statement trace granularity; savepoint RELEASE commit flush.

## Consequences

22 goldens replay byte-identical; anti-cheat (live-stmt close cycle, pid-seeded
update_hook log, runtime commit_hook abort) green; cargo 589/589.
connection-lifecycle-api-002 → **partial** (backup coupling + MISUSE matrix
residual), -003 → **partial** (in-process lock model only), -004 → **partial**
(trace granularity + WITHOUT ROWID/truncate residuals). Stretch skipped.
