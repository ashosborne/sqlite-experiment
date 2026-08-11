# ADR 0010 — engine v12: disk debt (pack v12)

Date: 2026-08-12 · Status: BOUND (supersedes pack v11; v1–v11 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY overnight charter, run 22)

## Context
Since v7 the file engine wrote multi-page table b-trees C could integrity_check, but two
debts remained (ADR 0005 known risks): payloads had to fit one leaf (no overflow pages),
and UNIQUE was stripped from persisted SQL with de-duplication happening only in-session
— reopen did not enforce uniqueness the way C does.

## Decision
1. **Overflow law.** Payloads exceeding one leaf cell use real SQLite overflow chains:
   the local/spill split follows the file-format formula (maxLocal = usable−35 = 4061,
   minLocal = (usable−12)·32/255−23 = 489, surplus rule), cells carry a 4-byte first
   overflow page, chain pages carry 4-byte next pointers + data. Reader reassembles
   chains (also for C-written files). BLOB values (serial 12+2n) added end-to-end
   (`Val::Blob`, X'…' literals).
2. **Index/UNIQUE durability law.** UNIQUE stays in persisted SQL and is backed by real
   index b-trees (leaf 0x0a; records = key columns + rowid, binary-collation sorted,
   rowid tie-break): column autoindexes (`sqlite_autoindex_<t>_<n>`, NULL sql),
   multi-column UNIQUE(a,b) table constraints, and explicit CREATE [UNIQUE] INDEX
   (with their CREATE sql). Reopen re-derives enforcement from the durable schema —
   never from RAM-only sets. DROP INDEX persists. NULLs stay distinct in unique
   columns (fixed a latent conflict_row bug the C009 golden exposed).
3. **20 cases frozen on pinned C** (engine-overflow 8, engine-indexes 12; two-run
   determinism, delegated stamp) covering long TEXT/BLOB, mixed pages, UPDATE
   grow/shrink, multi-overflow, marker payloads, UNIQUE column/index/multi-column,
   OR IGNORE/REPLACE + upsert after reopen, DROP INDEX, NULL uniqueness.
4. **Honesty gate (both mandatory interop tests PASS):** `rust_write_c_read_overflow`
   (C integrity_check=ok + exact 9000-char payload equality) and
   `rust_write_c_unique_after_reopen` (C itself reports "UNIQUE constraint failed"
   against Rust-written autoindex b-trees; Rust reopen gives rc 19). Plus runtime
   anti-cheat markers/keys through overflow + unique paths.

## Consequences
- cargo 268/268; prior goldens byte-identical; memory kitchen + all prior file suites
  green; SCRIPT_TABLE still 0.
- Honest remaining limits: single-leaf index b-trees (small pinned indexes),
  no expression/partial/multi-column *explicit* indexes, indexes not consulted for
  lookups (scan + constraint checks), no freelist (full rewrite on save), no WAL.
  ddl-schema-002 therefore stays partial. SQLite is NOT migrated.
