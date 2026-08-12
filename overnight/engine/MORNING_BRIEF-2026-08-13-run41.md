# MORNING BRIEF — engine v31: vtab core (run 41)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–40 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v31-vtab-core, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 25). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @31 BOUND — VTAB-CORE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v30 → **v31**
(versions/1–31 retained; ADR `0029-engine-v31-vtab-core.md`; schema VALID; 37 laws).
Baseline honesty: `sqlite3_create_module`/`_v2`, `sqlite3_declare_vtab` and the whole
`sqlite3_module` method table are core C API in the bare amalgamation (`src/vtab.c`) — every
pin came from tiny **in-process test modules** (`intseries`, `pairtab`) compiled into the
no-extension C harness. No ext/misc module was force-linked or claimed.

## 2. What landed — a user-registered module is now a real virtual table

Module methods driven by modern: **xCreate**, xConnect (as create alias),
**xBestIndex** (invoked, zero-constraint full scan), **xOpen / xFilter / xEof / xColumn /
xNext / xClose**, **xDisconnect** (close), **xDestroy** (DROP), `_v2` **module destructor**
(replace + close).

| Behaviour | Evidence |
| --- | --- |
| Registration | `sqlite3_create_module(_v2)` on a per-connection registry; redefine replaces + runs the old `_v2` destructor; close runs the rest (pinned counters 0→1→2) |
| CREATE VIRTUAL TABLE | invokes xCreate with C argv convention (`intseries/main/nums/7` pinned round-trip); `sqlite_master` row = `table,nums,nums,0,CREATE VIRTUAL TABLE nums USING intseries(5)` |
| Errors | unknown module → `no such module: nosuch` (exact); xCreate failure surfaces the module's pzErr (`intseries: bad limit 'bogus'`) and leaves **no schema entry** |
| declare_vtab | fixes names/types from inside xCreate/xConnect; outside a constructor → SQLITE_MISUSE 21; `pragma table_info` reports the visible shape |
| HIDDEN columns | excluded from `SELECT *` (n=1 names=value) but selectable (`SELECT lim`) and filterable (`WHERE lim = 5`) via xColumn |
| Cursor SELECT | full scan 1..5; WHERE/aggregates/ORDER BY DESC/JOIN-to-real-table/two-instance subselects all run over the real cursor scan |
| Lifecycle | DROP TABLE → xDestroy (pinned counter) then `no such table`; drop+recreate with a different arg yields new rows; fresh connection must re-register (pinned `no such module: intseries`) |

## 3. Flips table

| Card | Before | After | Why |
| --- | --- | --- | --- |
| vtab-core-001 | none | **partial** | real registry + xCreate + errors + xDestroy + destructors; residual: xConnect schema-reload, eponymous-only, drop_modules, deferred destructor |
| vtab-core-002 | none | **partial** | declare_vtab shape + HIDDEN + MISUSE pinned; residual: vtab_config, HIDDEN constraints via xBestIndex/xFilter argv |
| engine-vtab31-001/002/003 | — | **full** (composed) | exact frozen batches (10 + 8 + 7 cases) |
| misc-vtab-packs-001 | none | none (notes) | umbrella not flipped; harvest28 wholenumber/completion NOT re-homed (bare baseline has no such modules to register) |

## 4. Anti-cheat + goldens

- 25 new HUMAN_ACCEPTED goldens (`tests/characterization/engine-vtab31/`), two-run
  deterministic on the pinned bare C build; prior 642 goldens untouched.
- 3 anti-cheat tests: runtime module/table names with runtime row count/sum
  (`m{seed}`/`vt{seed}`, N from pid), drop+recreate reshape (intseries→pairtab shape and
  payload change), runtime-random unknown-module exact error text.
- SCRIPT_TABLE.len()==0; no cheat sheet; rows come from the module cursor at query time.

## 5. cargo

`cargo test` (modern): **696 passed / 0 failed** (was 668; +28 vtab31 twins/anti-cheat).
attach30 / none29 / harvest28 / ANALYZE / conn / blob / vacuum / WAL suites all green.

## 6. Scoreboard

before → after: **134 full / 58 partial / 84 none of 276** → **137 full / 60 partial / 82 none of 279**.

## 7. Not migrated

SQLite is NOT migrated. vtab residuals: xBestIndex constraint pushdown/cost solving,
vtab_config, xUpdate (writes through vtabs), eponymous-only modules, xConnect on schema
reload, shadow names. Planner, WAL depth, wasm/jni/vfs/FTS/rtree/session untouched.

## 8. Next call (pick one)

1. **xBestIndex deepen** — offer real constraints to the module (EQ pushdown for the test
   module; HIDDEN-column argv path), flipping vtab-core-002 residual honestly.
2. **attached-trigger firing** — the run-40 residual (unqualified trigger-body resolution
   at execution time inside an attached schema).
3. **status/limit matrix** — sqlite3_status/limit surfaces (present core, cheap pins).
