# MORNING BRIEF — engine v14: thin-gap harvest #2 (run 24)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–23 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v14-thin-gap-2. MAX_NEW_CASES 40 (used 29).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @14 BOUND

versions/14.yaml + ADR 0012, schema-validated; all prior laws carried. One deliberate
honesty decision recorded in the pack: the two-table **EXPLAIN QUERY PLAN join case was
NOT frozen** — C's output exposes its cost-based planner (BLOOM FILTER, AUTOMATIC
COVERING INDEX) and reproducing that text from a nested-loop engine would be a fake
planner essay. EQP is pinned only where this engine's honest plan (SCAN t) is the truth.

## 2. Scoreboard before → after

| State | run 23 | **run 24** |
|---|---|---|
| full | 53 | **66** |
| partial | 57 | **52** (6 notes tightened) |
| none | 103 | 103 |
| behaviours | 213 | 221 (+8 harvest slices) |

## 3. Prepare cards — final verdicts

| Card | old → new | Why |
|---|---|---|
| 001 prepare family | partial → **partial** (UTF-16 only) | prepare_v3 + prepFlags real (PERSISTENT/NO_VTAB honest no-ops); UTF-16 is the sole remaining gap |
| 005 reset/reprepare | partial → **full** | auto-reprepare on schema change real: DDL under a live stmt re-resolves; DROP → step SQLITE_ERROR "no such table" as pinned |
| 006 introspection | partial → **partial** (tighter) | readonly/busy + honest EQP + EXPLAIN 8-column shape real; bytecode listing honestly absent (no VDBE) |

## 4. Tier B verdicts

**→ full (4):** printf-format-001 (comma grouping + %p close the last holes) ·
upsert-002 (multi-assignment SET with expressions over excluded.* + WHERE) ·
triggers-002 (RAISE(IGNORE) row-skip, FAIL/ROLLBACK rc19, INSTEAD OF UPDATE/DELETE
with real view projections — base tables untouched exactly as C pins) ·
serialize-memdb-api-001 (populated images via the shared dbfile writer/reader; all
five storage classes round-trip).

**stayed partial, tighter (4):** window-functions-001 (rank/dense_rank/lag/lead +
framed aggregates + PARTITION BY real; first_value/ntile family absent) ·
window-functions-002 (RANGE-peers/ROWS/GROUPS real; EXCLUDE + offset RANGE absent) ·
misc-decimal-001 (decimal_exp closed; arbitrary precision still i128-bounded) ·
printf-format-003 (appendchar/reset/length/value + empty-finish NULL; raw append +
vappendf varargs absent). printf-format-002 unchanged (vmprintf needs a varargs ABI).

**New full (8):** engine-window2 / raise / upsert3 / printf3 / decimal2 / prepare2 /
serialize2 / str2 -001.

## 5. What was implemented

General window engine (partitions, stable window ordering matching C's tie behaviour,
RANGE-with-peers default frame, ROWS/GROUPS frames, 10 functions); trigger RAISE family
with a per-row proceed flag; INSTEAD OF UPDATE/DELETE over real view projections;
upsert multi-assignment evaluated against excluded env; printf `,` grouping + %p;
decimal_exp (+ one-fraction-digit rendering rule discovered from C bytes);
sqlite3_prepare_v3; auto-reprepare via the store schema counter; EQP/EXPLAIN
statements; populated serialize/deserialize sharing the v12 file-format writer/reader;
sqlite3_str appendchar/reset/length/value, finish→NULL when empty.

## 6. Cases / anti-cheat / cargo

29 new goldens (17 script + 12 bespoke), two-run deterministic, delegated stamp, all
replay byte-identical (script replays via exec; bespoke via the statement API).
Anti-cheat **18/18** (new: runtime window rank/RANGE-peer sums; runtime upsert
v=v+excluded.v; runtime printf grouping). `cargo test` **329/329**; all 269 prior
goldens md5-identical; kitchens, file suites, interop unchanged green.

## 7. Explicit honesty line

**SQLite is NOT migrated.** 66 of 221 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified. No transactions, no VDBE, no planner.

## 8. Next call

(a) UTF-16 prepare/text APIs (finishes prepare-001 + column UTF-16 notes),
(b) index-driven lookups + multi-column explicit indexes (ddl-schema-002 focused loop),
(c) transactions (BEGIN/COMMIT/ROLLBACK — unlocks OR ROLLBACK, dml-codegen-002, and
savepoint surface). Pack v15 + goldens first.
