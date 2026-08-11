# MORNING BRIEF — sqlite-experiment run 16: engine v7, multi-page + durable schema on disk

Run: 2026-08-11 · from `75af6d9ed` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-engine-v7-schema-disk`
(Run-15 brief preserved as `MORNING_BRIEF-2026-08-11-run15.md`.)

## 1. Pack @7 BOUND — the multi-page + persist-rules law (quoted)

`sqlite-experiment-c-to-rust@7` — **BOUND** (SUPERSEDE v6; versions/1–7 retained; schema-valid; 150 in-scope). `forbidden[0]`:

> "MULTI-PAGE + PERSIST-RULES LAW (v7): a file-path kitchen exceeding one leaf MUST use real SQLite multi-page b-trees (interior 0x05 + leaves) the pinned C library reads (integrity_check must pass); FK declarations (REFERENCES) and triggers the memory kitchen supports MUST be persisted in sqlite_schema so reopen + C observe them. Answering any durable_path_cases SQL by script_table string-match remains a SCOPE_VIOLATION. WAL still forbidden."

## 2. New cases: 28 frozen, 0 deferred-in-batch (2 themes deferred at design)

| Batch | Feature | Cases | Themes |
| --- | --- | --- | --- |
| A (size/pages) | engine-files-002 | 6 | 1200-row multi-leaf count + WHERE middle row; 40×200-char TEXT multi-leaf count + specific text; 40-row mixed; small mixed ORDER BY with fresh literal 888002 |
| B (schema on disk) | engine-files-003 | 12 | FK orphan-after-reopen→19, FK valid, CASCADE, trigger persist+fire, sqlite_master lists trigger, ALTER RENAME, ALTER ADD COLUMN DEFAULT, PK upsert DO NOTHING/DO UPDATE, IPK survives, multi-column, two-FK-tables |
| C (catalogue twins) | engine-files-004 | 10 | engine-kitchen C001–C005 durable twins, name-resolution, UNIQUE-INDEX + column-UNIQUE dedup twins, FK CASCADE, trigger*2 |

All 28 recorded on the pin C (`2026-08-11T2000Z-legacy-record-files-batch`, two-run determinism 164/164 OBS
lines), delegated HUMAN_ACCEPTED. **Design-time deferrals (honest, in pack `known_risks`):** overflow pages
(TEXT > one leaf) and post-reopen UNIQUE-index enforcement (non-IPK autoindex b-trees) — column-UNIQUE is
stripped from persisted sql (rows already de-duplicated in-session), so a duplicate insert *after* reopen is
not enforced on disk this version. Not cheated.

## 3. Page kinds implemented

Table b-tree **leaf** pages (0x0d) and **interior** pages (0x05) with a right-most pointer — real
multi-leaf b-trees for large tables. Records use SQLite serial types + varints. INTEGER PRIMARY KEY
stored as rowid (column NULL). `sqlite_schema` (page 1 leaf) carries table rows (with REFERENCES/FK sql)
and trigger rows (type='trigger', rootpage 0, sql). **Overflow pages: not implemented** (deferred).

## 4. Interop results (both mandatory C-read tests PASS)

- **`rust_write_c_read_large` — PASS:** Rust writes a 1500-row multi-page table with a runtime value; the
  pinned C CLI reports `count(*)=1501`, reads the runtime value via `WHERE`, and **`PRAGMA integrity_check`
  returns `ok`** (proves the interior b-tree is well-formed to C).
- **`rust_write_c_read_unique_or_fk` — PASS (via FK):** Rust writes an IPK parent + FK child; C reads the
  valid child and **rejects an orphan insert** (`FOREIGN KEY constraint failed`).
- Also green: `c_write_rust_read`, `rust_write_c_read`, `anti_cheat_reopen_runtime`, `anti_cheat_reopen_many_rows`.

## 5. cargo test

**150/150 green** — 8 spine + 69 generated script compares + 20 kitchen/re-homed + 5 engine-files (v6) +
28 file-batch (v7) + 6 interop + 3 leftovers + 11 bespoke. The `:memory:` memory kitchen is unchanged and
still green. (De-flaked `loadext_002`: it now tracks auto-inited db pointers instead of a process-global
counter, robust under parallel test threads.)

## 6. Catalogue impact — durable file evidence

Behaviours with durable **file-path** evidence now: engine-files-001..004 (the 33 durable cases), covering
size/pages, FK, triggers, ALTER, PK upserts, IPK, multi-column, and durable twins of engine-kitchen /
name-resolution / ddl / dml / fk. Memory-only-but-not-durable still: everything else in the 190-behaviour
catalogue (the recognizer/expression scripts, and advanced constraints whose on-disk enforcement is deferred).
`legacy_green` = **100 of 190**.

## 7. Leftover (as expected)

WAL, crash recovery, VFS matrix, overflow pages, on-disk UNIQUE-index enforcement after reopen, joins,
wasm/jni/bindings, and the whole engine core (`vdbe-engine`, `btree`, `pager`, `pcache`, `where-optimizer`)
which remain documented-but-not-green (honest notes, no green flags).

## 8. completeness: incomplete

150 in-scope cases; a real-but-toy pager/b-tree that stores multi-page tables and durable FK/triggers.
No WAL, no overflow, no planner. **SQLite is not migrated.**

## 9. Invariants

All 127 prior goldens byte-identical (md5). C003 (UAF) BLOCKED. Same branch, no PR, no wasm, no
`sqlite3.c` link (45 exported symbols), no private durability format, no WAL. `parity_green` = 0.

## 10. Next operator call

WAL + crash-safety (big pager law change), OR overflow pages + on-disk UNIQUE autoindexes (finish durable
schema), OR joins/expressions in the kitchen. Pack v8 + goldens first, either way.
