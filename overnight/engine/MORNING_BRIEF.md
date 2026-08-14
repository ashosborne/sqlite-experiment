# MORNING BRIEF — engine v48: the first table-scan bytecode (run 59, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–58 stamped alongside
(run-58 brief archived at MORNING_BRIEF-2026-08-15-run58.md).
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v48-openread, IMPLEMENT_VDBE_OPENREAD,
REQUIRE_EXPLAIN_MATCHES_C, REQUIRE_COLUMN_FROM_BTREE_CELL, FORBID_STORE_ROWS_AS_COLUMN,
FORBID_INDEX_OPENREAD, FORBID_OPENWRITE. MAX_NEW_CASES 40 (used 10).

## 1. Pack @48 BOUND — OPENREAD/CURSOR law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v47 → **v48**
(versions/1–48 retained; ADR `0046-engine-v48-openread.md`; schema VALID; 54 laws).
The law: the FROM residual shrinks only if EXPLAIN matches probed C (incl. OpenRead's real
root page), step runs that program in the dispatch loop, AND OP_Column copies from the current
btree cell — never the kitchen store. Counters prove both sides.

## 2. C's program (probed, frozen)

`EXPLAIN SELECT a FROM t`: Init 0 7 · OpenRead 0 **2** 0 p4=1 · Rewind 0 6 · Column 0 0 1 ·
ResultRow 1 1 · Next 0 3 p5=1 · Halt · Transaction 0 0 **1** p4=0 p5=1 · Goto 0 1 — OpenRead p2
is the real root page, p4 the column-count hint (SELECT b keeps p4=2), Transaction p3 the schema
cookie. Same listing empty and after reopen; execution returns rowid-order rows, empty scan's
first step is DONE, reset replays, a later INSERT is visible. Stretch: after a 12-row overflow
split (page_count 4, interior root) the listing is unchanged and the scan still yields all rows;
`SELECT length(b)` is pinned as the kitchen boundary. 10 goldens under
`tests/characterization/engine-vdbe48/`, two-run deterministic, delegated HUMAN_ACCEPTED.

## 3. Modern: Column reads cells

`vdbe::parse_scan`/`compile_scan` emit C's program (root page + cookie read from the file);
`execute_with` drives the cursor: OpenRead opens on `pager::read_table_cells` output (0x0d leaf
or the v46 one-level 0x05 interior), Rewind/Next position (cursor-read counter), **Column
decodes the current cell's payload** via `dbfile::decode_record` into a register. The kitchen
store is never consulted: the cursor-read counter moves on the scan (incl. a runtime pid-derived
payload) and does not move on SELECT 1 (v47 path) or on a join (kitchen). Gate =
`store::vm_scan_ctx`: file-backed, autocommit, non-WAL, one plain rowid table, no IPK (C emits
Rowid there — unimplemented, named). `sqlite_master.rootpage` now answers from the file image.

## 4. Inventory

- **vdbe-engine-001 stays partial** — FROM residual rewritten: scan opcodes landed; WHERE
  compares, joins, select-list expressions, aggregates, index OpenRead/SeekGE, OpenWrite/DML
  codegen, IPK Rowid, ~185 opcodes, OP_Program/interrupt/progress remain kitchen.
- **btree-002 stays partial** — live-read residual rewritten: the v48 scan slice reads cells
  via the cursor; joins/WHERE/expressions/IPK/multi-table/WAL/in-txn reads still store.
- **vdbe-engine-002 stays none** — registers are a plain value Vec, not Mem cells.
- **prepare-statement-api-006 stays partial** — bytecode rows real for FROM programs too;
  nested-loop EQP still not bytecode (v37).
- Composed `engine-vdbe48-001/002/003` full. pager/wal/btree-001/select/expr untouched.

## 5. Scoreboard

**236 full / 45 partial / 74 none of 355.** All 61 test binaries green (vdbe47, btree46,
pager44, sqlite_sql_suite first slice included). SCRIPT_TABLE.len()==0.

## 6. Not migrated

SQLite is **not** migrated. One read shape rides the VM; everything else is kitchen. No WHERE,
no joins, no indexes, no DML bytecode, no Mem cells, no interrupt, ~185 opcodes absent.

## 7. Next call

**WHERE compares on that cursor** (Ne/Eq/Gt family + SeekRowid — would let `SELECT b FROM t
WHERE a=?` ride the VM), or **OpenWrite / Insert via VDBE** (DML bytecode on the v45 cursor),
or **leaf merge on DELETE** (shrink below the split; needs a freelist).
