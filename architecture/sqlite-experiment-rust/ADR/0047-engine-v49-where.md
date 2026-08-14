# ADR 0047 — engine v49: WHERE compares on the cell cursor (Ne/Eq/Le family, SeekRowid, Variable)

- Status: BOUND (pack v49, supersedes v48; v1–v48 retained at `versions/`)
- Operator: Ash Osborne (full autonomy, delegated stamps)
- Date: run 60

## Decision

Put the WHERE test into the dispatch loop. For `SELECT col(s) FROM t WHERE col
<op> <int-lit|?>` on the v48 scan scope, modern compiles C's exact program and
the compare runs **as an opcode** over btree cells — compare a register to a
column, skip the row or keep it. Never a Rust filter over kitchen or cell rows.

## Which SQL, and C's exact program (probed on the pin)

`EXPLAIN SELECT b FROM t WHERE a=1` (root 2, cookie 1):

```
0  Init        0 9  0
1  OpenRead    0 2  0  p4=2
2  Rewind      0 8  0
3  Column      0 0  1              -- a -> r1
4  Ne          2 7  1  p4=BINARY-8 p5=84   -- jump to Next when r1 != r2
5  Column      0 1  3              -- b -> r3
6  ResultRow   3 1  0
7  Next        0 3  0  p5=1
8  Halt        0 0  0
9  Transaction 0 0  1  p4=0 p5=1
10 Integer     1 2  0              -- the literal, loaded by the init section
11 Goto        0 1  0
```

**C inverts the test into a jump-to-Next compare**: `=`→Ne, `<>`/`!=`→Eq,
`>`→Le, `<`→Ge, `>=`→Lt, `<=`→Gt (all probed, all BINARY-8/84). Two result
columns shift ResultRow to (3,2). The no-match literal only changes the
init-section Integer. `WHERE a=?` swaps Integer for `Variable 1 2`. `WHERE
rowid=2` is a different shape: `Integer 2 1` + `SeekRowid 0 6 1` + Column +
ResultRow — a single-cell touch with no Rewind/Next loop. Nine goldens frozen
under `tests/characterization/engine-vdbe49/`, two-run deterministic.

## How the compare reads registers

`execute_bound` adds Eq/Ne/Lt/Le/Gt/Ge: compare r[p3] (the cell-decoded column)
with r[p1] (the literal/parameter register), jump to p2 when the relation holds
— or when a side is NULL and p5 carries the jump-if-null bit 0x10. Value order
is SQLite's NULL < numbers < text < blob. `Variable` reads the statement's
1-based parameters; `SeekRowid` positions the cursor on the cell whose rowid
equals r[p3] (ticking the cursor-read counter) or jumps when absent. The
anti-cheat pins that the cursor still positions on **rejected** rows (counter ≥
cell count for full scans) and that a runtime payload behind a decoy row
round-trips through Column + Ne.

The prepare-time dry-run falls back to the VM compiler for the rowid-seek
shape only (the kitchen has no rowid binding for plain tables and never runs
that statement) — colnames are the selected identifiers, as C reports.

## What the kitchen still owns

AND/OR, LIKE/GLOB/IS NULL/BETWEEN, text-literal compares, joins, select-list
expressions (`length(b)` still the pinned boundary), aggregates, ORDER BY,
IPK tables (C emits Rowid), index seeks (SeekGE/IdxGE forbidden by law),
OpenWrite/DML codegen, multi-table/attached/WAL scopes, reads inside an open
transaction. ~180 opcodes unimplemented. SeekRowid positions over the parsed
cell list, not a b-tree page descent — named in the residual.

## Inventory effect

- `vdbe-engine-001` stays **partial**; WHERE residual rewritten.
- `btree-002` stays **partial**; the v49 WHERE/seek shapes read cells through
  the same cursor; other read shapes still store.
- `vdbe-engine-002` **stays none** — Eq compares plain values, not Mem cells
  with probed affinity (BINARY-8's affinity byte is replicated in the listing,
  not implemented as a Mem lattice).
- `prepare-statement-api-006` stays partial; bytecode rows now real for the
  landed WHERE programs too; nested-loop EQP still not bytecode (v37 law).
- Composed `engine-vdbe49-001/002` full for the frozen batches.

SQLite is NOT migrated.
