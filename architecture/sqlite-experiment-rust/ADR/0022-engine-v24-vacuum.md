# ADR 0022 — engine v24: VACUUM and VACUUM INTO

Status: accepted (pack v24 BOUND, Ash Osborne, delegated autonomy run 34)

## Context (plain language)

VACUUM rebuilds a database: it copies everything into fresh, tightly-packed storage
and throws away the wasted space left behind by deletes. `VACUUM INTO 'file'` does the
same rebuild but writes the result into a brand-new file, leaving the original alone.
Until this run, modern had neither — vacuum-001 had a C golden from the recognizer era
but was honestly marked "not implemented" (legacy_green ≠ done), and vacuum-002 had
nothing.

## What modern now does

1. **A real rebuild.** After deletes, plain rowids get renumbered 1,2,3… (pinned:
   1,3,5 → 1,2,3), exactly like C. Tables with an INTEGER PRIMARY KEY or WITHOUT ROWID
   keep their keys (pinned). Skipping the rebuild would fail these pins — that is the
   anti-cheat.
2. **Free space is reclaimed.** `PRAGMA page_count` now models the freelist: it grows
   with data and does NOT shrink when rows are deleted — only VACUUM brings it back
   down (pinned as before>after plus a small absolute bound).
3. **Files are rewritten immediately** and stay C-readable (`rust_vacuum_c_read`:
   the pinned C CLI reads a Rust file after delete+VACUUM, integrity ok). In WAL mode
   the -wal is rewritten too, so stale pre-rebuild frames cannot resurrect old rowids;
   journal_mode stays wal (pinned).
4. **You cannot VACUUM inside a transaction** — C's exact error, and the transaction
   keeps working afterwards (pinned).
5. **VACUUM INTO** writes a fresh database at the target: existing target → C's
   "output file already exists"; unwritable path → rc 14 "unable to open database:";
   inside a transaction → same txn error; a `:memory:` source exports to a file
   (all pinned). The runtime anti-cheat has C read an INTO target whose path and
   contents are chosen at run time.

## Engine holes the pins forced open (all real fixes)

- DELETE with an arbitrary WHERE (`n > 5`, `v % 2 = 0`, `IN (...)`) — evaluated
  per row by the expression engine, replacing the col=int-only filter.
- INSERT VALUES with constant expressions (`zeroblob(1000)`) — computed, not parsed
  as a literal. This is what lets the old vacuum-001-C001 golden finally replay.
- `SELECT rowid ...` projections and ORDER BY rowid — routed to the row store (the
  evaluator's rows carry no rowids), with rowid aliasing the INTEGER PRIMARY KEY.
- `sqlite_master` type/name projections with multi-key ORDER BY.

## Deliberate residuals (not claimed)

Pending `page_size` / `auto_vacuum` application during VACUUM; attached-schema
forms (`VACUUM schema`); URI filename targets for INTO; RBU vacuum. Stretch cards
(get_table, status counters, close_v2) were skipped — VACUUM alone was the run.

## Consequences

18 new goldens replay byte-identical; 2 C-interop round trips + runtime anti-cheat
green; cargo 541/541. vacuum-001 flips none→**partial** (rebuild real; page_size/
auto_vacuum apply + attached forms remain), vacuum-002 none→**partial** (INTO real;
URI target form remains — it is named in the card, so full would over-claim).
