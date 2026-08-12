# ADR 0019 — engine v21: upsert expression conflict targets

Status: accepted (pack v21 BOUND, Ash Osborne, delegated autonomy run 31)

## Context

`upsert-001` has been a standing partial since the v17 index run tightened it:
conflict targets resolved to PK / UNIQUE columns and explicit UNIQUE indexes, but
`ON CONFLICT (<expr>…)` against an expression UNIQUE index (e.g.
`CREATE UNIQUE INDEX i ON t(lower(c))` + `ON CONFLICT(lower(c))`) was the sole
named gap. The engine also silently ignored the conflict-target list entirely —
any target behaved like a catch-all, and mismatches did not error like C.

## Decision

1. **Target is parsed, not discarded.** `Stmt::Insert` carries
   `target: Option<(Vec<String>, Option<String>)>` — the expression list and the
   optional `WHERE` predicate between `ON CONFLICT` and `DO`.
2. **Structural resolution (sqlite3UpsertAnalyzeTarget spirit).**
   `resolve_upsert_target` normalizes case/whitespace and matches, in order:
   UNIQUE `IndexDef`s (expression / multi-column / partial — a target WHERE must
   structurally equal the index predicate), then single PK/UNIQUE columns, then
   UNIQUE table constraints. No match → the pinned C error
   `ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint` (rc 1).
3. **Eval consistency.** Targeted conflict detection for index targets goes through
   the same `index_key_for` expression evaluation as v17 insert-time unique
   enforcement (`unique_index_conflict` on exactly the resolved IndexDef) —
   no second string table.
4. **Non-targeted conflicts abort like C.** With a target resolved, a conflict on
   any OTHER constraint aborts with the qualified message
   (`UNIQUE constraint failed: t.c` / `index 'i1'`, rc 19) — pinned.
5. **Partial targets land (Batch C).** `ON CONFLICT(c) WHERE flag=1` matches the
   partial UNIQUE index; absent or wrong WHERE reproduces the mismatch error.
6. **Durability holds.** Expression UNIQUE indexes reload from the file
   (v17 machinery), so upsert-after-reopen replays byte-identical.

## Out of scope (unchanged)

UNIQUE + custom-collation targets (v20 residual, unpinned), covering-index planner
claims, EXPLAIN honesty for upsert index choice, WAL.

## Consequences

16 goldens (engine-upsert-expr-001 ×10, -002 ×4 regression pins, -003 ×2 partial)
replay byte-identical; runtime anti-cheats prove resolution and mismatch. Card
`upsert-001` honestly flips to full; `upsert-002` stays full untouched.
