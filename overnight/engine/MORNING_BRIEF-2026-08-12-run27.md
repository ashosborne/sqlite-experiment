# MORNING BRIEF — engine v17: index lookups (run 27)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–26 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v17-index-lookups. MAX_NEW_CASES 40 (used 20).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @17 BOUND — index-lookup / explicit-shape / multi-leaf laws

versions/17.yaml + ADR 0015, schema-validated. WAL forbidden; no cost-based planner.

## 2. Lookup + index shapes implemented

- **Index-driven lookups:** eval's SELECT executor detects a single bare store table
  with a simple `col = / < / > / <= / >= / BETWEEN` predicate on an indexed column and
  fetches candidates through a BTreeMap built from the durable index entries, bumping a
  **probe counter** (anti-cheat). Result bytes match the scan path; the win is real
  index use.
- **Explicit shapes:** `IndexDef { exprs, unique, where_c, sql }` — multi-column
  `(a,b)`, expression `lower(nm)` (evaluated via eval), and partial `WHERE a > 10`
  indexes; `index_key_for` computes keys + honours the partial predicate; unique
  conflicts consult them on INSERT (NULL components stay distinct). Persist in
  sqlite_schema, reload on open.
- **Multi-leaf index b-trees:** entries exceeding one 0x0a leaf spill dividers up into
  a real 0x02 interior page. C integrity_check accepts 600- and 1000-key Rust indexes.
- **Honest EQP:** `SEARCH t USING INDEX i (col=?)` only when an index truly serves the
  WHERE column, else `SCAN t`. No fabricated BLOOM/AUTOMATIC-COVERING artifacts.

## 3. Interop results (mandatory gate — PASS)

`rust_write_c_index_lookup`: on a 600-row Rust file, C reports **integrity_check=ok**,
finds `k=432 → v432`, and **C's own planner plans `USING INDEX ik`** — proving the
b-tree is a real structure, not decoration. `anti_cheat_multileaf_index`: 1000-key
index, C integrity_check ok + `k00777 → 777`, Rust re-probes after reopen.
`anti_cheat_partial_or_expr_index`: partial-UNIQUE predicate + expression index both
C-readable and correctly enforced.

## 4. Cases: 20 frozen / 0 deferred

engine-idxlookup-001 (6: equality probe, range >, BETWEEN, UNIQUE-index equality,
point, join+probe) · engine-idxlookup-002 (6: multi-col index + pragma_index_list,
expression index lookup, partial index membership, multi-col UNIQUE dup error, DROP
expression index, partial UNIQUE predicate error) · engine-idxfile-001 (8 bespoke:
500-row reopen probe, multi-col UNIQUE reopen dup, expression reopen, partial UNIQUE
reopen, 600-key multi-leaf + integrity_check, DROP multi-col persisted, EQP SEARCH
detail, missing-key empty). All two-run deterministic, delegated HUMAN_ACCEPTED, all
replaying byte-identical.

## 5. ddl-schema-002 verdict: **FULL**

All three v12-residual gaps closed: index-driven lookups (probe-counter proven),
multi-column + expression + partial explicit indexes (durable, reopen-enforced,
C-readable), and multi-leaf index b-trees (integrity_check ok). Flipped to
`impl_in_modern: full`, `status: converted`, `parity: UNVERIFIED`. upsert-001 note
tightened (conflict targets now resolve to explicit UNIQUE indexes incl. multi-column;
index-*expression* conflict targets remain its sole gap — kept partial, not greenwashed).

## 6. Anti-cheat + cargo

anti-cheat + interop: `rust_write_c_index_lookup`, `anti_cheat_index_lookup_runtime`
(runtime key found via probe, missing key empty), `anti_cheat_partial_or_expr_index`,
`anti_cheat_multileaf_index` — all green. `cargo test` **401/401**; all 366 prior
goldens md5-identical; SCRIPT_TABLE still 0.

## 7. Scoreboard before → after

| State | run 26 | **run 27** |
|---|---|---|
| full | 75 | **79** (+ddl-schema-002, +engine-idxlookup-001/-002, +engine-idxfile-001) |
| partial | 51 | **50** |
| none | 103 | 103 |
| behaviours | 229 | 232 |

## 8. Explicit honesty line

**SQLite is NOT migrated.** 79 of 232 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified. No cost-based planner, no covering-index
optimization, no WAL.

## 9. Next call

(a) UTF-16 text/prepare APIs (prepare-statement-api-001's sole gap + column UTF-16),
(b) sqlite3_value / user-defined-function registration surface, or
(c) index-expression conflict targets to finish upsert-001. Pack v18 + goldens first.
