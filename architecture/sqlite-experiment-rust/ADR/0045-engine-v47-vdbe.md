# ADR 0045 — engine v47: the first bytecode slice (constant-SELECT VDBE)

- Status: BOUND (pack v47, supersedes v46; v1–v46 retained at `versions/`)
- Operator: Ash Osborne (full autonomy, delegated stamps)
- Date: run 58

## Decision

Give the engine its first instruction stream. A statement is a list of
instructions; `sqlite3_step` runs them. For **constant SELECTs only**, modern
compiles the SQL to the same program C emits, `EXPLAIN` returns that listing
row-for-row against frozen C, and stepping the SQL executes the program through
a real dispatch loop. Everything else stays on the kitchen evaluator, honestly
named.

## Which SQL, and C's exact programs (probed, not copied from a textbook)

Probed on the pinned amalgamation (3.54.0, API_ARMOR off):

| SQL | C's program |
|---|---|
| `SELECT 1` / `SELECT 42` | `Init 0 4 0`, `Integer n 1 0`, `ResultRow 1 1 0`, `Halt 0 0 0`, `Goto 0 1 0` |
| `SELECT 1 WHERE 1` | identical — C folds the always-true WHERE |
| `SELECT 1 WHERE 0` | `Init 0 5`, **`Goto 0 4`** (jump straight to Halt), then the unreached Integer/ResultRow, `Halt`, `Goto 0 1` |
| `SELECT 1+2` | **not folded**: `Init 0 4`, `Add 3 2 1`, `ResultRow 1 1`, `Halt`, then the init section `Integer 1 2`, `Integer 2 3`, `Goto 0 1` |
| `SELECT 'hi'` | `String8 0 1 0` with the text in p4 |
| `SELECT 1, 2` | two Integers into r1/r2, `ResultRow 1 2` |

Row shape is C's eight EXPLAIN columns: addr, opcode, p1, p2, p3, p4 (NULL
except String8), p5 (0), comment (NULL on this default build). Frozen at
`tests/characterization/engine-vdbe47/` (5 cases, two-run deterministic).

## What the loop implements

`modern/src/vdbe.rs`: `compile()` recognises exactly the probed patterns and
emits C's layout (including the WHERE-0 jump and the 1+2 init-section loads);
anything else returns None and the kitchen keeps it. `execute()` is the
dispatch loop — a register file, a pc, and seven opcodes: **Init, Goto,
Integer, String8, Add, ResultRow, Halt**. Every executed opcode bumps a
dispatch counter; that counter is the anti-cheat (it moves on `SELECT 1`
steps, it does not move on a join, which is still the evaluator's).

`sqlite3_step` of a compiled statement runs the program — not
`stmt_query_typed`. `EXPLAIN`-mode statements return the real listing for
compiled programs and stay honestly empty for kitchen-owned SQL.

## What the kitchen still owns

Everything with a FROM (table-scan opcodes OpenRead/Rewind/Column are out of
scope until they drive the v45 btree cursor), DML, CTEs, aggregates, joins,
subqueries, all pragmas. ~192 opcodes unimplemented. No OP_Program, no
interrupt, no progress handler.

## Inventory effect

- `vdbe-engine-001` none → **partial** (VDBE/NONE law satisfied: EXPLAIN
  matches probed C *and* step dispatches; residual = FROM/other opcodes/
  triggers/interrupt/progress).
- `vdbe-engine-002` **stays none** — the loop's registers are a plain value
  Vec, not Mem cells with probed affinity (the lookaside/pcache lesson).
- `prepare-statement-api-006` stays partial; residual rewritten — bytecode row
  contents now real for the landed programs; EXPLAIN of kitchen-owned SQL
  still rowless; nested-loop EQP is still not bytecode (v37 law).
- Composed `engine-vdbe47-001/002` full for the frozen batches.

SQLite is NOT migrated.
