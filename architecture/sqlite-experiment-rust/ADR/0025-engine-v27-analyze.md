# ADR 0025 — engine v27: ANALYZE → sqlite_stat1

Status: accepted (pack v27 BOUND, Ash Osborne, delegated autonomy run 37)

## Context (plain language)

ANALYZE walks each table and index, counts rows and how selective each index
prefix is, and writes those numbers into a small table called sqlite_stat1 so the
query planner can read them later. analyze-stats-001 had a C golden since the
recognizer era but nothing real in modern; analyze-stats-002 (loading the stats
into the planner) had nothing at all.

## STAT4 decision

`sqlite_compileoption_used('ENABLE_STAT4')` = 0 on the pinned CLI and the bare
amalgamation harness build. Everything this pack claims is sqlite_stat1 only;
sqlite_stat4 is a named residual.

## What modern now does

1. **Real scans → real text.** For every index, the stat string is computed from
   actually-evaluated key tuples (the v17 `index_key_for` machinery): row count,
   then ceil(nRow / distinct-k-prefixes) per column — including C's near-1.0
   rounding quirk, pinned by the 11-rows/10-distinct case that renders `11 1`
   where the naive ceiling says 2. A formula guess would fail that golden.
2. **The pinned shape rules.** Empty tables write no row (but sqlite_stat1 is
   created); tables without indexes write one NULL-idx row holding the count;
   indexed tables write one row per index and no NULL row; WITHOUT ROWID primary
   keys appear as an index named like the table (`w|w|2 1`).
3. **Scoping.** `ANALYZE`, `ANALYZE main`, `ANALYZE <table>`, and
   `ANALYZE <index>` (which touches exactly that index's row) all pinned.
   Re-ANALYZE replaces the scope's rows; DROP INDEX / DROP TABLE clear theirs.
4. **Ordinary durable table.** sqlite_stat1 lives in the store as a normal
   catalog table, so it persists through the shared dbfile writer, survives
   reopen (integrity ok) and VACUUM, works on WAL files without deepening the
   WAL claim, and round-trips with C in both directions: the pinned CLI reads
   Rust's stats, and modern reads stats a C ANALYZE wrote.

## Planner-load honesty (analyze-stats-002)

Zero plan-shape cases were frozen. Modern's EXPLAIN QUERY PLAN output is its own
honest nested-loop text and does not consult sqlite_stat1; pinning C's
stats-driven EQP changes would have required implementing a cost model this run
did not attempt. **analyze-stats-002 stays none** with the note "planner cost
model not claimed this pack" — writing stats is claimed, using them is not.

## Deliberate residuals (not claimed)

sqlite_stat4 / STAT4 sampling; PRAGMA optimize's selective-ANALYZE history;
attached-schema stats; `sz=NNN` and `unordered` annotation tokens (not emitted by
this pin's data); stat1 load-time validation of hand-corrupted rows; planner use.

## Consequences

17 goldens replay byte-identical; anti-cheat proves runtime tables/counts land in
stat1 with correctly computed selectivities; two-direction C interop green; cargo
609/609. analyze-stats-001 flips none→**partial** (write scope real; STAT4 /
optimize / attached / annotations residual); analyze-stats-002 stays **none**.
Stretch batches skipped — ANALYZE depth was the run.
