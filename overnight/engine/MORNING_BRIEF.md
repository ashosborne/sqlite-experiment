# MORNING BRIEF — sqlite-experiment run 13: engine v1, the kitchen gets a real store

Run: 2026-08-11 · from `73a4f2e8a` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-engine-v1-kitchen`

## 1. Pack

`sqlite-experiment-c-to-rust@4` — **BOUND** (SUPERSEDE v3; bound_by Ash Osborne; versions/1–4 all retained; schema-valid; **117 in-scope cases**). The new law, quoted from `forbidden[0]`:

> "KITCHEN LAW (v4): answering kitchen-path SQL (CREATE TABLE / INSERT INTO / UPDATE / DELETE / SELECT-from-user-table shapes in kitchen_path_cases) by matching the full script string or its hash in script_table or ANY canned-answer map is a SCOPE_VIOLATION. Rows must live in an in-memory catalog (tables -> columns -> rows); values come from the statements, not a preloaded answer key."

`ddl-schema-001-C001` and `dml-codegen-001-C001` were **re-homed onto the store** (goldens unchanged, byte-matched) — no `still_recognizer` escape needed for them. The other CREATE-containing frozen scripts stay recognizer with a named `still_recognizer` known_risk (triggers/constraints/vtabs the store hasn't built). ADR: `ADR/0002-engine-v1-kitchen.md`.

## 2. Five kitchen goldens

New slice `engine-kitchen` (discovery card cites src/build.c:1225, insert.c:900, update.c:285, delete.c:288, select.c:7642). Frozen on the pinned C library (`2026-08-11T1800Z-legacy-record-kitchen`, two-run determinism 5/5, immediate replay byte-match) and delegated-stamped HUMAN_ACCEPTED: C001 (insert 7 → 7), C002 (two typed rows, ORDER BY), C003 (UPDATE 10→11), C004 (DELETE WHERE), C005 (fresh integer **424242**, present in no prior golden or table — proof the C side wasn't replaying either).

## 3. How the store works (one paragraph)

Each open connection owns a small in-memory catalog: a list of tables, each with column names and
rows of integer/text values. `CREATE TABLE` adds a table, `INSERT` appends rows parsed from the
statement, `UPDATE`/`DELETE` walk the rows and mutate/remove the ones matching `WHERE col=int`,
`SELECT` reads whatever is in the table at that moment (optionally sorted by an integer column),
and `sqlite_master`/`changes()`/`total_changes()` are answered from the catalog and its counters.
`sqlite3_exec` tries the store first; only scripts the store cannot parse fall back to the
recognizer — whose table no longer contains any kitchen SQL. It is a toy: two types, one table per
statement, no planner, no durability. **Not SQLite.**

## 4. Tests

`cargo test`: **110/110 green** — 8 original spine, 80 generated script compares (the two re-homed
cases removed from generation and re-asserted via the store in kitchen_compare.rs), 11 + 3 bespoke,
8 kitchen (five goldens + two re-homed + **`anti_cheat_runtime_value_round_trip`**, which inserts
`600000 + pid%1000`, updates it, and selects it back — `// anti-cheat: value not in script_table`).

## 5. Invariants

All 117 prior goldens **byte-identical** (md5). C003 still BLOCKED. Same branch, no PR, no wasm,
no `sqlite3.c` link, no files. `legacy_green` = 96 (engine-kitchen-001 flipped after stamp);
`parity_green` = 0. vdbe-engine/btree/pager stay documented-but-not-done — no fake Phase-B "done".

## 6. completeness: incomplete

117 cases, 96 of 186 behaviours legacy-green, one toy store. SQLite is not migrated.

## 7. Next operator call

Widen the kitchen (more types, richer WHERE, multiple tables/joins, constraint errors) **or** start
files-on-disk (a real pager increment — much bigger law change). Either way: pack v5 + new goldens first.
