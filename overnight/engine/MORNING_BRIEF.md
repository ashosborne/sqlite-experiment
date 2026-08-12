# MORNING BRIEF — engine v35: real lookaside pool (run 45)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–44 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v35-lookaside-none, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 14). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @35 BOUND — LOOKASIDE/NONE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v34 → **v35**
(versions/1–35 retained; ADR `0033-engine-v35-lookaside-none.md`; schema VALID; 41 laws).

## 2. Presence-check ledger (ADR 0033)

| Surface | Result |
| --- | --- |
| lookaside + db_config(LOOKASIDE) | **PRESENT** on the bare pin (default on; BUSY while live) → implemented |
| delta_create / eval / dbstat / sqlite_stmt / median | ABSENT (`no such function/table` reconfirmed) → **stay none** |
| unlock-notify | ABSENT (run-42 fingerprint) → **stays none** |

No absence "success" goldens frozen; run-39 absence pins untouched.

## 3. What landed — the run-39 "not honestly modellable" gap is closed with a real pool

- **A real slab**: acquired through the counting allocator (MEMORY_USED accounts it once,
  like C), carved into 8-rounded slots (default 1200×40 at connection open, C's shape).
- **Real allocations through it**: `sqlite3_prepare_v2` placement-allocates the
  prepared-statement object from a slot (hit), falls back to the heap on size/full
  misses, and `sqlite3_finalize` returns the slot for reuse. C additionally routes
  parse-tree allocations through lookaside, so magnitudes differ — every growth pin is
  a predicate, every zero pin is C-exact (run-38/44 style). **No invented counters.**
- **db_config(LOOKASIDE)**: OK on a quiet connection, **SQLITE_BUSY(5) while a statement
  is live**, OK after finalize; negative/huge size + negative count normalize rc 0;
  (0,0) disables (USED zeros, HIT frozen); unknown verbs rc 1; HIT/MISS counters survive
  reconfig (probed C behaviour).
- **LOOKASIDE db_status is now real**: USED = (outstanding, highwater; reset pulls hi to
  cur), HIT/MISS_SIZE/MISS_FULL = (current always 0, counter; reset clears). 64-byte
  slots force MISS_SIZE; a 512×2 pool under six live statements forces MISS_FULL; a
  freed slot HITs again; the DEFAULT pool serves traffic with zero config calls.
- Zombie safety: close_v2 with live statements parks the pool; late finalizes drain it
  and the slab frees with the last slot.

## 4. Flips table

| Card | Before | After | Why |
| --- | --- | --- | --- |
| malloc-subsystem-002 | **none** (run-39 "not honestly modellable") | **partial** | real pool + knobs + moving counters; residuals named: two-size mini slots, pBuf external buffers, non-stmt allocations, CONFIG_LOOKASIDE process default |
| error-status-api-003 | partial | partial (LOOKASIDE residual **cleared**) | leftovers now just CACHE_SPILL pressure + stmt_status/scanstatus |
| absent-extension nones | none | none (presence notes) | ledger reconfirmed, zero opportunistic flips |
| engine-lookaside35-001/002/003 | — | **full** (composed) | exact frozen batches (5 + 5 + 4) |

## 5. Anti-cheat + goldens

- 14 new HUMAN_ACCEPTED goldens (`tests/characterization/engine-lookaside35/`), two-run
  deterministic; prior 720 goldens untouched.
- 3 anti-cheat tests: runtime slot count N → exactly N pool-served live statements
  (USED current == N) with exactly the overflow missing FULL; runtime too-small slot
  size → every one of a runtime number of prepares MISS_SIZEs with HIT frozen; bad
  db_status/db_config ops still fail and BUSY guards a live pool.
- SCRIPT_TABLE.len()==0. FORBID_FAKE_LOOKASIDE_COUNTERS honoured — counters only move
  when the pool actually serves or misses an allocation.

## 6. cargo

`cargo test` (modern): **767 passed / 0 failed** (was 750; +17 lookaside35 twins/anti-cheat).
status34 / pragma34 / attach33 / compile32 / vtab31 / attach30 / none29 / harvest /
ANALYZE / conn / blob / vacuum / WAL all green — including conn's close/zombie paths over
the new pool and status34's quiet-op zeros under the default config.

## 7. Scoreboard

before → after: **149 full / 62 partial / 79 none of 290** → **152 full / 63 partial / 78 none of 293**
(malloc-subsystem-002 leaves none; three composed fulls).

## 8. Not migrated

SQLite is NOT migrated. Not claimed: two-size mini-slot carving, pBuf external buffers,
lookaside for non-statement allocations, SQLITE_CONFIG_LOOKASIDE, CACHE_SPILL pressure,
stmt_status/scanstatus, pcache config seam (stretch skipped), xBestIndex, planner,
unlock-notify and every absent extension in the ledger, wasm/jni/vfs/FTS/rtree/session,
btree/pager/vdbe/where internals.

## 9. Next call (pick one)

1. **xBestIndex deepen** — real constraint offers to vtab modules (EQ pushdown + HIDDEN
   argv path), cutting the vtab-core-002 residual.
2. **stmt_status thin slice** — FULLSCAN_STEP/VM_STEP/RUN composed pins (natural sequel
   to the status matrix; also names an error-status-api-003 leftover).
3. **pcache-001 config seam** — the stretch this run skipped (presence-check first).
