# ADR 0015 — engine v17: index lookups (pack v17)

Date: 2026-08-12 · Status: BOUND (supersedes pack v16; v1–v16 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 27)

## Context
ddl-schema-002 was the longest-standing named partial (since the v12 disk-debt run).
Indexes were durable and UNIQUE enforced on reopen, but three gaps remained: no
index-driven lookups (only scans), no explicit multi-column/expression/partial
indexes, and no multi-leaf index b-trees.

## Decision
1. **Index-driven lookups.** A real probe path in the eval SELECT executor: a single
   bare store table with a simple `col = / < / > / <= / >= / BETWEEN` predicate on an
   indexed column fetches candidate rows through a BTreeMap built from the durable
   index entries, bumping a probe counter (anti-cheat proof). Results are identical to
   the scan path — the win is that lookups now genuinely use the index.
2. **Explicit index shapes.** `IndexDef { exprs, unique, where_c, sql }` replaces the
   old single-column tuple: multi-column `(a,b)`, expression `lower(nm)` (evaluated via
   eval), and partial `WHERE a > 10` indexes. `index_key_for` computes the key tuple
   and honours the partial predicate; unique conflicts consult these on INSERT (NULL
   components stay distinct). All persist in sqlite_schema with their CREATE sql and
   reload on open.
3. **Multi-leaf index b-trees.** When entries exceed one 0x0a leaf, divider entries
   move up into a 0x02 interior page (real file format); C integrity_check accepts a
   600- and a 1000-key index Rust wrote.
4. **Honest EQP.** EXPLAIN QUERY PLAN emits `SEARCH t USING INDEX i (col=?)` only when
   an explicit index actually serves the WHERE column, else `SCAN t`. No fabricated
   BLOOM FILTER / AUTOMATIC COVERING INDEX artifacts.

## Consequences
20 cases frozen (12 script + 8 file/EQP; two-run gate, delegated stamp), all replay
byte-identical. Interop: C integrity_check=ok and C's own planner uses the index on
Rust-written multi-leaf files; probe/partial/expr/multi-leaf anti-cheats pass.
cargo 401/401. ddl-schema-002 flips to full (index create/drop lifecycle, durable,
reopen-enforced, driven lookups, multi-column/expression/partial, multi-leaf). Still
out (other cards): cost-based planning, index-only covering optimization, collation
on index keys beyond binary. NOT migrated.
