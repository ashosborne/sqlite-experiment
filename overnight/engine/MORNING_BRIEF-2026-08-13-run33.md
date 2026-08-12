# MORNING BRIEF — engine v23: thin-gap harvest (run 33)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–32 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v23-thin-gap-harvest, DEEPEN_WAL: false.
MAX_NEW_CASES 40 (used 24, as 33 case-ids across 9 batches). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @23 BOUND — thin-gap harvest law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v22 → **v23**
(versions/1–23 retained; ADR `0021-engine-v23-thin-gap-harvest.md`; schema VALID; 29 laws).
New law: harvest increments must pin the previously-MISSING behaviour, flip full only when
the COVERAGE-named residual is honestly gone, and REAL text rendering must stay on the
ported FpDecode pipeline. WAL claims unchanged from v22. SCRIPT_TABLE stays 0.

## 2. Flips table (card → before → after → residual)

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| loadext-api-002 | partial | **full** | — (cancel/reset/multi-entry landed) |
| malloc-subsystem-001 | partial | **full** | — (memory_used/highwater on the real allocator) |
| window-functions-001 | partial | **full** | — (all six leftovers + named WINDOW clause) |
| error-status-api-002 | partial | **full** | — (limit id matrix + prepare-time enforcement) |
| error-status-api-001 | partial | **full** | extended codes real for implemented error paths; IOERR/CANTOPEN families have no modern error source |
| foreign-keys-001 | partial | **full** | — (deferred FKs: COMMIT check, txn stays open, pragma reset) |
| printf-format-002 | partial | partial (tighter) | vmprintf needs C va_list — stable Rust cannot define it (platform residual) |
| printf-format-003 | partial | partial (tighter) | vappendf: same va_list residual; str_append landed |
| auth-callback-api-001 | partial | partial (tighter) | s1–s4 args, SQLITE_IGNORE, ~28 more action codes |
| tokenizer-002 | partial | partial (tighter) | string-literal-aware lexing in complete() |
| engine-harvest23-001..009 | — | **new full ×9** | composed cards for exactly the frozen batches |

## 3. What landed (modern/src/{lib,eval,store,fpdec}.rs)

- **auto-extension**: ordered multi-entry registry, cancel (1/0), reset, duplicate collapse.
- **malloc accounting**: counters in `sized_alloc` (origin of every `sqlite3_free` pointer);
  sticky highwater with reset-returns-prior semantics.
- **snprintf** (fixed-arity like the existing mprintf): truncation with NUL at n-1,
  n<=0 no-op returning buf; **sqlite3_str_append** raw n-limited bytes.
- **window functions**: first_value/last_value/nth_value (frame-aware, OOR→NULL),
  ntile (front-loaded buckets), percent_rank, cume_dist; `WINDOW <name> AS (...)` named
  windows; `ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING` frame.
- **sqlite3_limit**: full id matrix (defaults pinned: LENGTH 1e9, COLUMN 2000,
  FUNCTION_ARG 1000, ATTACHED 10, VARIABLE_NUMBER 32766), prior-value sets, compile-max
  clamping; VARIABLE_NUMBER enforced at prepare (`variable number must be between ?1 and ?N`).
- **errors**: sqlite3_errstr (extended codes fall through to base, pinned by e787/e2067);
  extended constraint codes 2067/1299/275/787 on real paths; UNIQUE/CHECK messages now
  qualified like C (`UNIQUE constraint failed: t.a`, `CHECK constraint failed: c>0`).
- **deferred FKs**: `DEFERRABLE INITIALLY DEFERRED` + `PRAGMA defer_foreign_keys`
  (resets at txn end); COMMIT-time whole-store validation; failed COMMIT keeps the
  transaction open (pinned); still immediate outside transactions.
- **authorizer**: INSERT/UPDATE/DELETE/CREATE_TABLE/PRAGMA deny → rc 23 "not authorized".
- **complete()**: real BEGIN/CASE/END nesting scan.

### The unplanned deep fix: `fpdec.rs`

Window pins exposed that modern rendered REALs by Rust shortest-round-trip while C
renders 1.0/3.0 as `0.33333333333333332` (SQLite's own 18-digit convert + round-to-17
artifact). `fpdec.rs` is a **faithful port of sqlite3FpDecode / Fp2Convert10 /
Fp10Convert2** (power-of-ten tables, 128-bit multiplies, %!.17g precision-reduction)
plus the printf %!g assembly. Every prior REAL pin replays through the port.

## 4. Anti-cheat + cargo

- `anti_cheat_harvest_runtime` — pid-seeded rows through window functions, a runtime
  VARIABLE_NUMBER limit with exact C errmsg, and a runtime deferred-FK commit cycle.
- Per-card mandatory pins demonstrate the previously-missing behaviour (e.g. wal-blind
  none of these — WAL untouched).
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 519/519 PASS** (was 508; +11 harvest tests). engine-wal / upsert-expr /
  collation / utf16 / prepare / index / UDF suites all green; 477 pre-run goldens
  md5-verified intact.

## 5. Scoreboard (impl_in_modern) — before → after

| State | Run 32 | Run 33 |
| --- | --- | --- |
| **full (converted)** | 94 | **109** |
| partial | 50 | 44 |
| none (remaining) | 101 | 101 |
| behaviours known | 245 | 254 |

6 umbrella Partials flipped full; 4 tightened honestly; 9 composed batch cards added.
legacy_green 155 → 164. parity_green 0. Nothing `verified`. WAL claim level unchanged.

## 6. Not migrated

SQLite is **not migrated**. 109/254 behaviours run honestly in modern for frozen scope
only. va_list ABI surfaces (vmprintf/vappendf) are platform residuals on stable Rust.
Parity UNVERIFIED everywhere (COMPARE never run).

## 7. Next call

1. **Another harvest** — auth s1–s4 args + SQLITE_IGNORE; tokenizer string-aware
   complete(); json_valid flags; connection-lifecycle URI thin pins.
2. **WAL deepen** — frame-level appends to shrink the wal-001 residual.
3. **A named none** — e.g. RETURNING clause as a new bounded DML card.
