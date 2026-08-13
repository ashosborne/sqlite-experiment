# MORNING BRIEF — engine v41: authorizer action codes (run 51, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–50 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v41-auth-codes, NO_FAKE_AUTH_MASTER,
AUTH001_NO_FULL, DO_NOT_RECLAIM_AUTH002, GOAL_PARTIAL_TO_FULL false.
MAX_NEW_CASES 40 (used 14).

## 1. Pack @41 BOUND — AUTH CODES WITHOUT MASTER FAKERY law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v40 → **v41**
(versions/1–41 retained; ADR `0039-engine-v41-auth-codes.md`; schema VALID; 47 laws).

## 2. Codes landed (outer events, probed args, per-code IGNORE/DENY)

| Code | Shape landed |
| --- | --- |
| **FUNCTION 31** (headline) | compile-time, once per occurrence, s1 NULL / s2 name, outer-then-inner nesting; DENY = `not authorized to use function: NAME` at **plain rc 1**; IGNORE yields NULL and **de-aggregates** (`min(a)` → 3 NULL rows, `coalesce(min(a),999)` → 999 per row); `count(*)` / ignored refs read the table with an EMPTY column name and NULL schema |
| **SAVEPOINT 32** | s1 = BEGIN/RELEASE/ROLLBACK, s2 = savepoint name; `ROLLBACK TO` split from TRANSACTION 22; DENY rc 23 fires before the exists check |
| **ANALYZE 28** | outer per-table event (s1 = table, s3 = main) on a DB that already has sqlite_stat1; DENY blocks |
| **ALTER_TABLE 26** | outer, **inverted** args (s1 = database, s2 = table) for RENAME + ADD COLUMN; IGNORE silently no-ops the rename; DENY rc 23 |
| **DROP_TABLE 11** | outer (s1 = table, s3 = main); DENY leaves the table |
| **CREATE_VTABLE 29 / DROP_VTABLE 30** | outer (s1 = table, s2 = module); a vtab drop fires 30, not 11; DENY blocks |
| **view s4** | the WHOLE view body's base-table READs carry s4 = view name (regardless of outer projection), then projected view columns (s4 NULL), then a nested SELECT consult with s4 = view |
| **trigger s4** | body INSERT + old./new. column READs fire with s4 = the trigger name |

## 3. Codes skipped (honest, ADR 0039)

- **CREATE_INDEX/DROP_INDEX, CREATE_VIEW/DROP_VIEW, CREATE_TRIGGER/DROP_TRIGGER full logs** — dominated by sqlite_master INSERT/UPDATE/DELETE/READ bookkeeping modern does not perform (the auth2-2.1 smoking gun). Not invented.
- **TEMP family (3–6, 12–15)** — distinct temp catalog; not aliased. Stays named.
- **REINDEX 27** — kitchen does not parse REINDEX; no fake rebuild. Stays named.
- **RECURSIVE 33** — scan-gated in C; modern has no recursive-CTE execution. Pin-absent.
- ALTER/DROP/ANALYZE/VTABLE **catalog tails** — outer codes frozen, tails deliberately dropped
  from goldens (the harness FILTERS the callback log to whitelisted codes).

## 4. Residual rewrite

auth-callback-api-001 **stays partial**: DDL sqlite_master/sqlite_temp_master bookkeeping
families (index/view/trigger full logs + the unfrozen catalog tails of ALTER/DROP/VTABLE),
TEMP family, REINDEX (unparsed), RECURSIVE (pin-absent). auth-callback-api-002 untouched.

## 5. Anti-cheat

A runtime-registered UDF name must reach s2 and denying exactly that name must fail with
C's per-name message; a runtime savepoint name must reach s2 of code 32. Both in
`modern/tests/engine_harvest41.rs`.

## 6. Probes / bug found

All shapes probed on C first (probe → freeze → implement). **Pre-existing eval bug** flushed
out by the coalesce pin: scalar functions over aggregates evaluated the aggregate per-row
(`coalesce(min(a),999)` returned 1, not -3). Fixed with aggregate-context argument evaluation.

## 7. Freezes / cargo

14 new cases under `tests/characterization/engine-harvest41/` (001 ×3, 002 ×2, 003 ×2,
004 ×2, 005 ×3, 006 ×2), two-run deterministic, delegated HUMAN_ACCEPTED, legacy_green 238,
prior golden md5s untouched. `cargo test` (51 binaries): **all green** including run-47
auth s1–s4 / DELETE-proceeds and auth-002 IGNORE→NULL. `SCRIPT_TABLE.len()==0`.

## 8. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 208 | **214** |
| partial | 42 | 42 |
| none | 78 | 78 |
| behaviours | 328 | 334 (+6 composed) |

partial→full: none (by design — GOAL_PARTIAL_TO_FULL false). Composed
engine-harvest41-001..006 full. auth-callback-api-001 deepened, still partial.

## 9. Not migrated

SQLite is **not migrated**. The authorizer's DDL bookkeeping walks, TEMP catalog, REINDEX,
recursive CTEs, WAL concurrency, planner/VDBE, pager/btree, FTS, sessions and most of the
wider C API remain partial or absent.

## 10. Next call

(a) recursive CTE execution (would unlock RECURSIVE 33 honestly and a chunk of select-codegen);
(b) json-funcs-002 array-path mutation; (c) pragma-surface-002 result-pragma projections;
(d) attach-detach-003 TEMP-trigger fire matrix. The auth master-bookkeeping family needs real
catalog DML first — do not fake it.
