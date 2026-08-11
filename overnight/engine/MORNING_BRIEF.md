# MORNING BRIEF — engine v11: thin-gap harvest (run 21)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–20 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v11-thin-gap-harvest, GOAL = honest full growth.
MAX_NEW_CASES 40 (used 33). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @11 BOUND

versions/11.yaml + ADR 0009, schema-validated. All laws carried (cheat-sheet ban, kitchen,
durability, join/subquery, completion-sweep); WAL forbidden; no planner claim.
completeness: **incomplete**.

## 2. Histogram before → after

| State | run 20 | **run 21** |
|---|---|---|
| full (converted, parity UNVERIFIED) | 26 | **45** |
| partial | 72 | **60** |
| none | 103 | 103 |
| behaviours known | 201 | 208 (+7 harvest slices) |

## 3. Behaviours flipped to full (12 + 7 new)

**Tier A:** builtin-scalar-agg-funcs-002 (DISTINCT + FILTER + HAVING close the aggregate
family). **Tier B:** misc-sha1-001 & misc-shathree-001 (sha1_query/sha3_query via the
extension's exact row-hash protocol — S{n}:sql · R · N/I/F/T/B big-endian value images) ·
misc-basexx-001 (base85 + is_base85 with SQLite's numeral set + trailing newline) ·
misc-ieee754-001 (from_blob/to_blob complete the set) · misc-uint-001 (collation now in
ORDER BY) · misc-totype-001 (blob/overflow/integral-real strictness pinned + real) ·
misc-uuid-001 (canonicalization) · name-resolution-002 (COLLATE terms + NULLS FIRST/LAST) ·
foreign-keys-002 (action matrix: +ON UPDATE CASCADE/SET NULL, +ON DELETE SET DEFAULT) ·
triggers-001 (INSTEAD OF on views, DROP TRIGGER, UPDATE OF — DDL lifecycle complete) ·
ddl-schema-003 (RENAME COLUMN + DROP COLUMN complete the ALTER family).
**New harvest slices (7, full from birth):** engine-agg-having / misc2 / order2 / fk2 /
trig2 / ddl2 / upsert2 -001.

## 4. Stayed partial — with honestly tighter notes (4)

misc-decimal-001 (decimal_exp + true arbitrary precision absent — i128-bounded) ·
printf-format-001 (thousands-separator ',' flag + %p absent) · triggers-002
(RAISE(IGNORE/FAIL/ROLLBACK), INSTEAD OF UPDATE/DELETE firing absent) · upsert-002
(multi-assignment SET breadth absent). Prefer under-claiming: none of these was flipped.

## 5. New cases: 33 frozen / 0 deferred

engine-agg-having 8 · engine-misc2 12 · engine-order2 2 · engine-fk2 3 · engine-trig2 5
(incl. RAISE error-script) · engine-ddl2 2 · engine-upsert2 1. Two-run deterministic,
delegated HUMAN_ACCEPTED, all replay byte-identical via the executor. C taught us:
decimal_pow2 renders `+1.024e+03`, base85 appends a newline, `PRAGMA busy_timeout` style
set-returns persist. Golden-caught executor bugs fixed: `_` treated as a word boundary
(`ieee754_from_blob` parsed as FROM), kitchen SELECT silently dropping ORDER BY clauses
it could not parse.

## 6. Anti-cheat + suite

anti-cheat **12/12** (new: runtime HAVING/DISTINCT group; runtime sha1_query digests
differ across statements; base85∘base85 round-trip of runtime ieee754 bits;
SCRIPT_TABLE still 0). `cargo test` **243/243**; all 216 prior goldens md5-identical;
memory + file kitchens green.

## 7. Explicit honesty line

**SQLite is NOT migrated.** 45 of 208 behaviours are done in modern; all 45 are
`parity: UNVERIFIED`; parity_green 0; nothing `verified`.

## 8. Next call (from the new tops)

Remaining leaders unchanged (fts5, wasm/jni, vfs/pager/btree/wal estate, blob-io,
compile-options). Partial closers within reach: (a) window-functions family breadth
(row_number/rank/lag/lead + frames — 2 cards), (b) prepare/bind/column API widening
(typed binds + column matrix, 6 partial cards), (c) on-disk debt (overflow pages +
UNIQUE autoindexes → ddl-schema-002 + engine-files notes). Pack v12 + goldens first.
