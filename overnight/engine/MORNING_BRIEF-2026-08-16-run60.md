# MORNING BRIEF — engine v49: WHERE compares on the cell cursor (run 60, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–59 stamped alongside
(run-59 brief archived at MORNING_BRIEF-2026-08-15-run59.md).
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v49-where, IMPLEMENT_VDBE_WHERE_EQ,
REQUIRE_COMPARE_IN_DISPATCH, FORBID_KITCHEN_FILTER_AS_WHERE, FORBID_INDEX_SEEK,
FORBID_OPENWRITE. MAX_NEW_CASES 40 (used 9).

## 1. Pack @49 BOUND — WHERE/CURSOR law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v48 → **v49**
(versions/1–49 retained; ADR `0047-engine-v49-where.md`; schema VALID; 55 laws).
The law: the WHERE residual shrinks only if EXPLAIN matches probed C, step runs the program,
the compare is AN OPCODE (never a Rust filter), and Column still decodes cells. The cursor
must position on rejected rows too.

## 2. C's program (probed, frozen)

C **inverts** the WHERE test into a jump-to-Next compare: `=`→**Ne**, `<>`→Eq, `>`→Le,
`<`→Ge, `>=`→Lt, `<=`→Gt — all `p4=BINARY-8 p5=84`, the literal loaded into r2 by an
init-section `Integer` (or `Variable 1 2` for `?`). Column a→r1, compare `Ne 2 7 1`, result
Column b→r3, `ResultRow 3 1`. `WHERE rowid=2` is a different shape: `Integer`+`SeekRowid`
touching a single cell, no Rewind/Next loop. Execution: matching rows in rowid order, no-match
first step DONE, the whole family runs, bound `?` returns the bound match, a later INSERT
becomes visible. Nine goldens under `tests/characterization/engine-vdbe49/`, two-run
deterministic, delegated HUMAN_ACCEPTED.

## 3. Modern: the compare is an opcode

`vdbe::parse_where_scan` + `compile_where_scan`/`compile_seek_rowid` emit C's layout;
`execute_bound` dispatches **Eq/Ne/Lt/Le/Gt/Ge** (compare r[p3] with r[p1], jump on hold or
on NULL via the 0x10 bit, SQLite's NULL < numbers < text < blob order), **Variable** (1-based
params) and **SeekRowid** (cursor positions on the sought cell). No `cells.iter().filter`
anywhere — the cursor-read counter proves the scan positions on rejected rows (≥ cell count),
and a runtime payload behind a decoy row round-trips through Column + Ne. The prepare dry-run
falls back to the VM compiler for the rowid-seek shape only (the kitchen has no rowid binding
and never runs it). Kitchen boundaries hold: joins and `length(b)` move neither counter; the
v48 unfiltered scan still rides the VM.

## 4. Inventory

- **vdbe-engine-001 stays partial** — WHERE residual rewritten: compare family + Variable +
  SeekRowid landed; AND/OR, LIKE/IS NULL/BETWEEN, text compares, joins, expressions,
  aggregates, index seeks, OpenWrite/DML codegen, IPK Rowid, ~180 opcodes remain kitchen.
- **btree-002 stays partial** — the v49 WHERE/seek shapes read cells through the cursor;
  everything else still store.
- **vdbe-engine-002 stays none** — compares run on plain values; BINARY-8's affinity byte is
  replicated in the listing, not implemented as a Mem lattice.
- **prepare-statement-api-006 stays partial** — bytecode rows real for WHERE programs too.
- Composed `engine-vdbe49-001/002` full. pager/wal/btree-001/select/expr untouched.

## 5. Scoreboard

**238 full / 45 partial / 74 none of 357.** All 62 test binaries green (vdbe48, vdbe47,
btree46, pager44, sqlite_sql_suite first slice included). SCRIPT_TABLE.len()==0.

## 6. Not migrated

SQLite is **not** migrated. One WHERE family rides the VM on one narrow scan scope. No AND/OR,
no indexes, no DML bytecode, no Mem cells, ~180 opcodes absent; SeekRowid is a cell-list
position, not a page descent.

## 7. Next call

**OpenWrite / Insert via VDBE** (DML bytecode on the v45 cursor — the write mirror of v48/49),
or **AND/OR on the same cursor** (two compares chained as C emits), or **leaf merge on DELETE**
(shrink below the split; needs a freelist).
