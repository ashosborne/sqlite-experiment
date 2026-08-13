# MORNING BRIEF — engine v47: the first bytecode slice (run 58, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–57 stamped alongside
(run-57 brief archived at MORNING_BRIEF-2026-08-15-run57.md).
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v47-vdbe, IMPLEMENT_VDBE_DISPATCH,
REQUIRE_EXPLAIN_MATCHES_C, FORBID_KITCHEN_EVAL_AS_VDBE, FORBID_EQP_AS_BYTECODE.
MAX_NEW_CASES 40 (used 5).

## 1. Pack @47 BOUND — VDBE/NONE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v46 → **v47**
(versions/1–47 retained; ADR `0045-engine-v47-vdbe.md`; schema VALID; 53 laws).
The law: vdbe-engine-001 leaves none only if EXPLAIN rows match probed C (names + p1–p5,
never a paraphrase) **and** sqlite3_step of the same SQL runs the program in a dispatch loop
(counter moves; kitchen-fallback SQL doesn't move it). Canned EXPLAIN, EQP-as-bytecode and
199-opcode full forbidden.

## 2. C's programs (probed, frozen)

`EXPLAIN SELECT 1` on this pin: `Init 0 4` · `Integer 1 1` · `ResultRow 1 1` · `Halt` ·
`Goto 0 1` (8 columns; p4 NULL except String8's text, p5 0, comment NULL). `WHERE 1` folds to
the same program; `WHERE 0` inserts **Goto→Halt** so no row is produced; **`1+2` is NOT
constant-folded** — `Add 3 2 1` with the `Integer` loads in the init section *after* Halt;
`'hi'` is `String8` with p4; `1, 2` widens to `ResultRow 1 2`. Execution: values, zero rows
for WHERE 0, and the step/step/reset/step cycle (ROW+1, DONE, ROW+1). 5 goldens under
`tests/characterization/engine-vdbe47/`, two-run deterministic, delegated HUMAN_ACCEPTED.

## 3. Modern: dispatch, not kitchen

`modern/src/vdbe.rs`: `compile()` recognises exactly the probed constant-SELECT shapes and
emits C's program layout; `execute()` is a dispatch loop over **Init, Goto, Integer, String8,
Add, ResultRow, Halt** (register file, pc jumps, dispatch counter). `sqlite3_step` of compiled
SQL runs the program — the counter moves; a join (still the evaluator's) does not move it.
EXPLAIN-mode statements return the real listing for compiled programs and stay honestly empty
for kitchen SQL. Prior prepare-006 pins (isexplain, EQP shape on FROM-table SQL) untouched.

## 4. Inventory

- **vdbe-engine-001 none → partial** — residual: everything with a FROM (OpenRead/Rewind/
  Column on the v45 cursor), ~192 opcodes, OP_Program/interrupt/progress, DML/CTE/aggregate
  codegen — all still kitchen, honestly named.
- **vdbe-engine-002 stays none** — the loop's registers are a plain value Vec, not Mem cells
  with probed affinity (lookaside/pcache lesson applied).
- **prepare-statement-api-006 stays partial** — bytecode rows now real for landed programs;
  kitchen-owned SQL still rowless; nested-loop EQP still not bytecode (v37).
- Composed `engine-vdbe47-001/002` full. btree/pager/wal/select/expr untouched.

## 5. Scoreboard

**233 full / 45 partial / 74 none of 352.** All 60 test binaries green (btree46, pager44,
sqlite_sql_suite first slice included). SCRIPT_TABLE.len()==0.

## 6. Not migrated

SQLite is **not** migrated. SQL with a FROM never touches the VM; 192 opcodes, Mem cells,
triggers, interrupt, progress, WAL depth, leaf merge, index btrees all absent.

## 7. Next call

**OpenRead/Rewind/Column on the v45 btree cursor** (the first table-scan program — would let
`SELECT * FROM t` step through real opcodes over real pages), or **leaf merge on DELETE**
(shrink below the split; needs a freelist).
