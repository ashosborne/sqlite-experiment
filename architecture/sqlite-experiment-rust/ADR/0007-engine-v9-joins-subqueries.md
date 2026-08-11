# ADR 0007 — engine v9: joins + scalar subqueries (pack v9)

Date: 2026-08-11 · Status: BOUND (supersedes pack v8; v1–v8 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 18)

## Context

Pack v8 killed the cheat sheet: the executor evaluates single-table SELECTs, UNION,
expressions, functions and pragmas. But scalar subqueries with LIMIT and multi-table
joins were still out — the biggest gap between a toy executor and the SQL real
applications write. One v8 defer (pragma-surface-002-C001) was blocked on exactly
that shape.

## Decision

1. **Subquery law.** `(SELECT ...)` in SELECT lists and WHERE — including LIMIT/OFFSET
   inside the subquery, EXISTS / NOT EXISTS, and correlated references to the outer
   row — is evaluated by the executor from real row sources. Implementation: string-level
   subquery extraction (quote/paren aware) → `Ex::Subq`/`Ex::Exists` AST nodes → recursive
   `select_rows_o` with outer-row bindings threaded through item/WHERE/ON evaluation.
2. **Join law.** INNER JOIN / comma-join with ON or WHERE equi-join (plus simple
   non-equi filters) over two or three real store tables returns C-matching rows via
   **nested-loop** execution over store snapshots. LEFT OUTER JOIN is implemented for
   real (NULL-extended unmatched left rows) and frozen in the v9 shapes. Explicitly NOT
   claimed: cost-based planner, index selection, RIGHT/FULL joins.
3. **Supporting shapes implemented for real:** `[AS]` aliases with qualified column
   resolution (`t1.a` first, bare `a` fallback), real GROUP BY (grouped aggregate
   evaluation in first-seen order, always paired with ORDER BY in pinned cases),
   multi-key ORDER BY with ASC/DESC, LIMIT/OFFSET at any select level.
4. **24 new cases frozen on the pinned C library** (two-run determinism, delegated
   HUMAN_ACCEPTED): engine-subquery-001/-002 (10), engine-join-001/-002 (14), including
   fresh literals 900017 / 910033 that exist in no other golden.
5. **Reclaimed defer:** pragma-surface-002-C001 (scalar subquery + LIMIT over
   pragma_database_list) replays through the executor against its untouched v8 golden.
6. **Anti-cheat:** runtime-keyed join and runtime scalar subquery tests; SCRIPT_TABLE
   asserted still empty.

## Consequences

- cargo suite 164/164; all prior goldens byte-identical; memory + file kitchens green.
- Still NOT a query planner; no indexes consulted; no OUTER beyond frozen LEFT shapes;
  date/time and remaining v8 defers unchanged; no WAL. SQLite is NOT migrated.
