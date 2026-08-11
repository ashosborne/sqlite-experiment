# MORNING BRIEF — engine v12: disk debt paid (run 22)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–21 stamped alongside.
Charter: FULL_AUTONOMY overnight, COMMIT_AS sqlite-engine-v12-disk-debt.
MAX_NEW_CASES 40 (used 20). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @12 BOUND — overflow + UNIQUE durability laws

versions/12.yaml + ADR 0010, schema-validated. **Overflow law** (payloads exceeding one
leaf use real overflow chains C accepts) + **index/UNIQUE durability law** (UNIQUE and
secondary indexes persist as on-disk index b-trees; reopen enforces as C does; stripping
UNIQUE from persisted SQL is banned — the v7 shortcut is now illegal). WAL still
forbidden. completeness: **incomplete**.

## 2. What was implemented (modern/src/dbfile.rs + store.rs)

- **Overflow chains:** SQLite local/spill formula (maxLocal 4061, minLocal 489, surplus
  rule), 4-byte first-overflow pointer in cells, 4-byte next pointers per chain page;
  reader reassembles chains incl. C-written files. `Val::Blob` (serial 12+2n) end-to-end,
  X'…' literals.
- **Index b-trees:** leaf 0x0a pages; records = key columns + rowid, binary-collation
  sort with rowid tie-break; column autoindexes (`sqlite_autoindex_<t>_<n>`, NULL sql in
  sqlite_schema — exactly C's shape), multi-column `UNIQUE(a,b)` table constraints,
  explicit `CREATE [UNIQUE] INDEX` (sql persisted); DROP INDEX persisted.
- **Reopen enforcement from the durable schema:** UNIQUE stays in persisted SQL; open
  re-derives column flags + UNIQUE(a,b) sets + explicit index defs from the file. A
  latent bug fell out: `conflict_row` treated NULL==NULL as a duplicate — the C golden
  (two NULLs allowed in a UNIQUE column) forced the fix.

## 3. Interop results (mandatory honesty gates — both PASS)

- `rust_write_c_read_overflow` — pinned C CLI on a Rust file: **PRAGMA integrity_check →
  ok**, `length(t)=9000`, tail substr exact, and **full-payload equality** (`t='…'` → 1).
- `rust_write_c_unique_after_reopen` — pinned C CLI **itself rejects duplicates**
  ("UNIQUE constraint failed") against Rust-written autoindex b-trees (int + text);
  Rust reopen gives rc 19; count stays 2.
- Plus `anti_cheat_overflow_runtime` (runtime marker at offset 7001 of an overflow chain,
  read back by C) and `anti_cheat_unique_runtime` (runtime key unique-enforced after
  reopen). Prior `rust_write_c_read_large`, FK interop, memory kitchen: still PASS.

## 4. Cases: 20 frozen / 0 deferred

engine-overflow-001 C001–C008 (long TEXT 6000, integrity_check, 6000-byte BLOB, mixed
page, UPDATE grow/shrink, 15000-char multi-overflow, frozen marker) ·
engine-indexes-001 C001–C012 (UNIQUE col / UNIQUE INDEX / secondary index visibility /
OR IGNORE / upsert / IPK+UNIQUE / DROP INDEX / UNIQUE(a,b) / NULL uniqueness /
post-reopen growth / TEXT UNIQUE / OR REPLACE). Two-run deterministic, delegated
HUMAN_ACCEPTED, all 20 replay byte-identical through the Rust engine.

## 5. Scoreboard before → after

| State | run 21 | **run 22** |
|---|---|---|
| full | 45 | **47** (+engine-overflow-001, +engine-indexes-001) |
| partial | 60 | 60 (2 notes materially tightened) |
| none | 103 | 103 |
| behaviours | 208 | 210 |

**ddl-schema-002 stays partial — deliberately.** Its note now reads: durable index
lifecycle real (on-disk b-trees, reopen-enforced, C-side duplicate rejection proven);
still absent: expression/partial/multi-column *explicit* indexes, index-driven lookups,
multi-leaf index b-trees. Flipping it full would have been greenwash.
upsert-001 note tightened likewise (durable conflict targets real; expression targets absent).

## 6. cargo + anti-cheat

`cargo test` **268/268** (disk_debt 20, disk_debt_interop 5, all 16 prior suites
unchanged green). SCRIPT_TABLE still 0. All 236 prior goldens md5-identical.

## 7. Honest leftovers

expression/partial indexes · multi-column explicit CREATE INDEX · index-driven lookups
(scans remain) · multi-leaf index b-trees · freelist (files fully rewritten on save) ·
WAL/crash recovery · everything in the none column (fts5, wasm/jni, vfs/pager/btree
cards stay amber — this run implements format structures, not the btree module card).

**SQLite is NOT migrated.** 47 of 210 behaviours done in modern; all parity UNVERIFIED.

## 8. Next call

(a) index-driven lookups + multi-column explicit indexes (finishes ddl-schema-002
honestly), (b) prepare/bind/column API widening (6 partial cards), or (c) freelist +
incremental save (durability polish). Pack v13 + goldens first.
