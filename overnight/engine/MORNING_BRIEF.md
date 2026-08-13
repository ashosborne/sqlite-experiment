# MORNING BRIEF — engine v45: a table b-tree on the pager (run 56, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–55 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v45-btree, IMPLEMENT_BTREE_ON_PAGER,
REQUIRE_TABLE_CURSOR_ON_PAGER_PAGES, FORBID_DBFILE_WHOLE_IMAGE_AS_BTREE, DEEPEN_VDBE/WAL false.
MAX_NEW_CASES 40 (used 4).

## 1. Pack @45 BOUND — BTREE/NONE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v44 → **v45**
(versions/1–45 retained; ADR `0043-engine-v45-btree.md`; schema VALID; 51 laws).

## 2. btree-001 — handle + txn_state

`sqlite3_txn_state` on the pager-backed handle matches probed C: idle 0, a **deferred BEGIN
alone** 0, a SELECT lifts to **read 1**, a write / BEGIN IMMEDIATE / BEGIN EXCLUSIVE lifts to
**write 2**, COMMIT/ROLLBACK return to 0; a readonly-file write fails rc 8. none → **partial**.

## 3. Cursor vs dbfile-encode

`pager.rs` gained a real **table cursor**: `schema_rootpage` walks page 1 to the table's
rootpage; `read_leaf` parses a 0x0d leaf's cells; `write_leaf` re-serialises the leaf's cell
array + content area. File-backed INSERT/DELETE/literal-UPDATE of a single-leaf plain rowid
table move cells on **that leaf page** (change-counter bumped), NOT via the whole-image encoder.
A `cursor_ops` counter moves on the file DML and **not** on a `:memory:` control. Anything
outside the single-leaf rowid scope falls back to the whole-image writer (no regressions).

## 4. C integrity_check — YES

A helper (`btree45_write_committed_file`, gated on `BTREE45_OUT`) writes a db through the cursor;
the **pinned C amalgamation** opens it, reads `1,one|3,three|4,cursor-made-me` (k=2 deleted,
runtime row inserted), and `PRAGMA integrity_check` returns rc 0.

## 5. btree-002 — YES (partial)

none → **partial**: cells move through the cursor on pager leaf pages for the single-leaf rowid
scope. RESIDUAL: single-leaf only (no split/merge balancing), no index b-trees, no WITHOUT ROWID,
no overflow cleanup, no saved-position restore; outside that scope the whole-image writer handles
the flush and **live query reads still evaluate over the in-memory store** (the write path +
reopen round-trip are what go through cells-on-pages).

## 6. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 227 | **229** |
| partial | 42 | **44** |
| none | 77 | **75** |
| behaviours | 346 | 348 (+2 composed) |

none→partial: **btree-001, btree-002**. Composed engine-btree45-001/002 full. None dropped by 2.

## 7. Freezes / cargo

4 goldens under `tests/characterization/engine-btree45/` (001 txn_state ×2, 002 cursor ×2),
two-run deterministic, delegated HUMAN_ACCEPTED, legacy_green 257. `cargo test` (57 binaries):
**all green**, including pager44, engine-txn, lookaside35, harvest43 and the sqlite_sql_suite
first slice. `SCRIPT_TABLE.len()==0`. Prior goldens untouched.

## 8. Not migrated

SQLite is **not migrated**. No VDBE, no index b-trees, no page split/merge, no WITHOUT ROWID,
no overflow cleanup, no planner; live query evaluation is still the in-memory kitchen, not a
btree-cursor read loop.

## 9. Next call

Leaf split (enough inserts to overflow one leaf, C integrity_check still ok — still not
btree-002 full, the balance-siblings matrix remains), OR the VDBE first slice (a real opcode
loop feeding the cursor) as the next `none` cut.
