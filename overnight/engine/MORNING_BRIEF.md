# MORNING BRIEF — engine v26: connection lifecycle (run 36)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–35 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v26-connection-lifecycle,
DEEPEN_WAL/VACUUM/BLOB: false. MAX_NEW_CASES 60 (used 22; stretch skipped).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @26 BOUND — connection lifecycle law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v25 → **v26**
(versions/1–26 retained; ADR `0024-engine-v26-connection-lifecycle.md`; schema VALID;
32 laws). Plain language: a connection won't close while statements live; close_v2
says OK and waits; busy handlers sleep and retry against a real (in-process) write
lock; and hooks fire on commit, row change, and trace events. WAL stays at v22,
VACUUM at v24, blob at v25.

## 2. What landed (esp. busy honesty)

| Piece | Behaviour |
| --- | --- |
| close | refuses (rc 5, exact C errmsg) while prepared statements **or blob handles** live; reset does not unblock; finalize/blob_close do (pinned) |
| close_v2 | returns OK, zombies; the statement stays usable (pinned read-after-close_v2); real teardown at the last handle release |
| open-txn close | rolls back — pinned via file reopen |
| **busy (the honesty story)** | a NEW in-process per-file write lock: BEGIN IMMEDIATE holds it, a second connection's write consults its busy handler with increasing retry counts (1-call and 3-call shapes pinned) or sleeps under busy_timeout, then fails rc 5 "database is locked"; handler ⟷ timeout mutually exclusive (pinned); COMMIT releases and the blocked write succeeds. **C's cross-process locking is NOT claimed** — this is the single-process regime only, and the card says so |
| cross-conn visibility | committed state now flushes to the file at COMMIT (C's durability point) and sibling connections reload before their next statement (only with no local writes / no open txn) |
| commit_hook | fires per committed txn (2 autocommits + 1 explicit = 3, pinned); non-zero turns COMMIT into a rollback (rc 19 "constraint failed", autocommit restored, data unchanged); autocommit aborts use a pre-statement snapshot; replacement returns the prior argument |
| update_hook | (op 18/23/9, "main", table, rowid) with IPK-aliased rowids (pinned log); unset stops fires and returns the prior argument |
| trace_v2 | STMT sees statement text; ROW per delivered row; CLOSE once at teardown; PROFILE per completed statement (counts); mask 0 unsets |

## 3. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| connection-lifecycle-api-002 | none | **partial** | backup-handle close coupling; post-close MISUSE matrix (use-after-close is UB — deliberately unfrozen) |
| connection-lifecycle-api-003 | none | **partial** | single-process lock model only; no cross-process locking / shared cache / unlock-notify |
| connection-lifecycle-api-004 | none | **partial** | STMT/PROFILE per exec (not per prepared stmt in multi-statement scripts); WITHOUT ROWID / truncate fast-path; legacy trace/profile |
| engine-conn-001/002/003 | — | **new full ×3** | composed cards for exactly the frozen batches |

connection-lifecycle-api-001 untouched (no thin URI crumb fell out). Stretch skipped.

## 4. Anti-cheat + cargo

- `anti_cheat_conn_runtime` — close-BUSY→finalize→close-OK cycle proves live
  statement tracking; a pid-seeded table name and rowid appear in the update_hook
  log; a runtime commit_hook abort leaves the runtime row out of the table.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 589/589 PASS** (was 565; +22 golden twins, +2 anti-cheat/guard).
  engine-blob / engine-vacuum / engine-wal / upsert-expr / collation / utf16 /
  harvest23 all green; 550 pre-run goldens md5-verified intact.

## 5. Scoreboard (impl_in_modern) — before → after

| State | Run 35 | Run 36 |
| --- | --- | --- |
| **full (converted)** | 115 | **118** |
| partial | 48 | 51 |
| none (remaining) | 97 | **94** |
| behaviours known | 260 | 263 |

legacy_green 170 → 173. parity_green 0. Nothing `verified`. WAL/VACUUM/blob unchanged.

## 6. Not migrated

SQLite is **not migrated**. 118/263 behaviours run honestly in modern for frozen
scope only. The lock model is in-process; no shared cache, no unlock-notify, no
cross-process coordination. Parity UNVERIFIED everywhere (COMPARE never run).

## 7. Next call

1. **analyze-stats none** — ANALYZE + sqlite_stat1 as real store output.
2. **get_table / status crumbs** — exec-convenience-api-002 + error-status-api-003.
3. **auth-callback-api-002** — column-read IGNORE → NULL (builds on auth-001).
