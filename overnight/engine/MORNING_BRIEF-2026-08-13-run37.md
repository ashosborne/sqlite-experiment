# MORNING BRIEF — engine v27: ANALYZE → sqlite_stat1 (run 37)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–36 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v27-analyze, no deepening of
WAL/VACUUM/blob/conn. MAX_NEW_CASES 55 (used 17; stretch skipped — ANALYZE depth was
the run). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @27 BOUND — ANALYZE law; planner load explicitly NOT claimed

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v26 → **v27**
(versions/1–27 retained; ADR `0025-engine-v27-analyze.md`; schema VALID; 33 laws).
Plain language: run ANALYZE and real table/index scans write row-count and
selectivity numbers into sqlite_stat1 in C's exact text format. **Writing stats is
claimed; using them is not** — no plan-shape pin exists, so the planner-load card
stays none rather than faking EXPLAIN QUERY PLAN changes.

## 2. STAT4 decision

`sqlite_compileoption_used('ENABLE_STAT4')` = 0 on the pinned CLI and the bare
amalgamation harness build. Everything claimed is sqlite_stat1 only; sqlite_stat4
is a named residual.

## 3. What ANALYZE now does in modern

| Behaviour | Evidence |
| --- | --- |
| Real selectivity text | per-index `N d1 d2…` computed from actually-evaluated key tuples (the v17 `index_key_for` machinery); multi-column prefixes pinned (`6 3 2`) |
| **The rounding-quirk pin** | 11 rows / 10 distinct renders `11 1`, not the naive ceiling's `11 2` — C's near-1.0 collapse, ported exactly; a guessed formula fails this golden |
| Shape rules | empty tables write no row (sqlite_stat1 still created); index-less tables get one NULL-idx row; indexed tables get one row per index and no NULL row |
| WITHOUT ROWID | the PRIMARY KEY appears as an index named like the table (`w|w|2 1`, pinned) |
| Scoping | `ANALYZE` / `ANALYZE main` / `ANALYZE <table>` / `ANALYZE <index>` (exactly that index's row) all pinned; re-ANALYZE replaces the scope's rows |
| DROP maintenance | DROP INDEX / DROP TABLE clear the matching stat1 rows (pinned) |
| Durable + interop | sqlite_stat1 is an ordinary catalog table: survives reopen (integrity ok) and VACUUM, works on WAL files, and round-trips with C **both directions** (pinned CLI reads Rust's stats; modern reads a C ANALYZE's rows) |

Also fixed: a latent flake in the run-30 collation twins (the two collation_needed
tests shared capture globals across threads) — now serialized; 8 consecutive clean runs.

## 4. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| analyze-stats-001 | none (legacy_green only) | **partial** | sqlite_stat4; PRAGMA optimize history; attached-schema stats; sz=/unordered annotation tokens (never emitted by pinned data) |
| analyze-stats-002 | none | **none (kept)** | planner cost model not claimed this pack — modern writes stats but does not load them; flipping without plan pins would be greenwash |
| engine-analyze-001/002 | — | **new full ×2** | composed cards for exactly the frozen batches |

## 5. Anti-cheat + C interop + cargo

- `anti_cheat_analyze_runtime` — a pid-seeded table with a runtime-chosen row count
  and duplicate pattern: the stat1 row must carry the runtime table name AND the
  correctly computed `N d` integers (quirk included). A script table cannot know them.
- `rust_analyze_c_read` — the pinned C CLI reads Rust's sqlite_stat1 (`f|ifa|4 2`,
  integrity ok) **and** modern reads rows a C ANALYZE wrote (`g|igz|2 1`).
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 609/609 PASS** (was 589; +17 golden twins, +3 anti-cheat/interop/guard).
  engine-conn / blob / vacuum / wal / upsert-expr / collation / utf16 / harvest23 green;
  572 pre-run goldens md5-verified intact.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 36 | Run 37 |
| --- | --- | --- |
| **full (converted)** | 118 | **120** |
| partial | 51 | 52 |
| none (remaining) | 94 | **93** |
| behaviours known | 263 | 265 |

legacy_green 173 → 175. parity_green 0. Nothing `verified`. Prior claims unchanged.

## 7. Not migrated

SQLite is **not migrated**. 120/265 behaviours run honestly in modern for frozen
scope only. Stats are written, not used: there is no cost model, no STAT4, no
planner claim. Parity UNVERIFIED everywhere (COMPARE never run).

## 8. Next call

1. **get_table / status crumbs** — exec-convenience-api-002 + error-status-api-003.
2. **auth-callback-api-002** — column-read IGNORE → NULL.
3. **ANALYZE deepen** — honest planner load for analyze-stats-002 (requires real
   stats-driven index choice + EQP pins), or STAT4 if the pin build ever grows it.
