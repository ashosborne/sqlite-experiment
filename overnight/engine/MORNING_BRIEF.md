# MORNING BRIEF — engine v42: recursive CTE execution (run 52, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–51 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v42-recursive-cte, RECURSIVE33_SCAN_GATED,
NO_FAKE_RECURSION_LIMIT, DO_NOT_RECLAIM_SELECT002, AUTH001_NO_FULL, GOAL_PARTIAL_TO_FULL false.
MAX_NEW_CASES 40 (used 11).

## 1. Pack @42 BOUND — RECURSIVE CTE EXECUTION law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v41 → **v42**
(versions/1–42 retained; ADR `0040-engine-v42-recursive-cte.md`; schema VALID; 48 laws).

## 2. Slices landed

| Slice | What landed |
| --- | --- |
| **Non-recursive WITH** | literal / real-table bodies, no-column-list, multiple CTEs with earlier references, MATERIALIZED variants; CTEs shadow same-named tables |
| **Recursive UNION ALL** (headline) | C's **Queue/Current FIFO**: seeds enqueue, ONE extracted row is the recursive table per step, outputs enqueue. Multi-seed terms interleave (`1|10|2|11|3|12|13`); parent/child walks are breadth-first; multi-column recursion carries expressions |
| **SQLITE_RECURSIVE 33** | scan-gated exactly like C: unused WITH RECURSIVE silent; a scanned CTE fires `[21\|c][33\|~\|~\|~\|c][21\|c][21\|c]`; DENY rc 23. Fired from prepare, never at WITH parse |
| **Recursive UNION (distinct)** | probed clean → landed: DistFifo seen-set, a cyclic graph terminates (`1|2|3`) |
| **Outer LIMIT / ORDER BY / subquery** | LIMIT stops an unbounded machine (`LIMIT 4` on an infinite CTE); ORDER BY sorts the output; recursive CTEs work as subquery sources |
| **Compile errors** | C's exact messages: `table c has 1 values for 2 columns`, `circular reference: a`, `multiple references to recursive table: c`, `recursive aggregate queries not supported` |

## 3. Skipped (honest, ADR 0040)

SEARCH/CYCLE (not in this pin's CTE block — not invented); recursive CTEs in VIEW bodies /
triggers; window-in-recursion; **no invented recursion-limit error** (an unbounded UNION ALL
without LIMIT runs unbounded, like C); flattening (select-codegen-003) and planner
(select-codegen-001) untouched; recursive **triggers** are a different surface (untouched).

## 4. Residual updates

- **auth-callback-api-001** stays partial — the "RECURSIVE 33 pin-absent" sentence is replaced
  by the landed scan-gated dispatch; sqlite_master bookkeeping / TEMP / REINDEX remain named.
- **select-codegen-001** stays partial — WITH/recursive execution noted; residual still full
  select.c orchestration / flattening / planner.
- **select-codegen-002 / 003** untouched.

## 5. Anti-cheat

A runtime stop bound must drive the row count (`x<{n}` for a pid-derived n); a runtime CTE
name must appear in the code-33 s4 slot when scanned. Both in `modern/tests/engine_harvest42.rs`.

## 6. Freezes / cargo

11 new cases under `tests/characterization/engine-harvest42/` (001 ×2, 002 ×2, 003 ×1,
004 ×2, 005 ×2, 006 ×2), two-run deterministic, delegated HUMAN_ACCEPTED, legacy_green 244,
prior golden md5s untouched. `cargo test` (52 binaries): **all green**, including the run-51
auth suite, harvest40, set-op and subquery suites. `SCRIPT_TABLE.len()==0`.

## 7. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 214 | **220** |
| partial | 42 | 42 |
| none | 78 | 78 |
| behaviours | 334 | 340 (+6 composed) |

partial→full: none (by design). Composed engine-harvest42-001..006 full.

## 8. Not migrated

SQLite is **not migrated**. Flattening, the planner, full select.c orchestration, SEARCH/CYCLE,
the authorizer's DDL bookkeeping walks, TEMP catalog, REINDEX, WAL concurrency, VDBE, pager/
btree, FTS and sessions remain partial or absent.

## 9. Next call

(a) TEMP schema family (CREATE TEMP TABLE + sqlite_temp_master would unlock auth TEMP codes
honestly); (b) json-funcs-002 array-path mutation; (c) pragma-surface-002 result-pragma
projections; (d) attach-detach-003 TEMP-trigger fire matrix; (e) subquery flattening probes
for select-codegen-003 (heavy — only with a full pack).
