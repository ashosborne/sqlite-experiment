# ADR 0012 — engine v14: thin-gap harvest #2 (pack v14)

Date: 2026-08-12 · Status: BOUND (supersedes pack v13; v1–v13 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 24)

## Context
53 full / 57 partial after run 23. Three prepare cards and several near-full Partials
(windows, RAISE family, upsert breadth, printf, decimal_exp, populated serialize)
blocked completion with bounded gaps.

## Decision
1. **General window engine** replacing the two pinned shapes: PARTITION BY, window
   ORDER BY (stable, DESC ties preserve source order like C), frames (default
   RANGE-with-peers, ROWS UNBOUNDED/N PRECEDING, GROUPS N PRECEDING), functions
   row_number/rank/dense_rank/lag/lead/sum/min/max/avg/count — computed per source
   row from real partitions and frames.
2. **Trigger semantics completed:** RAISE(IGNORE) skips the row operation via a
   proceed flag; RAISE(FAIL/ROLLBACK) fail with rc 19 (no transaction stack — not
   faked); INSTEAD OF UPDATE/DELETE fire against real view projections with
   old./new. envs, leaving base tables untouched exactly as C pins show.
3. **Upsert breadth:** DO UPDATE SET is a real multi-assignment list whose
   expressions evaluate against excluded.* + the existing row (v = v + excluded.v).
4. **printf** comma-grouping flag + %p (uppercase hex); **decimal_exp** in the
   extension's +D.De±EE form (one fraction digit minimum, matching pow2 rendering).
5. **Statement API leftovers:** sqlite3_prepare_v3 exported (prepFlags accepted;
   PERSISTENT/NO_VTAB are honest no-ops); auto-reprepare on schema change via the
   store's schema counter (DDL under a live statement re-resolves; DROP under a live
   statement steps to SQLITE_ERROR "no such table" like C); EXPLAIN QUERY PLAN
   returns THIS engine's honest plan (nested-loop SCAN); the C planner-artifact EQP
   join case (BLOOM FILTER / AUTOMATIC COVERING INDEX) was deliberately NOT frozen —
   reproducing it would be a fake planner essay. EXPLAIN exposes the real 8-column
   shape; the bytecode listing is honestly absent (no VDBE).
6. **Serialize/deserialize populated:** sqlite3_serialize builds a REAL SQLite image
   of the live store via the shared dbfile writer; deserialize parses with the shared
   reader and loads the store — all five storage classes round-trip.
7. **sqlite3_str completed:** appendchar/reset/length/value; finish returns NULL for
   an empty builder (pinned).

## Consequences
29 cases frozen (17 script + 12 bespoke; two-run gate, delegated stamp), all replay
byte-identical. cargo 329/329; anti-cheat 18/18. Honestly still out: UTF-16 prepare,
EXPLAIN bytecode rows, transactions (OR ROLLBACK semantics), PARTITION-BY-free
window breadth beyond pins, arbitrary-precision decimal. NOT migrated.
