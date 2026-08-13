# ADR 0040 — engine v42: recursive CTE execution (WITH / WITH RECURSIVE, SQLITE_RECURSIVE 33)

Status: accepted (run 52, pack v42)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

Before this run the kitchen neither parsed nor executed WITH / WITH RECURSIVE
(the keywords lived only in the completion census), and ADR 0039 left
SQLITE_RECURSIVE (33) pin-absent for that reason. This run probed C's CTE
machine, froze 11 goldens (engine-harvest42-001..006) and implemented it. No
estate card flips full. SQLite is NOT migrated.

## Probes (pinned bare build, two-run deterministic)

### Non-recursive WITH

Literal seed / real-table body / no-column-list / two CTEs with an earlier-CTE
reference (cross join) / MATERIALIZED and NOT MATERIALIZED (same results on the
pin). Column names come from the declaration or the body projection.

### Recursive UNION ALL — the Queue/Current machine

- `c(x): 1 UNION ALL x+1 WHERE x<5` → `1|2|3|4|5`.
- **Two seed terms interleave**: `1 UNION ALL 10 UNION ALL x+1 WHERE x%10<3` →
  `1|10|2|11|3|12|13` — seeds enter the queue, one row is extracted as Current
  per step (that row IS the recursive table), outputs enqueue FIFO. Not a
  stack; not a rerun over the accumulated set.
- A parent/child walk is **breadth-first** (`1|2|3|4|5|6|7` over a binary
  tree), same for the comma-join and `JOIN..ON` recursive members.
- Multi-column recursion carries expressions (`1,a|2,ab|3,abb`).

### SQLITE_RECURSIVE (33) — scan-gated

- Unused `WITH RECURSIVE … SELECT 42` → only the top-level `[21]`. **Silent.**
- Scanned → `[21][21|~|~|~|c][33|~|~|~|c][21|~|~|~|c][21|~|~|~|c]` (one
  s4-context SELECT consult, the 33, then one per body term).
- Non-recursive WITH → `[21][21|~|~|~|x]` only — **no 33**.
- DENY of 33 → prepare rc 23 `not authorized`, log stops after the 33.

### Probe-then-maybe (all landed clean)

- **Recursive UNION (distinct)**: a cyclic graph terminates (`1|2|3`) — the
  DistFifo never re-enqueues a seen row.
- **Outer LIMIT** stops an unbounded recursion (`LIMIT 4` → `1|2|3|4`);
  **outer ORDER BY** sorts the machine's output; a recursive CTE works as a
  **subquery** source.

### Error shapes (probed, then implemented verbatim)

- `table c has 1 values for 2 columns` (declared column-count mismatch, both
  recursive and plain WITH),
- `circular reference: a` (mutually-referencing non-recursive CTEs; names the
  CTE being re-entered),
- `multiple references to recursive table: c` (`FROM c, c AS d`; a `walk.n`
  column qualifier is NOT a second reference),
- `recursive aggregate queries not supported`.

## Implementation

CTEs live on a thread-local scope stack consulted FIRST by the FROM resolver
(a CTE shadows a real table, like C). Used CTEs materialize lazily,
dependency-first, with an in-progress guard that raises C's circular-reference
error. The recursive machine is a literal VecDeque: seed terms enqueue; each
step pops one row, outputs it, binds it as the CTE's single-row table, runs the
recursive member, enqueues its outputs (a seen-set makes UNION distinct). A
simple outer `SELECT … FROM cte LIMIT n` (no WHERE/GROUP/ORDER) caps the
machine — no invented recursion limit exists. WITH is routed as a query
through prepare / step / exec / get_table, and the authorizer walk fires the
probed consult sequence at prepare (scan-gated 33; DENY → SQLITE_AUTH).

## Stayed named / skipped (honest)

- SEARCH / CYCLE clauses — not probed into this pin's CTE block; not invented.
- Recursive CTEs as VIEW bodies or inside triggers — unexercised.
- Window functions in recursion / deeper affinity edges — unexercised.
- Flattening rewrite (select-codegen-003), planner (select-codegen-001),
  full parse.y WITH productions (parser-grammar-001) — unchanged residuals.

## Estate outcome

| Card | Outcome |
|---|---|
| select-codegen-001 | stays partial — notes record real WITH/recursive execution; residual still full select.c orchestration / flattening / planner |
| select-codegen-002 | untouched (already full) |
| select-codegen-003 | untouched (flattening still absent) |
| auth-callback-api-001 | stays partial — the RECURSIVE pin-absent sentence is REPLACED by the landed scan-gated 33; master bookkeeping / TEMP / REINDEX remain |
| engine-harvest42-001..006 | composed full for the frozen batches |

Scoreboard: 214 full / 42 partial / 78 none of 334 → **220 full / 42 partial /
78 none of 340** (+6 composed). `SCRIPT_TABLE.len()==0`. Run-51 auth suite,
harvest40, set-op and subquery suites all green.
