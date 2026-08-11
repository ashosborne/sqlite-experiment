# ADR 0005 — Engine v7: multi-page tables and durable schema the C library reads

Status: accepted (BOUND with pack v7, Ash Osborne, 2026-08-11 Europe/London, delegated).

v6 wrote one leaf page per table with memory-only rules. v7 makes on-disk mode carry a much larger
share of the kitchen: **multi-leaf table b-trees with an interior root (0x05)** for tables that
exceed one 4096 leaf, **INTEGER PRIMARY KEY = rowid** so the pinned C library enforces foreign keys
against `id INTEGER PRIMARY KEY` parents natively, and **durable FK declarations + triggers** written
into `sqlite_schema` so a reopen — in Rust or in C — observes them. The mandatory interop gates pass:
C runs `integrity_check=ok` on a 1500-row Rust file and reads a runtime value from a multi-page
b-tree; C enforces the FK on a Rust-written file (orphan insert → constraint failed).

Honest deferrals (not cheated): overflow pages (TEXT larger than one leaf) and post-reopen UNIQUE
enforcement via non-IPK autoindex b-trees are out of scope — column-UNIQUE is stripped from the
persisted table sql (the rows are already de-duplicated in-session and stored uniquely), so a
duplicate insert *after* reopen is not enforced on disk this version. WAL, crash recovery, and the
VFS matrix remain out. `pager-*`/`btree-*`/`vfs-*` stay documented-but-not-green. **SQLite is not migrated.**
