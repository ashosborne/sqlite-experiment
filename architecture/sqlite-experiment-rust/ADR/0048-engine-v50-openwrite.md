# ADR 0048 — engine v50: INSERT bytecode (OpenWrite/NewRowid/MakeRecord/Insert)

- Status: BOUND (pack v50, supersedes v49; v1–v49 retained at `versions/`)
- Operator: Ash Osborne (full autonomy, delegated stamps)
- Date: run 61

## Decision

Give the VM its first write. For a single-row `INSERT INTO t[(cols)]
VALUES(...)` on the v48/v49 scope, modern compiles C's exact program and
`sqlite3_step` dispatches it: open a write cursor, build a record, put a cell.
The cell that lands on disk is the one MakeRecord built — the kitchen store
only mirrors the row afterwards (bookkeeping, never the writer).

## Which SQL, and C's exact program (probed on the pin)

`EXPLAIN INSERT INTO t(a,b) VALUES(1,'one')` (root 2, cookie 1):

```
0 Init        0 8 0
1 OpenWrite   0 2 0  p4=2        -- p2 = the REAL root page, p4 = column count
2 Integer     1 2 0              -- a -> r2
3 String8     0 3 0  p4='one'    -- b -> r3
4 NewRowid    0 1 0              -- fresh rowid -> r1
5 MakeRecord  2 2 4  p4=DB       -- r2..r3 -> record blob r4; p4 = affinity string
6 Insert      0 4 1  p4=t p5=57  -- put (r1, r4) through cursor 0
7 Halt        0 0 0
8 Transaction 0 1 1  p4=0 p5=1   -- p2=1: a WRITE transaction
9 Goto        0 1 0
```

Omitting the column list emits the identical program; `VALUES(?,?)` swaps the
value loads for `Variable 1 2` / `Variable 2 3`. Execution: DONE,
`last_insert_rowid` 1, rowids in C's order, `changes 1 / total_changes 2`
after the second insert, reset+step inserts again with C's next rowids (3, 4),
bound values land in the row, and VDBE-inserted rows are read back by the v48
scan, the v49 WHERE compare and the rowid seek. Eight goldens frozen under
`tests/characterization/engine-vdbe50/`, two-run deterministic.

## How MakeRecord/Insert write the cell

`execute_dml` adds four opcodes: **OpenWrite** opens the cursor on the table's
cells; **NewRowid** computes max(rowid)+1 into a register; **MakeRecord**
encodes registers p1..p1+p2-1 into a record blob (`dbfile::encode_record` —
the p4 affinity string is replicated from C's listing, not applied as a Mem
lattice; vdbe-engine-002 stays none); **Insert** hands the (rowid, payload)
cell out and ticks the insert counter. `pager::insert_cell` then puts THAT
payload **verbatim** through the same leaf packing the v46 split uses
(cursor-op counter ticks; split-capable; refuses duplicates/overflow-chains),
and `store::vm_persist_insert` commits via the pager's journal-then-write
mini-txn before mirroring the row into the store (`next_rowid` keeps the
kitchen's last-assigned-rowid semantics). `sqlite3_total_changes`/`64` were
added — the store already tracked the value and the golden pins it.

## Gates (kitchen keeps everything else)

`store::vm_insert_ctx` = the v48 scan scope PLUS: value count must match the
table, a column list must be exactly the declared columns in order, no column
constraints (NOT NULL/UNIQUE/CHECK/DEFAULT/FK), no IPK (changes NewRowid), and
no update/commit hooks or authorizer registered (those side-effect paths stay
kitchen, honestly named). UPDATE/DELETE, INSERT SELECT, multi-row VALUES,
upsert, and the `sqlite3_exec` path remain kitchen. No index OpenWrite.

## Inventory effect

- `vdbe-engine-001` stays **partial**; DML residual rewritten.
- `btree-002` stays **partial**; the VDBE Insert now drives the cursor write.
- `vdbe-engine-002` **stays none** — affinity string replicated, not applied.
- `prepare-statement-api-006` stays partial; bytecode rows real for INSERT
  programs too.
- Composed `engine-vdbe50-001/002` full for the frozen batches.

SQLite is NOT migrated.
