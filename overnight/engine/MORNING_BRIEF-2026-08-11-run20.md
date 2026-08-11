# MORNING BRIEF — engine v10: completion sweep (run 20)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–19 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v10-completion-sweep, GOAL = maximize honest
impl_in_modern=full. MAX_NEW_CASES 40 (used 37). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @10 BOUND

versions/10.yaml + ADR 0008, schema-validated. New **completion-sweep law**: the increment's
success metric is honest `impl_in_modern: full` growth; goldens exist to unlock upgrades,
never to inflate legacy_green. All prior laws kept; WAL still forbidden; no planner claim.
v8 defer list shrank 18 → **13** (4 date/time + misc-func-packs reclaimed, goldens untouched).

## 2. Operator progress histogram — before → after

| State | run 19 | **run 20** |
|---|---|---|
| full (converted, parity UNVERIFIED) | 11 | **26** |
| partial | 75 | **72** |
| none | 108 | **103** |
| behaviours known | 194 | 201 (+7 sweep slices) |

## 3. Every behaviour that moved

**none → full (4):** date-time-funcs-001 (julian-day engine), -002 (strftime), -003
(modifier grammar; `localtime` deliberately errors — TZ), -004 (timediff). All four v8
defers cleared by real execution of the frozen scripts.

**partial → full (4):** select-codegen-002 (INTERSECT+EXCEPT close the set-op family) ·
name-resolution-001 (prepare-time "ambiguous column name" / "no such column" via eager
schema-row probe) · ddl-schema-001 (views: create/drop/expand/write-reject; tables already
real memory+durable) · misc-rot13-001 (rot13 collating sequence joins the function).

**new full (7):** engine-datetime/setops/constraints/views/triggers/funcs/pragma-001 —
composed sweep slices whose accepted scope IS the 37 frozen cases, all executing for real.

**none → partial (1):** misc-func-packs-001 (decimal_mul + REGEXP execute; ~16 pack
functions still absent).

**partial, gap tightened (8):** dml-codegen-002 (+CHECK/NOT NULL/OR FAIL; ROLLBACK absent) ·
triggers-001/-002 (full B/A × I/U/D matrix + WHEN + old/new; INSTEAD OF, RAISE, recursion
absent) · foreign-keys-002 (+SET NULL/RESTRICT; SET DEFAULT/ON UPDATE absent) ·
printf-format-001 (flags/width/precision + 15 conversions; %w/positional absent) ·
pragma-surface-001 (27 of ~70) · builtin-scalar-agg-funcs-001 (~24 of ~60) ·
misc-decimal-001 (+sub, mul trims trailing zeros; decimal(X)/pow2/collation absent).

**Deliberately NOT flipped:** select-codegen-001/-003, where-optimizer-*, pager/btree/vfs/
wal/fts/wasm/jni — untouched per charter.

## 4. New cases: 37 frozen / 0 deferred

engine-datetime 8 · engine-setops 4 · engine-constraints 8 (5 error-scripts) ·
engine-views 3 · engine-triggers 4 · engine-funcs 7 · engine-pragma 3. Two-run
deterministic on the pinned C build, delegated HUMAN_ACCEPTED, and **all 37 replay
byte-identical through the executor**. C taught us: `%c` converts its arg to text first
(printf('%c',65) → '6'), decimal_mul trims trailing zeros ('1.25'×'4' → '5'),
`PRAGMA busy_timeout=N` returns a row, rot13 collation compares rot13-images.

## 5. Anti-cheat + suite

anti-cheat 10/10 (new: runtime day-of-month through strftime/date/unixepoch; runtime CHECK
bound + INTERSECT; SCRIPT_TABLE still 0 — grep: no pins). `cargo test` **208/208**:
oneshot 56 (51 + 5 reclaims), sweep 37, query 24, kitchen/files/interop unchanged green.
All 179 prior goldens md5-identical; only new RECORD files added.

## 6. Explicit honesty line

**SQLite is NOT migrated.** 26 of 201 behaviours are done in modern; every one is
`parity: UNVERIFIED` (COMPARE has never run; parity_green 0; nothing `verified`).

## 7. Next call (from the new Remaining/Partial tops)

Remaining leaders: fts5, wasm/jni, vfs/pager/btree/wal estate (out of scope until chosen),
compile-options matrix, blob-io-api, analyze-stats. Partial closers within reach:
(a) HAVING + DISTINCT aggregates (builtin-002, select partials), (b) on-disk debt
(overflow pages + UNIQUE autoindexes → ddl-schema-002), (c) prepare/bind/column API
widening (typed binds + column matrix). Pack v11 + goldens first, either way.
