# MORNING BRIEF — engine v21: upsert expression conflict targets (run 31)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–30 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v21-upsert-expr-targets. MAX_NEW_CASES 40 (used 16).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @21 BOUND — expression-target / eval-consistency laws

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v20 → **v21**
(versions/1–21 retained; ADR `0019-engine-v21-upsert-expr-targets.md`; schema VALID; 25 laws).
New laws: **EXPRESSION-TARGET LAW** (structural resolution of `ON CONFLICT (<expr-list>)
[WHERE <pred>]` against UNIQUE indexes / PK-UNIQUE columns / UNIQUE table constraints;
mismatch and non-targeted-conflict errors as pinned) and **EVAL-CONSISTENCY LAW**
(target conflict detection reuses v17 `index_key_for` — no second string table).
WAL forbidden; SCRIPT_TABLE stays 0; `completeness: incomplete`.

## 2. Targets implemented (modern/src/store.rs)

Pre-run honesty note: the engine previously **discarded the conflict-target list**
entirely — every target acted as a catch-all and mismatches never errored. Now:

| Piece | Semantics |
| --- | --- |
| Target parse | `Stmt::Insert.target` = expression list + optional `WHERE` between `ON CONFLICT` and `DO` (balanced-paren, top-level commas) |
| `resolve_upsert_target` | normalize case/whitespace, match UNIQUE `IndexDef`s first (expression / multi-column / **partial**: target WHERE must structurally equal the index predicate), then PK/UNIQUE columns, then UNIQUE table constraints; no match → pinned C error (rc 1) |
| Targeted conflict | index targets via `unique_index_conflict` on exactly the resolved IndexDef → same `index_key_for` evaluation as v17 insert-time enforcement |
| Non-targeted conflict | aborts rc 19 with the qualified message (`UNIQUE constraint failed: t.c` / `index 'i1'`), pinned |
| Durability | expression UNIQUE indexes reload from file (v17 machinery) — upsert after reopen pinned |
| Catch-all / OR IGNORE / OR REPLACE | unchanged paths, re-pinned |

## 3. Frozen cases (16 new, two-run deterministic, delegated HUMAN_ACCEPTED)

| Feature | Cases | Pins |
| --- | --- | --- |
| engine-upsert-expr-001 | C001–C010 | lower(c) DO NOTHING / DO UPDATE excluded.*, arithmetic (a+b), multi-expr (lower(a),b), mismatch errors, catch-all, durable reopen, OR IGNORE/REPLACE, case/space folding, non-targeted conflict aborts rc 19 |
| engine-upsert-expr-002 | C001–C004 | regressions: column UNIQUE, multi-column index, INTEGER PRIMARY KEY targets, DO UPDATE ... WHERE guard |
| engine-upsert-expr-003 | C001–C002 | partial UNIQUE index: matching target WHERE works; absent/wrong WHERE → mismatch error |

RECORD run `2026-08-12T2000Z-legacy-record-upsert-expr`; catalog20.json; all 445 pre-run
goldens md5-verified intact.

## 4. upsert-001 verdict: **FLIPPED to full / converted**

Expression conflict targets — the card's sole named gap — work for the frozen scope,
including the partial-index WHERE stretch (Batch C frozen, not skipped). UNIQUE +
custom-collation targets stay unpinned (v20 residual; never part of this card's gap).
upsert-002 stays full, untouched. `parity: UNVERIFIED` — Verification has not run.

## 5. Anti-cheat + cargo

- `anti_cheat_upsert_expr_runtime` — pid-seeded values through lower() unique +
  ON CONFLICT DO UPDATE change the row (values absent from every golden).
- `anti_cheat_upsert_expr_mismatch` — runtime-named column target → C-matching
  mismatch error.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 488/488 PASS** (was 469; +16 golden twins, +3 anti-cheat/guard).
  Prior upsert / UNIQUE index / collation / UDF / UTF-16 suites all green.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 30 | Run 31 |
| --- | --- | --- |
| **full (converted)** | 88 | **92** |
| partial | 49 | 48 |
| none (remaining) | 103 | 103 |
| behaviours known | 240 | 243 |

Flips: upsert-001 partial→full; new full cards engine-upsert-expr-001/002/003.
legacy_green 150 → 153. parity_green 0. Nothing `verified`.

## 7. Not migrated

SQLite is **not migrated**. 92/243 behaviours run honestly in modern for frozen scope
only. No WAL, no planner, no VDBE, no UNIQUE+custom-collation upsert targets, no
covering-index claims. Parity UNVERIFIED everywhere (COMPARE never run).

## 8. Next call

1. **errmsg16 / create_collation16 / create_function16** — finish the UTF-16 API
   surface end-to-end (trivial wrappers over the run-29 codec + run-30/28 registries).
2. **Collation-aware index keys** — the honest residual on engine-collation-002.
3. **RETURNING clause** — a bounded, high-visibility DML surface if a new named card
   is preferred over residual-closing.
