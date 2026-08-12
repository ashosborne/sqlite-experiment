# ADR 0020 — engine v22: first WAL slice

Status: accepted (pack v22 BOUND, Ash Osborne, delegated autonomy run 32)

## Context

Every prior pack version set ALLOW_WAL: false; file durability was rewrite-on-save
with journal_mode=delete (files) / memory (:memory:), no -wal/-shm ever written.
wal-001 / wal-002 sat at impl_in_modern: none. This run lifts the prohibition with a
deliberately bounded first slice. WAL is a large surface — this ADR is explicit about
what is IN and what is OUT. Prefer under-claiming.

## What is IN (frozen scope)

1. **Real WAL file format.** `dbfile.rs` writes/reads the SQLite WAL: 32-byte header
   (magic 0x377f0682 → little-endian-word checksums on the pinned x86-64 baseline,
   version 3007000, page size 4096, fixed salts), 24-byte frame headers, cumulative
   checksums over frame-header[0..8]+page. Proven by the pinned C CLI recovering a
   Rust-written db+wal mid-session (`rust_write_c_read_wal`, integrity_check ok) —
   the copy holds an empty main db with ALL data only in the -wal, so C acceptance
   proves frames/salts/checksums, not just the main-db writer.
2. **Commit granularity.** Each committed exec/step on a WAL-mode file connection
   rewrites the -wal with the full committed image as one WAL transaction (commit
   frame carries the db size). This is honest WAL format, not C's frame-level
   incrementality — documented as a residual below.
3. **journal_mode pragma.** delete/wal for files (get+set, prepared or exec'd),
   memory for :memory: (WAL refused, pinned). Mode persists via header versions=2;
   sidecars appear on first write, not at the pragma (pinned). wal→delete backfills,
   drops sidecars, persists mode 1 (pinned).
4. **Clean close = checkpoint.** Like C: main db gets the committed image
   (versions=2), -wal/-shm deleted (pinned).
5. **Checkpoint pragmas.** PASSIVE/FULL/RESTART/TRUNCATE + bare form all pinned in
   the single-connection regime: busy=0, log==checkpointed, TRUNCATE zeroes -wal.
   Backfill is observable: a wal-blind (immutable=1) C read of the main db lacks the
   row before PASSIVE and contains it after (`anti_cheat_wal_checkpoint_passive`).
6. **Same-process second connection** sees committed rows (pinned) via sidecar
   recovery at open.

## What is OUT (honest residuals)

- Multi-process reader/writer coordination, shm/wal-index locking protocol
  (the -shm written is an existence placeholder for the pinned observations;
  C rebuilds its own wal-index when it opens our files).
- Frame-level incremental commits, mxFrame reader snapshots across connections,
  busy/blocking checkpoint semantics, wal_autocheckpoint, recovery-from-torn-write
  matrices, checksum-corruption rejection pins, wal2, journal modes truncate/persist/off.

## Consequences

16 goldens (engine-wal-001 ×10, engine-wal-002 ×6) replay byte-identical; 3 mandatory
interop/anti-cheat tests green. wal-001 flips to **partial** (write path + reopen +
C interop + single-process visibility real; multi-process snapshots and recovery
matrix absent). wal-002 flips to **partial** (all four checkpoint modes pinned but
only in the single-connection regime; busy/blocking semantics unexercised — the
"all four modes" letter of full is met only nominally, so partial is the honest call).
