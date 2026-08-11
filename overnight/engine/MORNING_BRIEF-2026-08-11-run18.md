# MORNING BRIEF — engine v9: joins + scalar subqueries (run 18)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–17 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v9-joins-subqueries, MAX_NEW_CASES 36 (used 24).

## 1. Pack @9 BOUND — subquery + join law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v8 → **v9 BOUND** (Ash Osborne,
delegated). versions/1–9 retained; ADR 0007; schema-validated. New laws: **subquery law**
(scalar/EXISTS/correlated, LIMIT inside, evaluated from real row sources), **join law**
(nested-loop INNER/comma/LEFT over 2–3 real store tables; no planner claim), anti-cheat with
runtime literals. `in_scope_cases` 174; `query_path_cases_v9` 25. completeness: **incomplete**.

## 2. New cases — 24 frozen / 0 deferred

All 24 passed the two-run determinism gate on the pinned C library first try, were
delegated HUMAN_ACCEPTED, and **all 24 replay byte-identical through the Rust executor**
(no case needed deferral).

| Feature | Cases | Shapes |
|---|---|---|
| `engine-subquery-001` | C001–C005 | SELECT-list scalar `(SELECT max(b) FROM t2)`; WHERE scalar; LIMIT + LIMIT/OFFSET inside subquery; pragma_database_list twin; subquery-in-FROM + ORDER BY + LIMIT |
| `engine-subquery-002` | C001–C005 | EXISTS true/false; correlated NOT EXISTS (empty-ish result); correlated scalar in WHERE with LIMIT; correlated SELECT-list scalar; fresh literal **900017** + empty-subquery→NULL |
| `engine-join-001` | C001–C007 | JOIN…ON both-sides projection; INNER JOIN count; comma join + WHERE; AS aliases; no-match empty; non-equi ON (`q.a > p.a`); fresh-literal join **910033** |
| `engine-join-002` | C001–C007 | three-table A⋈B⋈C; self-join `n p, n q`; join aggregates (count/min/max); join + **GROUP BY**; **LEFT JOIN** row shape (NULL-extended); `count(*)` vs `count(r.t)` over LEFT JOIN; single-table GROUP BY |

## 3. v8 defers: reclaimed vs still deferred

- **Reclaimed: `pragma-surface-002-C001`** (scalar subquery + LIMIT over pragma_database_list)
  — replays green through the executor against its **untouched** golden; removed from the
  pack defer list. oneshot suite grew 50 → 51.
- **Still deferred (18)**, reasons unchanged: 4 date/time pins, ANALYZE, VACUUM, dbstat,
  csv/completion/wholenumber vtabs, zlib compress, fossil delta, next_char, eval(),
  cross-schema trigger, 2 C-registry-count pins, decimal_mul formatting. None of these is
  blocked on subquery/join shape.

## 4. What the executor now implements (modern/src/eval.rs)

- **Subqueries:** quote/paren-aware string extraction → `Ex::Subq` / `Ex::Exists` nodes;
  scalar = first row/col (empty → NULL); EXISTS = non-empty; **correlated** outer-row
  bindings threaded through items/WHERE/ON (inner columns shadow outer).
- **Joins:** FROM parser normalizes INNER/LEFT [OUTER]/CROSS at top level, splits items
  with optional `[AS]` aliases and `ON` expressions; **nested-loop** fold over store
  snapshots; comma join = cartesian + WHERE; **LEFT JOIN** emits NULL-extended right
  columns for unmatched left rows. Rows carry qualified keys (`e.name`) + bare fallback.
- **GROUP BY:** real grouping (multi-key capable), aggregates per group, first-seen order
  (pinned cases always ORDER BY).
- **ORDER BY:** multi-key, ASC/DESC, output-name / qualified-name / ordinal resolution.
- **LIMIT/OFFSET** at any select level (top level and inside subqueries).
- INNER-only claim **plus** honestly-implemented LEFT JOIN; no RIGHT/FULL, no planner,
  no index use.

## 5. Anti-cheat results (anti_cheat_v8.rs, v9 section) — 8/8 green

- `anti_cheat_join_runtime` — runtime key {pid/time}: two tables, INNER JOIN → computed row. ✅
- `anti_cheat_scalar_subquery_runtime` — `(SELECT max(v))`, `(SELECT v ORDER BY v LIMIT 1)`,
  `(SELECT n+1)` with runtime n. ✅
- `anti_cheat_script_table_still_empty` — SCRIPT_TABLE.len() == 0. ✅ (grep proof: 0 `("` pins)
- prior v8 anti-cheat (expr/pragma/json/unknown-SQL) still green. ✅

## 6. Test suite

`cargo test` — **164/164 green**: spine 11, bespoke 28+5+20+3, kitchen 6 (memory), files 8 +
interop 8 (C reads Rust files, integrity_check=ok — unchanged), oneshot 51 (incl. reclaimed
pin), query_compare 24 (new), anti-cheat 8. All 155 prior golden files md5-identical; the
only new files are the 24 RECORD additions. legacy_green 104; parity_green **0**.

## 7. Catalogue impact

Multi-table SQL now executes in Rust: the join/subquery shapes unlock realistic queries
across the kitchen slices (select-codegen, where-optimizer surface shapes, name-resolution,
expr) — approx 6–8 slices move from "single-table only" to "multi-table SQL runs". The
where-optimizer / pager / btree / vfs cards stay amber: nested-loop is **not** a planner.

## 8. Leftover (still not a full SQL engine)

cost-based planner + index selection · RIGHT/FULL OUTER · date/time engine · WAL/crash
recovery · scalar subqueries in ORDER BY/GROUP BY positions · HAVING · correlated EXISTS
depth beyond pinned shapes · remaining 18 v8 defers · overflow pages + on-disk UNIQUE
autoindexes (v7 debt).

completeness: **incomplete** — SQLite is NOT migrated.

## 9. Next call

(a) HAVING + subqueries in more positions + date/time engine (clears 4 defers), or
(b) on-disk debt: overflow pages + UNIQUE autoindexes + file-path join twins, or
(c) WAL law change. Pack v10 + goldens first, either way.
