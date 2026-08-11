# ADR 0004 — Engine v6: durable files the C library can read

Status: accepted (BOUND with pack v6, Ash Osborne, 2026-08-11 Europe/London, delegated).

## What changes

v4/v5 built an in-memory kitchen; closing the connection lost everything. v6 adds durability:
`sqlite3_open(path)` on a filesystem path writes rows to a **real SQLite database file** — the
on-disk format from fileformat2.html (100-byte header, table b-tree leaf pages, records with SQLite
serial types), NOT a private dump. The honesty gate is a test, `rust_write_c_read`: Rust writes the
file, the **pinned C `sqlite3` CLI** opens it and `SELECT`s the value back. It passes. A private
JSON/bincode durability format would be a SCOPE_VIOLATION.

## How

The in-memory v5 store engine is reused unchanged. `modern/src/dbfile.rs` is the format boundary:
on a file-path open it parses an existing SQLite file into the store; on close it serialises the
store's tables back to the file (page 1 = header + `sqlite_schema` leaf; one leaf page per table).
`:memory:` connections are untouched. Interop both ways is tested (`rust_write_c_read` mandatory;
`c_write_rust_read` best-effort, also passing on the tiny single-page case).

## Honest limits

Page size 4096; **single leaf page per table** (tiny data — no interior/overflow pages); INTEGER/
TEXT/NULL; no WAL, no crash recovery, no VFS matrix, no freelist; UNIQUE/FK/triggers are NOT
persisted to disk (the memory kitchen keeps them; file goldens use only the plain subset). This is
the first durable increment, not a pager/btree. **SQLite is not migrated.** `pager-*`, `vfs-*`,
`btree-*`, `vdbe-engine-*` remain documented-but-not-green (APP_MANIFEST carries a "touched by
engine-files, not frozen" note, no green flag).
