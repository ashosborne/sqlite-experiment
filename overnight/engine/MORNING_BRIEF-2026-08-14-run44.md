# MORNING BRIEF — engine v34: status/pragma matrix (run 44)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–43 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v34-status-pragma-matrix, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 27). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @34 BOUND — STATUS/PRAGMA MATRIX law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v33 → **v34**
(versions/1–34 retained; ADR `0032-engine-v34-status-pragma-matrix.md`; schema VALID; 40 laws).
Everything pinned is core on the bare amalgamation; the probe ran before every freeze.
Counter magnitudes are machine state, so pins are **predicates + exact zeros + exact rc
codes + exact pragma rows** (the run-38 status style).

## 2. Status ops newly tracked (the run-38 residual, cut down)

| Seam | What's real now |
| --- | --- |
| status64 global | ops 0..9 all answer (out-of-range → MISUSE 21): MEMORY_USED (existing), **MALLOC_SIZE / MALLOC_COUNT** (real allocator largest-alloc + outstanding count), **PAGECACHE_OVERFLOW / PAGECACHE_SIZE** (real bytes of db file images held/flushed); **SCRATCH_\*/PARSER_STACK/PAGECACHE_USED exact zeros** (NOT USED on the pin — no tracking invented); resetFlag re-arms highwater; **sqlite3_status 32-bit twin** added |
| db_status | ops 0..12 all answer (bad op → ERROR 1): **CACHE_USED(_SHARED) / SCHEMA_USED / STMT_USED** real byte footprints with highwater 0 like C (STMT_USED goes 0→pos→0 across prepare/finalize); **CACHE_HIT / MISS / WRITE** wired to modern's real I/O events (miss = actual file-image load, write = actual flush, hit = memory-served read; magnitudes not claimed — ADR 0032); **DEFERRED_FKS** = on-demand deferred-FK violation scan, pinned exactly 0→1→0 around a deferred txn |
| honesty line | **lookaside**: C's default build runs one (probe: 6/50 used, 249 hits); modern has none — LOOKASIDE ops stay honest-zero / run-38 vacuous predicates; fabricating positives = greenwash, refused |

## 3. Pragmas / TVFs landed

- Dispatcher: `data_version` (own writes don't bump; **sibling commits do**),
  `schema_version` +1 per DDL, `freelist_count`, `collation_list` (live registry,
  newest-first), `table_xinfo` / `index_info` / `index_xinfo` (incl. C's rowid row),
  **`query_only` enforced** (write → `attempt to write a readonly database`, rc 8),
  **`ignore_check_constraints` enforced**, **`quick_check` really validates CHECKs**
  (pinned `CHECK constraint failed in u` via a row smuggled in under icc),
  **unknown pragma names silently ignored** (get + set — the classic trap; modern used
  to error).
- TVFs: `pragma_collation_list`, `pragma_table_xinfo`, `pragma_index_info`,
  `pragma_compile_options` (the v32 38-entry fingerprint), bare + parenthesized forms.
- Skipped honestly: `pragma_module_list` — the probe showed C fills it lazily with
  whichever pragma vtabs the session has touched; pinning that is fragile (ADR note).

## 4. Flips table

| Card | Before | After | Why |
| --- | --- | --- | --- |
| error-status-api-003 | partial (op-matrix residual) | **partial** (residual shrunk to named leftovers) | full valid-op matrices real; leftovers: lookaside positives, CACHE_SPILL under pressure, stmt_status/scanstatus |
| pragma-surface-001 | partial (~27 of ~70) | **partial (~40 of ~70)** | dispatcher batch + two enforcements + silent-unknown |
| pragma-surface-002 | partial (registries deferred) | **partial** (residual shrunk) | 4 TVFs landed; module_list deferred with reason |
| engine-status34-001/002 | — | **full** (composed) | exact frozen batches (6 + 8) |
| engine-pragma34-001/002 | — | **full** (composed) | exact frozen batches (9 + 4) |

## 5. Anti-cheat + goldens

- 27 new HUMAN_ACCEPTED goldens (`engine-status34/`, `engine-pragma34/`), two-run
  deterministic; prior 706 goldens untouched.
- 3 anti-cheat tests: runtime-sized allocation must move MEMORY_USED by ≥ that size and
  a runtime-named table must grow SCHEMA_USED (hi stays 0); runtime pragma value + runtime
  column name round-trip through dispatcher and TVF; bad ops still fail and a
  runtime-registered collation appears at seq 0 of collation_list (live registry).
- SCRIPT_TABLE.len()==0.

## 6. cargo

`cargo test` (modern): **750 passed / 0 failed** (was 739; +11 status34 twins/anti-cheat).
attach33 / compile32 / vtab31 / attach30 / none29 / harvest / ANALYZE / conn / blob /
vacuum / WAL all green — including after the silent-unknown-pragma and main-only
bare-sqlite_master behavior corrections.

## 7. Scoreboard

before → after: **145 full / 62 partial / 79 none of 286** → **149 full / 62 partial / 79 none of 290**
(four new composed fulls; the three deepened cards stay honest partials with shrunk,
precisely named residuals).

## 8. Not migrated

SQLite is NOT migrated. Not claimed: lookaside allocator, SCRATCH tracking, CACHE_SPILL
pressure paths, stmt_status/scanstatus, the remaining ~30 pragmas, pragma_module_list,
integrity_check corruption taxonomy, xBestIndex pushdown, planner, unlock-notify (absent
on pin), WAL depth, wasm/jni/vfs/FTS/rtree/session.

## 9. Next call (pick one)

1. **xBestIndex deepen** — real constraint offers to vtab modules (EQ pushdown + HIDDEN
   argv path), cutting the vtab-core-002 residual.
2. **stmt_status thin slice** — FULLSCAN_STEP/VM_STEP/RUN on pinned statements
   (the natural sequel to this run's matrix).
3. **another present-core none** — sweep COVERAGE's 79 for the next honest cutter.
