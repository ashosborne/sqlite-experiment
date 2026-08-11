# engine-files-001 — Durable file round-trip

Slice: `engine-files` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T16:43:02Z
Composed slice (run 15): the thin durability cross-section of pager/vfs/btree, carved so pack v6 can
require Rust to write a **real SQLite database file** the pinned C library can read.

## Summary
sqlite3_open(path) creates/opens an on-disk SQLite DB (page size 4096, rollback-journal default,
no WAL). CREATE/INSERT/UPDATE/DELETE mutate the file's tables; after close a fresh open on the same
path sees committed rows. As implemented by the pinned C library for the five frozen scripts.

## Entrypoints (citations)
- sqlite3_open(path) → openDatabase (src/main.c:3742) → pager (src/pager.c:4789) → unix VFS
  (src/os_unix.c:6519) → btree (src/btree.c:2562). The on-disk contract is the SQLite file format
  (fileformat2.html): 100-byte header + table b-tree leaf pages + records.

## Observables
- write.rc / reopen.rc / read.rc, callback rows after reopen — frozen in the engine-files goldens.

## Assumptions / unknowns
- Single-file main DB; tiny data (single leaf page per table); no WAL, no crash recovery, no interior
  pages, no persisted UNIQUE/FK/triggers this version (memory kitchen keeps those).

## Evidence
- Five frozen goldens under tests/characterization/engine-files/ recorded on the pin (run 2026-08-11T1900Z-legacy-record-files).
