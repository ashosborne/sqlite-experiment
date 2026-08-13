# ADR 0043 — engine v45: a table b-tree on the v44 pager (btree-001/002 none→partial)

Status: accepted (run 56, pack v45)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

The second internal-engine pack, on top of the v44 rollback-journal pager. It
adds the btree handle's public transaction pin (`sqlite3_txn_state`) and a table
cursor that moves cells on the pager's leaf pages. No VDBE, no index trees, no
split/merge balancing. SQLite is NOT migrated.

## Probe (file-backed, journal_mode=DELETE)

`sqlite3_txn_state`: idle 0; a **deferred** `BEGIN` alone stays 0; a `SELECT`
lifts to read 1; a write / `BEGIN IMMEDIATE` / `BEGIN EXCLUSIVE` lifts to write
2; `COMMIT`/`ROLLBACK` return to 0. A write on a readonly-opened file fails rc 8.
Cursor observables: literal INSERT/DELETE/UPDATE persist and reopen by rowid;
`PRAGMA integrity_check` = ok.

## btree-001 — handle + txn_state

`sqlite3_txn_state(db, zSchema)` reports the connection's transaction level:
none 0 / read 1 / write 2, tracked as the store's txn opens, a read runs
(`stmt_query_typed` / a SELECT statement), or a write runs / `BEGIN
IMMEDIATE|EXCLUSIVE` starts. Pinned engine-btree45-001. Full forbidden (no
shared-cache locks, no schema-cookie out-param matrix).

## btree-002 — table cursor on pager pages

`pager.rs` gained a table cursor for the single-leaf plain-rowid scope:
- `schema_rootpage` walks page 1's sqlite_master leaf to find the table's
  physical rootpage.
- `read_leaf` parses a table-leaf page (0x0d) into `(rowid, payload)` cells
  (refuses interior 0x05 / overflow → fall back).
- `write_leaf` re-serialises the leaf page from rowid-sorted cells (cell-pointer
  array + content area) — a real page-level cell layout, not a whole-image
  rebuild.
- `rewrite_table_leaf` moves the current row set onto that one leaf page through
  cursor cell puts (one op per cell; removed rowids count as deletes), bumps the
  file change counter, and returns the new image (only the leaf page + page-1
  header differ). A `cursor_ops` counter proves the traffic.

The DELETE-mode flush (`cursor_or_whole_image`) routes a file-backed connection
with exactly one main plain-rowid table (no indexes/views/vtabs/triggers/
attached/temp) through the cursor; **everything else falls back** to the
whole-image writer (so no regressions, and those cases stay honest).

### Cursor vs dbfile-encode

The cursor edits the specific leaf page's cells on the existing file image — it
does NOT call `dbfile::write_db_bytes` (the whole-image encoder). The fallback
path still uses the encoder, but that path is not counted as cursor work and is
not what the btree-002 pins exercise.

### C integrity_check

**Passes.** A helper (`btree45_write_committed_file`, gated on `BTREE45_OUT`)
writes a db through the cursor (INSERT ×3, DELETE k=2, INSERT a runtime row);
the pinned C amalgamation opens it, reads `1,one|3,three|4,cursor-made-me`, and
`integrity_check` returns rc 0. The `:memory:` control moves neither the cursor
nor pcache file counters.

## Honest limits (what stayed / residual)

btree-002 partial residual: single-leaf only (no page split/merge balancing),
no index b-trees, no WITHOUT ROWID, no overflow-page cleanup, no saved-position
restore matrix; outside the single-leaf rowid scope the whole-image writer
handles the flush, and **live query reads still evaluate over the in-memory
store** (the write path + the reopen round-trip are what go through
cells-on-pages). pager-001 stays partial (unchanged). pager-002, pcache-001/002,
vdbe-*, where-* untouched. wal-001/002 not deepened.

## Estate outcome

| Card | Outcome |
|---|---|
| btree-001 | **none → partial** — handle + sqlite3_txn_state on the pager |
| btree-002 | **none → partial** — table cursor cells on pager leaf pages (single-leaf rowid scope) |
| pager-001 | partial (unchanged) |
| pager-002 / pcache-001/002 / vdbe / where | untouched |
| engine-btree45-001..002 | composed full |

Scoreboard: 227 full / 42 partial / 77 none of 346 → **229 full / 44 partial /
75 none of 348** (+2 composed; none −2). `SCRIPT_TABLE.len()==0`. pager44,
engine-txn, lookaside35, harvest43 and the sqlite_sql_suite first slice all green.
