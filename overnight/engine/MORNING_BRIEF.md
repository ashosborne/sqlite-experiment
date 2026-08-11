# MORNING BRIEF — sqlite-experiment run 15: engine v6, durable files the C library can read

Run: 2026-08-11 · from `4ee6e851a` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-engine-v6-files`
(Run-14 brief preserved as `MORNING_BRIEF-2026-08-11-run14.md`.)

## 1. Pack @6 BOUND — the durability law (quoted)

`sqlite-experiment-c-to-rust@6` — **BOUND** (SUPERSEDE v5; versions/1–6 retained; schema-valid; 122 in-scope). `forbidden[0]`:

> "DURABILITY LAW (v6): for a file-path open, persisting kitchen data only in process memory while claiming the file worked, OR using a Rust-only dump format (JSON/bincode/msgpack) as the on-disk durability format, is a SCOPE_VIOLATION. The file must be a SQLite database file the pinned C library can open and SELECT from. PRAGMA journal_mode=WAL is out of scope this version."

## 2. File goldens frozen + stamped

New slice `engine-files` (discovery card cites main.c/pager.c/os_unix.c/btree.c). Five durable
round-trips (write → close → reopen → SELECT) frozen on the pinned C library
(`2026-08-11T1900Z-legacy-record-files`, two-run determinism 5/5) and delegated HUMAN_ACCEPTED:
C001 (INTEGER 7), C002 (INTEGER+TEXT, ORDER BY), C003 (INSERT then UPDATE/DELETE across two
reopens → [3,12]), C004 (777001 survives), C005 (424243 survives).

## 3. How persistence works (one paragraph)

`sqlite3_open(path)` on a real path loads any existing SQLite file into the in-memory v5 store, all
DDL/DML/SELECT run there, and `sqlite3_close` serialises the store back to the file. The format is
the genuine SQLite on-disk layout (`modern/src/dbfile.rs`): a 100-byte database header, page 1
holding the `sqlite_schema` b-tree leaf, and one b-tree leaf page per user table with rows encoded
as records (SQLite serial types). Page size 4096, rollback-journal header, no WAL. Limits: single
leaf page per table (tiny data — no interior/overflow pages), INTEGER/TEXT/NULL, and UNIQUE/FK/
triggers are not persisted to disk (memory kitchen keeps them).

## 4. Interop results

- **`rust_write_c_read` — PASS (mandatory):** Rust writes a file with a runtime value; the pinned
  `sqlite3` CLI opens it and `SELECT`s the value back. This is the honesty gate — C reads Rust.
- **`c_write_rust_read` — PASS (best-effort, not deferred):** the pinned CLI creates a DB; Rust's
  `dbfile::read_db` parses it and SELECTs the value.
- **`anti_cheat_reopen_runtime` — PASS:** runtime integer, Rust close/reopen, SELECT back
  (`// anti-cheat: not in script_table`).

## 5. cargo test

**119/119 green** — 8 spine + 69 generated script compares + 20 kitchen/re-homed (v4/v5) + 5
engine-files replays + 3 interop + 3 leftovers + 11 bespoke. The v4/v5 **memory kitchen is
untouched and still green**; `:memory:` opens never touch the file path.

## 6. Still incomplete

WAL, crash recovery, the VFS matrix, interior/overflow b-tree pages, full btree feature parity, and
persisted UNIQUE/FK/triggers on disk. `pager-*`, `vfs-*`, `btree-*`, `vdbe-engine-*` stay
documented-but-not-green (APP_MANIFEST notes "touched by engine-files, not frozen" — no green flag).

## 7. completeness: incomplete

122 cases; `legacy_green` = 97 of 187 behaviours; a toy single-page file writer. **SQLite is not migrated.**

## 8. Invariants

All 122 prior goldens byte-identical (md5). C003 (UAF) still BLOCKED. Same branch, no PR, no wasm,
no `sqlite3.c` link in `modern/`, no private durability format. `parity_green` = 0.

## 9. Next operator call

WAL/crash-safety (big pager law change), OR widen memory kitchen (JOIN/WHERE/expressions), OR more
schema on disk (multi-page tables, persisted indexes/FK). Pack v7 + goldens first.
