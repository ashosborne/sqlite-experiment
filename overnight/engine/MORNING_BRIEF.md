# MORNING BRIEF — engine v13: prepare/bind through the real engine (run 23)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–22 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v13-prepare-bind. MAX_NEW_CASES 40 (used 26).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @13 BOUND — statement-API / bind / column laws

versions/13.yaml + ADR 0011, schema-validated. prepare must execute via the SAME
store/eval engine as sqlite3_exec — pin tables and per-golden state machines banned;
typed binds must provably affect results; column accessors cover the frozen matrix
with real coercions. completeness: **incomplete**.

## 2. How prepare executes (plainly)

**Shared engine, not a mini-VDBE.** prepare slices the first statement (pzTail),
scans ?, ?N and :name parameters, and resolves names at prepare time exactly where C
does — a side-effect-free dry run of SELECTs with NULL parameters yields
"no such table:" / "no such function:" errors and the column-name list; DML targets
are checked against the catalog. step substitutes typed bound values into the
statement and runs it through the same engine as exec: SELECT/PRAGMA materialize
typed rows at first step (nested-loop eval — **not** a bytecode VDBE), DML/DDL run
the script engine with real constraint codes (19). Autoreset (OMIT_AUTORESET=off)
re-executes after DONE; reset keeps bindings and discards rows. The three legacy
recognizer pins (SELECT 1 / SELECT ? / '42abc') now pass through this real path
byte-identically — the recognizer is gone.

## 3. Bind / column coverage

| Family | Real now |
|---|---|
| binds | null, int, int64, double, text, blob (values copied — TRANSIENT-safe), parameter_count/name/index, SQLITE_RANGE on bad index |
| columns | count, name (expression spelling), type (5/1/2/3/4), int, int64, double, text, blob, bytes — real coercions: text integer-prefix ('42abc'→42), real truncation (2.5→2), before-step/after-done → NULL/0, out-of-range → NULL/0 |
| statement | stmt_readonly / stmt_busy real properties; prepared DML effects visible to later statements |
| store | `Val::Real` end-to-end (literals, file serial 7) — bound doubles insert + persist |

## 4. prepare-statement-api-001..006 — each card

| Card | old → new | Why |
|---|---|---|
| 001 prepare family | partial → **partial** (tighter) | v2 fully real; v1/v3 prepFlags + UTF-16 honestly absent |
| 002 step machine | partial → **full** | real execution, ROW/DONE/19, autoreset — no VDBE claim |
| 003 typed binding | partial → **full** | full typed matrix + names + RANGE, runtime-proven |
| 004 column access | partial → **full** | full accessor matrix with real coercions |
| 005 reset/finalize | partial → **partial** (tighter) | reset/finalize/autoreset real; auto-reprepare on schema change absent |
| 006 introspection | partial → **partial** (tighter) | readonly/busy real; EXPLAIN absent |

Plus 3 new full behaviours (engine-prepare-001/-002/-003).

## 5. Cases / anti-cheat / cargo

26 bespoke goldens frozen on pinned C (two-run determinism, delegated stamp), all
replay byte-identical through the Rust statement path. Anti-cheat **15/15** (new:
runtime binds drive a computed sum; prepared DML with a runtime key visible to a
later prepared SELECT; mixed runtime column types). `cargo test` **297/297**; all 256
prior goldens md5-identical; kitchens, file suites, interop unchanged green.

## 6. Scoreboard before → after

| State | run 22 | **run 23** |
|---|---|---|
| full | 47 | **53** |
| partial | 57+3 | **57** (3 prepare cards tightened) |
| none | 103 | 103 |
| behaviours | 210 | 213 |

## 7. Explicit honesty line

**SQLite is NOT migrated.** 53 of 213 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified. The statement path is a shared-engine
executor, not a bytecode VDBE.

## 8. Next call

(a) UTF-16 + prepFlags + auto-reprepare (finishes 001/005 honestly), (b) index-driven
lookups + multi-column explicit indexes (ddl-schema-002), or (c) sqlite3_value / 
user-defined function API surface. Pack v14 + goldens first.
