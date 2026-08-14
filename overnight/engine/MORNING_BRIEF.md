# MORNING BRIEF — engine v50: INSERT bytecode (run 61, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–60 stamped alongside
(run-60 brief archived at MORNING_BRIEF-2026-08-16-run60.md).
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v50-openwrite, IMPLEMENT_VDBE_OPENWRITE,
REQUIRE_INSERT_VIA_CURSOR, FORBID_KITCHEN_INSERT_AS_VDBE, FORBID_DBFILE_FALLBACK_AS_INSERT,
FORBID_INDEX_OPENWRITE. MAX_NEW_CASES 40 (used 8).

## 1. Pack @50 BOUND — OPENWRITE/CURSOR law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v49 → **v50**
(versions/1–50 retained; ADR `0048-engine-v50-openwrite.md`; schema VALID; 56 laws).
The law: the DML residual shrinks only if EXPLAIN matches probed C, step dispatches the program,
OP_Insert puts a cell through the btree cursor (never the store, never write_db_bytes), and a
later cell scan reads the new row including a runtime payload.

## 2. C's program (probed, frozen)

`EXPLAIN INSERT INTO t(a,b) VALUES(1,'one')`: Init 0 8 · OpenWrite 0 **2** 0 p4=2 · Integer→r2 ·
String8→r3 · NewRowid→r1 · MakeRecord 2 2 4 **p4=DB** (the affinity string) · Insert 0 4 1
**p4=t p5=57** · Halt · Transaction 0 **1** 1 (a WRITE txn) · Goto. Column-list-less is
identical; `?,?` swaps in Variable loads. Execution: DONE + last_insert_rowid, rowid order,
changes/total_changes, reset+step next rowids (3, 4), bound insert, and VDBE rows read back by
the v48 scan / v49 WHERE / rowid seek. Eight goldens under
`tests/characterization/engine-vdbe50/`, two-run deterministic, delegated HUMAN_ACCEPTED.

## 3. Modern: Insert writes cells

`vdbe::parse_insert`/`compile_insert` emit C's layout (root + cookie + affinity string derived
from the declared types); `execute_dml` dispatches OpenWrite/NewRowid(max+1)/MakeRecord
(`encode_record`)/Insert (pending cell + insert counter). **`pager::insert_cell` puts the
MakeRecord payload verbatim** through the v46 leaf packing (cursor-op counter ticks,
split-capable), and `store::vm_persist_insert` commits via the journal mini-txn, then mirrors
the row into the store (bookkeeping — `next_rowid` keeps the kitchen's last-assigned semantics).
Anti-cheat: dispatch + OP_Insert + cursor-write counters move on the VM INSERT; none move on a
join, an INSERT SELECT, or `length(b)`; a v49 WHERE read ticks no write counter; a runtime
payload survives close+reopen and comes back through the v48 cell scan.
`sqlite3_total_changes`/`64` added (store already tracked it; the golden pins it).

## 4. Inventory

- **vdbe-engine-001 stays partial** — DML residual rewritten: single-row INSERT landed;
  UPDATE/DELETE bytecode, INSERT SELECT, multi-row/upsert, constraint tables, hooked/authorized
  connections, the sqlite3_exec path, affinity APPLICATION, IPK NewRowid remain kitchen.
- **btree-002 stays partial** — the VDBE Insert now drives the cursor write; kitchen DML still
  flushes through cursor_or_whole_image.
- **vdbe-engine-002 stays none** — MakeRecord's p4 is replicated, not applied.
- **prepare-statement-api-006 stays partial** — bytecode rows real for INSERT programs too.
- Composed `engine-vdbe50-001/002` full. pager/wal/btree-001/select/expr untouched.

## 5. Scoreboard

**240 full / 45 partial / 74 none of 359.** All 63 test binaries green (vdbe49, vdbe48, vdbe47,
btree46, pager44, sqlite_sql_suite first slice included). SCRIPT_TABLE.len()==0.

## 6. Not migrated

SQLite is **not** migrated. One INSERT shape rides the VM; UPDATE/DELETE/constraints/hooks and
~175 opcodes are absent; affinity is a string in a listing, not a Mem lattice.

## 7. Next call

**UPDATE/DELETE bytecode** (the remaining DML mirror — Delete/IdxDelete on the same cursor), or
**AND/OR on the read cursor** (chained compares as C emits), or **leaf merge on DELETE** (shrink
below the split; needs a freelist).
