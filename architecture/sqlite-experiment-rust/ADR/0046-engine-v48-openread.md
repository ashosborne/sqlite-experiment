# ADR 0046 — engine v48: the first table-scan bytecode (OpenRead/Rewind/Column/Next)

- Status: BOUND (pack v48, supersedes v47; v1–v47 retained at `versions/`)
- Operator: Ash Osborne (full autonomy, delegated stamps)
- Date: run 59

## Decision

Put the v47 dispatch loop on top of the v45 btree cursor for the one read shape
both already own: `SELECT col(s) FROM t` on a file-backed plain rowid table.
Open a read cursor on the table root, move to the first row, copy a column into
a register, advance until done. Constant SELECT stays on the v47 path; joins,
WHERE, expressions, aggregates and DML stay on the kitchen evaluator, honestly
named.

## Which SQL, and C's exact program (probed on the pin)

`EXPLAIN SELECT a FROM t` (single-CREATE file db, root page 2, cookie 1):

```
0 Init        0 7 0
1 OpenRead    0 2 0  p4=1      -- p2 = the REAL root page; p4 = column-count hint
2 Rewind      0 6 0            -- p2 = Halt when the table is empty
3 Column      0 0 1            -- cursor 0, column 0 -> r1
4 ResultRow   1 1 0
5 Next        0 3 0  p5=1      -- loop back to the first Column
6 Halt        0 0 0
7 Transaction 0 0 1  p4=0 p5=1 -- p3 = the schema cookie from the file header
8 Goto        0 1 0
```

Two columns widen to two `Column` ops and `ResultRow 1 2` with OpenRead p4=2;
`SELECT b` alone keeps p4=2 (hint = max used column + 1). The listing is
identical on an empty table (the Rewind jump is only observable at run time —
first step returns DONE) and after reopen. Ten goldens frozen under
`tests/characterization/engine-vdbe48/`, two-run deterministic.

## Which cursor, and how Column reads a cell

`store::vm_scan_ctx` gates to the v45 scope (file-backed, autocommit, non-WAL,
exactly one plain rowid user table, no indexes/views/vtabs/attached/triggers,
no WITHOUT ROWID, no INTEGER PRIMARY KEY — C emits `Rowid` there, which is not
in this pack) and returns the file image, the real root page, the resolved
column indices, and the schema cookie. `OpenRead` opens the cursor on the cells
`pager::read_table_cells` parses from that image (0x0d leaf or one-level 0x05
interior — the v46 split shape). `Rewind`/`Next` position on cells and tick a
**cursor-read counter**; `Column` decodes the current cell's record payload
(`dbfile::decode_record`) into a register. The kitchen store's rows are never
consulted — the anti-cheat proves it: the counter moves on the scan (including
over a split interior root and for a runtime pid-derived payload), and does not
move on `SELECT 1` (v47, no cursor) or on a join (kitchen).

## What the kitchen still owns

Joins, WHERE, select-list expressions (`length(b)` pinned as the boundary),
aggregates, ORDER BY, IPK tables, multi-table schemas, WAL mode, reads inside
an open transaction, all DML. ~185 opcodes unimplemented; no index OpenRead,
no OpenWrite, no SeekGE/MakeRecord, no OP_Program/interrupt/progress.

## Inventory effect

- `vdbe-engine-001` stays **partial**; FROM residual rewritten (first
  table-scan opcodes landed; WHERE/joins/index/DML codegen remain).
- `btree-002` stays **partial**; live-read residual rewritten (the v48 scan
  slice reads cells via the cursor; every other read shape still store).
- `vdbe-engine-002` **stays none** — registers are still a plain value Vec,
  not Mem cells with probed affinity.
- `prepare-statement-api-006` stays partial; bytecode rows now real for the
  landed FROM programs too; nested-loop EQP is still not bytecode (v37 law).
- Composed `engine-vdbe48-001/002/003` full for the frozen batches.
- `sqlite_master.rootpage` projection now answers from the file image (the
  same number OpenRead carries) for file-backed tables.

SQLite is NOT migrated.
