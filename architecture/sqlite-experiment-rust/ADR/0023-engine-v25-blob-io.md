# ADR 0023 — engine v25: incremental blob I/O

Status: accepted (pack v25 BOUND, Ash Osborne, delegated autonomy run 35)

## Context (plain language)

The blob handle API lets an application open a handle on ONE cell of ONE row and
read or write bytes at an offset — without pulling the whole value through SQL.
The handle is live: if the row underneath it changes, the handle dies and further
reads/writes fail with "query aborted". Until this run both blob cards were none.

## What modern now does

1. **Open** on (schema, table, column, rowid) with C's validation order and exact
   error text: `cannot open view: v` → `no such table: main.nope` →
   `no such column: "zz"` → `cannot open indexed column for writing` (read-write
   only; read-only opens on indexed columns are allowed) → `no such rowid: 99` →
   `cannot open value of type null`. rowid aliases the INTEGER PRIMARY KEY.
   Text cells open fine and read their bytes (pinned "texty").
2. **bytes / read / write.** bytes reports the live length (0 after expiry, pinned).
   Reads and writes at an offset; out-of-range or negative → rc 1 "SQL logic error"
   with the caller's buffer untouched (no partial transfer, pinned). Writes never
   resize; a read-only handle refuses writes with rc 8. zeroblob(N) preallocation
   then interior writes is the pinned sizing tool.
3. **reopen** repositions to another rowid; missing rowid errors and aborts the
   handle like C.
4. **Expiry.** Modifying the handle's row (UPDATE or DELETE, same connection) kills
   it: rc 4 "query aborted", bytes → 0 (pinned both ways, plus a runtime anti-cheat).
5. **Durable + interop.** Handle writes persist across reopen (integrity ok); the
   pinned C CLI reads a Rust file whose bytes were written only through a handle.
   WAL-mode files work without deepening the WAL claim.

## Implementation notes

Handles carry a connection-write marker (total_changes, schema_version) captured at
open/reopen; any DML on the connection expires the handle. **Residual (documented):
C expires per-row — ours is connection-write granular; every pinned case modifies
the handle's own row, so the pins cannot tell the difference, but the card notes it.**
Handle writes deliberately do NOT bump change counters (a handle must not expire
itself — pinned by write-then-bytes). One engine hole opened: UPDATE ... SET with a
constant expression (`zeroblob(4)`) now evaluates instead of failing literal parse.

## Deliberate residuals (not claimed)

Per-row expiry granularity; attached-schema / UTF-16 names; blob_open interactions
with explicit transactions beyond pins; writing into TEXT cells via handles;
sqlite3_blob_open on WITHOUT ROWID tables. Stretch cards skipped — blob alone
was the run.

## Consequences

22 goldens replay byte-identical; anti-cheat + C interop green; cargo 565/565.
blob-io-api-001 flips none→**partial** (lifecycle real; attached-schema/UTF-16
name forms and txn interactions unpinned), blob-io-api-002 none→**partial**
(I/O + bounds + expiry real for pins; per-row expiry granularity residual).
