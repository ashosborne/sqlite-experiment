# MORNING BRIEF — engine v36: partial→full harvest (run 46, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–45 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v36-partial-to-full, PACING overnight,
FORBID_GREENWASH_FULL, ALLOW_XBESTINDEX. MAX_NEW_CASES 120 (used 36).

## 1. Pack @36 BOUND — PARTIAL→FULL HARVEST law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v35 → **v36**
(versions/1–36 retained; ADR `0034-engine-v36-partial-to-full.md`; schema VALID; 42 laws).

## 2. Partial → FULL (10 estate flips — the stretch goal landed)

Every flip's former residual is covered by fresh pins (36 goldens, two waves, probe-first):

| Card | Former residual → evidence |
| --- | --- |
| **tokenizer-002** | audit confirmed run-38 closed the chain; fresh comment/string/END pins |
| **parser-grammar-002** | quoted reserved TABLE names end-to-end (all quote styles, CRUD, qualified, master) + runtime-keyword anti-cheat |
| **attach-detach-002** | locked DETACH: active statement reading the schema → `database aux is locked`; main-only statements don't lock |
| **json-funcs-003** | json_valid FLAGS matrix: strict/JSON5 text validators, JSONB byte walker, 1..15 range error (strict `.5` rejection fixed) |
| **foreign-keys-003** | drop-order matrix: immediate block, child-first, NULL children, deferred-drop + COMMIT catch + ROLLBACK restore |
| **vacuum-001** | pending page_size/auto_vacuum apply **at VACUUM** (pending readback pinned); `VACUUM <schema>`; `unknown database` |
| **vacuum-002** | URI INTO **pin-absent with evidence**: USE_URI off → literal-path refusal pinned |
| **vtab-core-002** | sqlite3_vtab_config (ctor-only/MISUSE matrix) + real xBestIndex EQ offers — consumed argvIndex delivers the value to xFilter argv, module bounds the scan (runtime anti-cheat) |
| **connection-lifecycle-api-002** | backup coupling: src close BUSY + errmsg, dst close defers to finish, step/finish error after deferral, tombstoned double-close → MISUSE (post-close depth stays unfrozen UB by design) |
| **blob-io-api-001** | attached-schema opens read real bytes, WITHOUT ROWID refusal, txn write-through durable, non-ASCII names; UTF-16 forms pin-absent (no UTF-16 blob_open API) |

## 3. Deepened, honestly still partial

| Card | This run | Remaining |
| --- | --- | --- |
| error-status-api-003 | **sqlite3_stmt_status**: FULLSCAN_STEP exact rows−1 tally + accumulation (runtime anti-cheat), RUN cycles, MEMUSED real footprint, VM_STEP predicate-only | CACHE_SPILL pressure, VM_STEP magnitudes (no VDBE), scanstatus |
| analyze-stats-001 | attached ANALYZE → `aux.sqlite_stat1` (+ `CREATE INDEX aux.ti` schema resolution); PRAGMA optimize missing-stats contract | optimize usage-gating heuristics; stat4 + sz=/unordered stay pin-absent |

## 4. Stayed partial untouched (structural, per charter)

WAL multi-conn, planner/VDBE, ~30 remaining pragmas, ~60 scalars, zlib byte-format,
va_list, lookaside mini-slots, decimal precision, regexp NFA, dlopen, unlock-notify,
compile-options census, pager/btree/vfs/fts/wasm/jni/session/expert,
attach-detach-003 TEMP-fire matrix, vtab-core-001 mega lifecycle.

## 5. xBestIndex outcome

Real constraint plumbing landed: C-layout `sqlite3_index_constraint(_usage)` arrays, a
detected `col = literal` EQ term offered on single-vtab FROMs, consumed values delivered
through xFilter argv, engine WHERE still applied (safe with omit). Under-claim recorded:
one simple EQ term per scan; series/prefixes/wholenumber not re-homed.

## 6. Anti-cheat + goldens + cargo

- 36 new HUMAN_ACCEPTED goldens (engine-harvest36-001…011); prior 734 goldens untouched.
- 4 anti-cheat tests: runtime-keyword quoted table round-trip; runtime row count →
  FULLSCAN_STEP == N−1; runtime EQ bound reaching the module through argv (offer + argc
  + bounded rows asserted); plus the wave-1 golden runtime probes.
- `cargo test` (modern): **806 passed / 0 failed** (was 767; +39 harvest36 twins).
  All prior suites green. SCRIPT_TABLE.len()==0.

## 7. Scoreboard

before → after: **152 full / 63 partial / 78 none of 293** → **173 full / 53 partial / 78 none of 304**
(10 estate partial→full + 11 composed fulls; partial count down 10; none untouched).

## 8. Not migrated

SQLite is NOT migrated — the §4 list alone spans the pager/btree/VDBE core.

## 9. Next call (pick one)

1. **another partial→full wave** — next one-holes: prepare-statement-api-006 leftovers,
   connection-lifecycle-api-001 URI open, auth-callback s1–s4/IGNORE, backup multi-page,
   window EXCLUDE, global-init-config matrices.
2. **vtab-core-001 lifecycle deepen** — xConnect schema reload, drop_modules, xUpdate.
3. **stmt_status VM_STEP honesty study** — whether any real modern quantity can carry it.
