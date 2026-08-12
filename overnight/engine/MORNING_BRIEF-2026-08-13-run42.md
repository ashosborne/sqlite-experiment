# MORNING BRIEF — engine v32: compile-option diagnostics (run 42)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–41 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v32-compile-options, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 19). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @32 BOUND — COMPILE-OPTIONS law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v31 → **v32**
(versions/1–32 retained; ADR `0030-engine-v32-compile-options.md`; schema VALID; 38 laws).

## 2. Presence checks (the whole point of this run)

- **compileoption diagnostics PRESENT** on the pinned bare build: `sqlite3_compileoption_get`
  enumerates **38 options** (`ATOMIC_INTRINSICS=1` … `THREADSAFE=1`), `used()` handles
  plain / `SQLITE_`-prefixed / `=value` / case-insensitive forms. The `COMPILER=gcc-13.3.0`
  row is frozen as a **pin decision** (ADR 0030): the table is the pinned C baseline's
  fingerprint, not a claim about modern's toolchain.
- **`generate_series` ABSENT** from the bare amalgamation (it's ext/misc) — the planned SQL
  census-count pin was dropped rather than frozen against a conflicting oracle.
- **unlock-notify presence check FAILED**: `used("ENABLE_UNLOCK_NOTIFY")=0` on the pin →
  `unlock-notify-api-001` stays **none**, zero cases, noted in ADR + manifest.

## 3. What landed

| Behaviour | Evidence |
| --- | --- |
| C API | `sqlite3_compileoption_used` / `sqlite3_compileoption_get` answer from the pinned 38-entry table; unknown/empty → 0; past-end/negative → NULL |
| Matching rule | `THREADSAFE`/`SQLITE_THREADSAFE`/`THREADSAFE=1` → 1, `THREADSAFE=0` → 0; `DEFAULT_AUTOVACUUM=1` → 0 (bare gate, `=` boundary); `threadsafe`/`ThreadSafe=1` → 1 (case-insensitive) |
| SQL twins | `sqlite_compileoption_used` / `sqlite_compileoption_get` registered in the evaluator; typeof pins integer/text/null |
| OMIT census | 9 probed `OMIT_*` gates all 0 (incl. charter's `OMIT_AUTORESET` and the self-gate `OMIT_COMPILEOPTION_DIAGS`); zero `OMIT_` entries enumerated |
| ENABLE census | 8 probed `ENABLE_*` gates all 0 (incl. charter's `ENABLE_API_ARMOR`); zero `ENABLE_` entries enumerated |
| Cross-checks | `MAX_ATTACHED=10` / `MAX_VARIABLE_NUMBER=32766` / `TEMP_STORE=1` rows agree with limits pinned in earlier runs |

## 4. Flips table

| Card | Before | After | Why |
| --- | --- | --- | --- |
| compile-options-omit-enable-001 | none | **full** | C + SQL diagnostics real against the pinned fingerprint; residual-free for the pinned seam (self-gated OMIT_COMPILEOPTION_DIAGS builds out of scope by law) |
| compile-options-omit-enable-002 | none | **partial** | pinned OMIT census (all probes 0, zero OMIT_ entries); 77-guard per-feature census NOT claimed |
| compile-options-omit-enable-003 | none | **partial** | pinned ENABLE census (all probes 0, zero ENABLE_ entries); 51-guard per-feature census NOT claimed |
| unlock-notify-api-001 | none | none (note) | presence check failed on the pin |
| engine-compile32-001/002/003 | — | **full** (composed) | exact frozen batches (11 + 4 + 4 cases) |

## 5. Anti-cheat + goldens

- 19 new HUMAN_ACCEPTED goldens (`tests/characterization/engine-compile32/`), two-run
  deterministic; prior 667 goldens untouched.
- 2 anti-cheat tests: runtime-generated fake option name → 0 through both C API and SQL
  twin; C/SQL enumeration round-trip agrees entry-for-entry, terminates at the same index
  (38), and every enumerated entry reports `used()=1`.
- SCRIPT_TABLE.len()==0; no cheat sheet.

## 6. cargo

`cargo test` (modern): **717 passed / 0 failed** (was 696; +21 compile32 twins/anti-cheat).
vtab31 / attach30 / none29 / harvest / ANALYZE / conn / blob / vacuum / WAL all green.

## 7. Scoreboard

before → after: **137 full / 60 partial / 82 none of 279** → **141 full / 62 partial / 79 none of 282**.

## 8. Not migrated

SQLite is NOT migrated. This run pinned the *diagnostics/census seam only* — the behaviour
of gated features is untouched; no OMIT/ENABLE build variants are modelled; unlock-notify,
planner, xBestIndex pushdown, WAL depth, wasm/jni/vfs/FTS/rtree/session all remain out.

## 9. Next call (pick one)

1. **xBestIndex deepen** — real constraint offers to vtab modules (EQ pushdown + HIDDEN
   argv path), cutting the vtab-core-002 residual.
2. **attached-trigger firing** — the run-40 residual (unqualified trigger-body resolution
   at execution inside an attached schema).
3. **status deepen** — sqlite3_status/status64 matrix beyond what conn/malloc runs pinned.
