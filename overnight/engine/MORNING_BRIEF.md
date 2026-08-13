# MORNING BRIEF — engine v40: partial-to-full harvest (run 50, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–49 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v40-partial-to-full, FORBID_GREENWASH_FULL,
VTAB001_FAMILY_ALL_OR_NOTHING, PREPARE006_NO_VDBE, NO_FAKE_AUTH_MASTER,
SERIES_OWN_RESIDUAL_ONLY. MAX_NEW_CASES 80 (used 23).

## 1. Pack @40 BOUND — PARTIAL-TO-FULL HARVEST law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v39 → **v40**
(versions/1–40 retained; ADR `0038-engine-v40-partial-to-full.md`; schema VALID; 46 laws).

## 2. partial → full (5 estate + 8 composed)

| Card | What cleared |
| --- | --- |
| **vtab-core-001** | xRename (ALTER RENAME rewrites the quoted schema sql; renames even without an xRename method) + the savepoint family: xBegin at first write, xSync/xCommit at COMMIT/statement end, xRollback, and xSavepoint/xRelease/xRollbackTo with **C's txn-savepoint-excluded numbering** (-1 below the join, RELEASE of the txn savepoint commits). ROLLBACK TO now changes vtab DML visibility. Residual empty. |
| **connection-lifecycle-api-004** | WITHOUT ROWID update_hook suppression + DELETE truncate fast-path (0 events, changes() counts); legacy `sqlite3_trace`/`sqlite3_profile` (shared slot, mutual displacement, param-expanded text, trace_v2 replaces both). Residual empty. |
| **json-funcs-004** | json_tree + full vtab columns (key/value/type/atom/id/parent/fullkey/path) with **JSONB-byte-offset ids**, parent-row chains, minified containers, second path arg. Residual empty. |
| **global-init-config-003** | the whole toggle family with REAL effects: DQS_DDL, WRITABLE_SCHEMA, **DEFENSIVE now enforced**, LEGACY_ALTER, RESET_DATABASE, TRUSTED_SCHEMA (INNOCUOUS-aware), load-ext C-API/SQL split. Residual empty. |
| **misc-completion-001** | mirrors live C's phase contract (147-keyword census, databases, tables+views, columns; hidden phase column). Dead phases (pragmas/functions/collations) + ranking are **pin-absent** in live C. Residual empty. |
| engine-harvest40-001..008 | composed full for the frozen batches |

## 3. Still partial (named residuals, honest)

- **pragma-surface-002** — index_xinfo TVF landed (40-008); RESIDUAL: remaining result pragmas (index_list/foreign_key_list projections stay count-only).
- **error-status-api-003** — CACHE_SPILL under real spill, scanstatus, VM_STEP magnitudes (no VDBE).
- **malloc-subsystem-002** — two-size mini-slots, pBuf, CONFIG_LOOKASIDE.
- **auth-callback-api-001** — ~22 action codes (no faked sqlite_master sequences).
- **wal-001/002** — multi-connection mxFrame / blocking checkpoint.
- **json-funcs-001/002** — wildcards/#, JSONB, JSON5, array-path mutation.
- **util-primitives-001** — string hash tables; ChaCha20 byte parity unclaimed.
- **misc-compress-001** (zlib), **printf-format-002/003** (va_list), **loadext-api-001** (dlopen),
  **prepare-statement-api-006** (EXPLAIN bytecode, no VDBE), planner/optimizer/codegen family,
  **serialize-memdb-api-002** (USE_URI off), **series/prefixes/wholenumber** (ADR 0034 no re-home),
  attach mazes, global-init-002 before-init ops, pager/btree/vfs/fts/session/expert. All structural.

## 4. Probes

legacy trace/profile **exported** on the pin → implemented; completion phases 2–6 + ranking
**pin-absent** in live C → residual deletable; JSONB ids computed from C's encoding; DEFENSIVE
verified to gate schema writes and no-op PRAGMA writable_schema; index_xinfo tail row cid -1/key 0.

## 5. Anti-cheat

Runtime rename target must reach xRename + the schema; a runtime-length batch after a savepoint
must vanish on ROLLBACK TO; a runtime document's json_each ids must track JSONB offsets; a
runtime-named table completes under phase 8. All in the engine_harvest40* test binaries.

## 6. Freezes

23 new cases under `tests/characterization/engine-harvest40/` (001 rename ×2, 002 savepoint ×3,
003 legacy trace ×2, 004 WR/truncate ×2, 005 json ×3, 006 db_config ×6, 007 completion ×2,
008 index_xinfo ×2), two-run deterministic, delegated HUMAN_ACCEPTED. legacy_green 230.
Prior golden md5s untouched.

## 7. Cargo

`cargo test` (workspace, 50 binaries): **all green**, including harvest36/37/39, vtab38,
lookaside35, status34/pragma34, utf16 suites. `SCRIPT_TABLE.len()==0` enforced.

## 8. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 195 | **208** |
| partial | 47 | 42 |
| none | 78 | 78 |
| behaviours | 320 | 328 (+8 composed) |

partial→full: vtab-core-001, connection-lifecycle-api-004, json-funcs-004,
global-init-config-003, misc-completion-001 (estate) + engine-harvest40-001..008 (composed).
deepen-still-partial: pragma-surface-002 (index_xinfo added, residual shrunk).

## 9. Not migrated

SQLite is **not migrated**. WAL concurrency, planner/VDBE bytecode, pager/btree internals,
FTS, sessions, most of the pragma census and wider C API remain partial or absent. This run
closed two Tier-A one-family leftovers and three probe-then-maybe cards; nothing more is claimed.

## 10. Next call

(a) auth-callback-api-001 remaining action codes that DON'T need sqlite_master bookkeeping;
(b) json-funcs-002 array-path mutation (JSONB stays named); (c) pragma-surface-002 result-pragma
projections; (d) attach-detach-003 TEMP-trigger fire matrix. No planner/WAL/VDBE full without
the underlying engine.
