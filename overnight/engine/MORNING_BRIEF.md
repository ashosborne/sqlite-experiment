# MORNING BRIEF — sqlite-experiment run 14: engine v5, eleven scripts re-homed

Run: 2026-08-11 · from `e7a47a5b8` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-engine-v5-rehome`
(Run-13 brief preserved as `MORNING_BRIEF-2026-08-11-run13.md`.)

## 1. Pack

`sqlite-experiment-c-to-rust@5` — **BOUND** (SUPERSEDE v4; versions/1–5 retained; schema-valid).
kitchen_path_cases now **18** (7 prior + the eleven below); the kitchen law (script-string lookup =
SCOPE_VIOLATION) covers all of them.

## 2. The eleven — all re-homed, zero deferred

| ID | Store learned |
| --- | --- |
| name-resolution-001-C001 | qualified `n1.a`, bare `a`, `rowid` (per-row rowids) |
| ddl-schema-002-C001 | CREATE UNIQUE INDEX, INSERT OR IGNORE, count(*) |
| dml-codegen-002-C001 | column UNIQUE, OR REPLACE (fresh rowid), OR IGNORE |
| ddl-schema-003-C001 | RENAME TO, ADD COLUMN DEFAULT, column-list INSERT w/ defaults |
| upsert-001-C001 | PK conflict + ON CONFLICT DO NOTHING; aggregate bare-col row |
| upsert-002-C001 | DO UPDATE SET b=excluded.b |
| foreign-keys-001-C001 | fk_on + REFERENCES insert check → **rc 19**, errmsg set |
| foreign-keys-002-C001 | ON DELETE CASCADE across two tables |
| foreign-keys-003-C001 | DROP-parent-with-children check → **rc 19** |
| triggers-001-C001 | trigger catalog; sqlite_master WHERE type='trigger' |
| triggers-002-C001 | AFTER INSERT fires; `new.a*2` into the log table |

Two implementation bugs found and fixed by the goldens themselves before anything shipped: a
trigger-body trim-order bug, and the widened SELECT parser swallowing `pragma_compile_options`
(fixed with a referenced-table gate: the store only claims scripts whose tables it made).

## 3. Cheat-sheet proof

`grep -cE 'CREATE TABLE (n1|t2\(|u\(|t3\(|up\(|up2|par|p2\(|p3\(|tr\(|tr2)' modern/src/script_table.rs` → **0
0**.
script_table is down to 70 entries — pure-expression and API pins only, zero user-table SQL.

## 4. Tests

`cargo test`: **111/111 green** (8 spine + 69 generated script compares + 20 kitchen/re-homed +
11 + 3 bespoke). Anti-cheats: `anti_cheat_runtime_value_round_trip` and the new
`anti_cheat_runtime_fk_round_trip` (runtime key through parent+child, then a missing key → rc 19).

## 5. Leftover still_recognizer (per-ID reasons, in the pack)

name-resolution-002 + window-functions (UNION subqueries, no user table); vacuum-001 (page
semantics); analyze-stats-001 (ANALYZE); attach-detach (catalogs); tokenizer/parser corpora;
misc-csv (vtab), misc-nextchar (index probe); introspection-vtabs (absence pin); pragma projections;
the pure-expression scripts (their honest home).

## 6. Invariants

All **122 prior goldens byte-identical** (md5 diff clean). C003 BLOCKED. Same branch, no PR, no
wasm, no C link, no files. `legacy_green` = 96; `parity_green` = 0.

## 7. completeness: incomplete

UNIQUE/FK/triggers are in-memory rules over vectors, not SQLite's btree. Not a planner. Not durable.
**SQLite is not migrated.**

## 8. Next operator call

More kitchen (JOIN, richer expressions/WHERE, multi-column keys) **or** files-on-disk (a real pager
increment — the big law change). Pack v6 + goldens first either way.
