# ADR 0042 — engine v44: a real rollback-journal pager (pager-001 none→partial)

Status: accepted (run 55, pack v44)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

The first internal-engine pack. Before this run, `dbfile.rs` rewrote the whole
image on save and ROLLBACK was pure in-memory store undo — not a pager. This
run puts a real rollback journal on the file-backed DELETE-mode write path. No
VDBE, no btree cursors, no WAL. SQLite is NOT migrated.

## Probe (file-backed, journal_mode=DELETE)

C creates `<db>-journal` next to the db during an open write transaction and
removes it on COMMIT and on ROLLBACK. ROLLBACK restores the pre-images (UPDATE,
DELETE, multi-row); COMMIT persists; a reopen (and the pinned C amalgamation)
reads the committed rows. `sqlite3_status`/`sqlite3_db_status` cache counters
exist but are machine-state magnitudes (not frozen). `sqlite3_txn_state`
reports 0 idle / 2 write.

## Implementation (modern/src/pager.rs)

- **txn_write** (open explicit write txn): read the txn-start db bytes as the
  rollback pre-image; for each page whose bytes changed, copy the ORIGINAL page
  into `<db>-journal` (accumulating across writes in the txn); then overwrite
  the changed pages in the db file. The journal is left in place → observably
  present for the life of the txn.
- **ROLLBACK**: `pager::rollback` reads the journal and writes each original
  page back into the db file (truncating any rolled-back page growth), then
  deletes the journal. The store also unwinds (engine-txn), so file and store
  agree.
- **COMMIT**: the pages were already written during the txn; `commit_drop_journal`
  removes the journal (data durable).
- **autocommit write**: `commit_over` runs a journal-then-write-then-delete
  mini-transaction (net: no journal remains), like C's per-statement journal.
- The committed db file is exactly the `dbfile::write_db_bytes` image — valid
  SQLite the pinned C reader opens (`SELECT` returns the runtime row,
  `PRAGMA integrity_check` = ok).

Every page get/write is fetched through a **methods2-shaped page cache**
(`Pcache2`): `xFetch` pins+counts, `xUnpin` releases. The counters
(fetch/unpin/write) move under real file-txn traffic and do NOT move under a
`:memory:` control that has no pager file.

## Is the committed file C-readable?

**Yes.** A helper test (`pager44_write_committed_file`, gated on `PAGER44_OUT`)
writes a committed db through the modern pager; the pinned C amalgamation opens
it, reads `1,one|2,two|3,three|4,pager-made-me`, and `integrity_check` returns
rc 0.

## Is the journal C-readable? (hot-journal)

**No — deliberately.** The journal is a Rust-private format (magic `RJRNL01`,
page-size, base-length, page records). The charter permits a private journal so
long as ROLLBACK really replays it and the committed *db file* is C-readable.
Hot-journal crash recovery by C is therefore NOT pinned (stretch skipped).

## pcache-001: why it stays none

pcache-001's surface is the **pluggable `sqlite3_config(SQLITE_CONFIG_PCACHE2)`
seam** — installing an external `sqlite3_pcache_methods2`. The pager genuinely
routes real page traffic through a methods2-shaped default cache (proven by the
`pager44_pcache_coupling` anti-cheat: counters move on the file txn, not on a
`:memory:` control), but modern does NOT implement the external
`sqlite3_config` registration boundary the card names. Under-claim: the coupling
is real and pinned as a composed anti-cheat, but pcache-001 **stays none** and
pcache-002 (LRU/pressure) stays none.

## Estate outcome

| Card | Outcome |
|---|---|
| pager-001 | **none → partial** — real rollback journal (A+B pinned). Full forbidden: no two-phase commit, no hot-journal C-recovery, no WAL-as-pager |
| pcache-001 | stays none — real coupling, but the pluggable sqlite3_config(PCACHE2) seam is not implemented |
| pager-002 | stays none — six journal modes (only DELETE journalled here) |
| pcache-002 | stays none — LRU/pressure |
| btree/vdbe/where | untouched |
| engine-pager44-001..002 | composed full |

Scoreboard: 225 full / 41 partial / 78 none of 344 → **227 full / 42 partial /
77 none of 346** (+2 composed; none −1). `SCRIPT_TABLE.len()==0`. engine-txn,
lookaside35, harvest43 TEMP/JSON and the sqlite_sql_suite first slice all green.

Note: the run-55 C002 golden was re-recorded with a literal `UPDATE t SET b='cc'`
(the original `'C'||a` conflated a kitchen expression-UPDATE gap — out of scope —
into a pager pin); the pin now cleanly tests multi-row commit persistence.
