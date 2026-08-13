# MORNING BRIEF — engine v37: one-hole partial→full wave (run 47, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–46 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v37-oneholes, URI_HONESTY, PREPARE006_NO_VDBE,
FORBID_GREENWASH_FULL. MAX_NEW_CASES 80 (used 18).

## 1. Pack @37 BOUND — ONE-HOLE PARTIAL→FULL law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v36 → **v37**
(versions/1–37 retained; ADR `0035-engine-v37-oneholes.md`; schema VALID; 43 laws).

## 2. The URI probe (charter's honesty gate — ADR 0035)

USE_URI is off on this pin: plain `sqlite3_open("file:...")` is a **literal path**
(CANTOPEN pinned; no parser invented). `sqlite3_open_v2 + SQLITE_OPEN_URI` **does**
parse per-open — pinned: real path creation, `mode=ro` → write rc 8, `mode=memory` off
disk, `vfs=nosuch` → `no such vfs`, parameters readable via
`sqlite3_uri_parameter/int64/boolean` on the query-stripped `db_filename`. The urifuncs
SQL surface is ABSENT on the bare pin (pinned refusal).

## 3. Partial → FULL (5 estate flips)

| Card | Former residual → cleared by |
| --- | --- |
| **connection-lifecycle-api-001** | URI parsing + open flags → open_v2 matrix (READONLY/CREATE/MEMORY/zero-flags/uncreatable paths) + per-open URI behaviour above |
| **misc-urifuncs-001** | URI parameter parsing real; SQL surface pin-absent with pinned refusal (vacuum-002 bar) |
| **backup-api-001** | multi-page lifecycle with destination content verified; empty-pair pins stay exact |
| **backup-api-002** | remaining()/pagecount() move with exact step-quantum relations; runtime-sized anti-cheat |
| **window-functions-002** | EXCLUDE ×4 + offset RANGE ×2 + composition, pinned over tie rows |

## 4. Deepened, honestly still partial (tightened residuals)

| Card | Landed | Left |
| --- | --- | --- |
| auth-callback-api-001 | probed s1–s4 strings for 9 statement shapes; IGNORE: INSERT/UPDATE skip, **DELETE proceeds** | ~22 codes (sqlite_master bookkeeping sequences, function 31, trigger/view s4) |
| backup-api-003 | restart re-pinned on real multi-page copy; dest sees late write | cross-connection BackupUpdate page patching |
| prepare-statement-api-006 | stmt_isexplain + stmt_explain mode switching real | EXPLAIN bytecode row contents (no VDBE — not claimed, per law) |
| global-init-config-002 | after-init matrix (threading/MEMSTATUS/URI MISUSE; LOG legal; PCACHE_HDRSZ real record size) | before-init ~20-op configuration matrix |
| global-init-config-003 | ENABLE_TRIGGER / ENABLE_VIEW / DQS_DML **with real effects** + DEFENSIVE round-trip | DQS_DDL, WRITABLE_SCHEMA, RESET_DATABASE, LEGACY_ALTER, TRUSTED_SCHEMA; DEFENSIVE effect |

## 5. Anti-cheat + goldens + cargo

- 18 new HUMAN_ACCEPTED goldens (engine-harvest37-001…007); prior 761 goldens untouched.
- 3 anti-cheat tests: runtime URI parameter round-trip through the real parser; runtime
  source size → backup remaining/pagecount relations + copied row count; runtime payload
  excluded from a window frame by EXCLUDE CURRENT ROW.
- `cargo test` (modern): **827 passed / 0 failed** (was 806; +21). The reworked REAL
  backup engine still satisfies the run-12 pins (the legacy twin's source was re-created
  to the C harness's real 2-page shape; goldens untouched). SCRIPT_TABLE.len()==0.

## 6. Scoreboard

before → after: **173 full / 53 partial / 78 none of 304** → **185 full / 48 partial / 78 none of 311**
(5 estate flips + 7 composed; 5 tightened partials; nones untouched).

## 7. Not migrated

SQLite is NOT migrated. Untouched per charter: WAL multi-conn, planner/VDBE, pragma
census, ~60 scalars, zlib, va_list, mini-slots, decimal, regexp NFA, dlopen,
unlock-notify, guard census, pager/btree/vfs/fts/wasm/jni/session/expert, vtab-core-001
mega lifecycle, attach-detach-003 TEMP fire, error-status-api-003 leftovers.

## 8. Next call (pick one)

1. **auth action-code sweep** — decide how far the sqlite_master-bookkeeping callback
   sequences can honestly go (the biggest remaining auth leftover).
2. **connection-lifecycle-api-004** STMT/PROFILE multi-statement leftovers + other
   opportunistic one-holes (pragma_module_list, blob per-row expiry, util PRNG APIs).
3. **vtab-core-001 lifecycle deepen** — xConnect reload / drop_modules / xUpdate family.
