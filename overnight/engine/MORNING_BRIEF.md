# MORNING BRIEF — engine v22: first WAL slice (run 32)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–31 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v22-wal, **ALLOW_WAL: true (first time)**.
MAX_NEW_CASES 40 (used 16). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @22 BOUND — the WAL ban is lifted, narrowly

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v21 → **v22**
(versions/1–22 retained; ADR `0020-engine-v22-wal.md`; schema VALID; 28 laws).
The blanket "WAL forbidden" clauses embedded in the v6–v21 law texts are superseded by
three new laws: **WAL-FORMAT** (real SQLite WAL the pinned C amalgamation recovers — a
private sidecar log is a SCOPE_VIOLATION), **JOURNAL-MODE** (delete/wal on files,
memory on :memory:, pinned transitions), **CHECKPOINT-MIN** (backfill observable
wal-blind; single-connection regime only). SCRIPT_TABLE stays 0; `completeness: incomplete`.

## 2. What WAL subset landed (modern/src/{dbfile,store,eval,lib}.rs)

| Piece | Semantics |
| --- | --- |
| WAL format | 32-byte header (magic 0x377f0682 → LE-word checksums, v3007000, psz 4096, fixed salts) + 24-byte frame headers, cumulative checksums; commit frame carries db size |
| Commit granularity | each committed exec/step rewrites the -wal with the full committed image as ONE WAL transaction — honest C-valid format, **not** C's frame-level appends (residual) |
| journal_mode | wal/delete get+set on files (exec **and** prepared paths), memory pinned for :memory:; mode persists via header versions=2; sidecars appear on first write, not at the pragma (pinned) |
| Clean close | checkpoint into main db + delete -wal/-shm, mode persists — exactly C's pinned file states |
| Open/recovery | reopen reads header versions + recovers committed frames from an existing -wal (checksum-validated scan) |
| wal→delete | backfill, drop sidecars, persist mode 1 (pinned) |
| Checkpoints | bare/PASSIVE/FULL/RESTART/TRUNCATE: busy=0, log==checkpointed row, TRUNCATE zeroes -wal; backfill via full-image main-db write |

Two engine fixes forced by the pins: prepared statements now surface pragma result rows
(`PRAGMA journal_mode=WAL` via prepare/step returns `wal` like C), and pragma column
names reach `sqlite3_column_count` on the prepared path.

## 3. C interop results (the honesty gate)

- **`rust_write_c_read_wal`** — the copied Rust db+wal holds an *empty main db* with all
  data only in the -wal; the pinned C CLI recovers it, returns the runtime row, and
  reports `integrity_check = ok`. This proves frames/salts/cumulative checksums, not
  just the main-db writer.
- **`anti_cheat_wal_checkpoint_passive`** — a wal-blind (`immutable=1`) C read of the
  main db lacks the row before PASSIVE and contains it after: backfill is real.
- **`anti_cheat_wal_runtime_reopen`** — pid-seeded row survives close/reopen in WAL
  mode; reopened connection reports `journal_mode=wal`.

## 4. Checkpoint modes: done vs residual

| Mode | Pinned | Residual |
| --- | --- | --- |
| bare / PASSIVE | ✔ counts + durability + wal-blind backfill | — |
| FULL | ✔ single-connection | busy/blocking vs readers NOT exercised |
| RESTART | ✔ single-connection | same |
| TRUNCATE | ✔ zeroes -wal | same |
| wal_autocheckpoint | ✗ | not claimed |

## 5. Frozen cases (16 new, two-run deterministic, delegated HUMAN_ACCEPTED)

| Feature | Cases | Pins |
| --- | --- | --- |
| engine-wal-001 | C001–C010 | pragma returns wal + no sidecars until first write; -wal/-shm live during session; clean-close deletion + persistence; wal→delete; :memory: refusal; multi-commit; rollback; 60-row multi-page; second same-process connection; empty-DB mode + integrity |
| engine-wal-002 | C001–C006 | PASSIVE / bare / FULL / RESTART / TRUNCATE (+wal zeroing), checkpoint-then-write-then-reopen |

RECORD run `2026-08-12T2200Z-legacy-record-wal`; catalog21.json; all 461 pre-run goldens
md5-verified intact (rollback-path behaviour untouched).

## 6. wal-001 / wal-002 verdicts (under-claimed deliberately)

| Card | Verdict |
| --- | --- |
| wal-001 | **none → partial.** Real: C-valid write path, C interop, mode persistence, reopen recovery, single-process visibility. Residual: full-image commits (not frame appends), no multi-conn mxFrame snapshots, no shm locking protocol, no corruption/torn-write recovery matrix. |
| wal-002 | **none → partial.** All four modes + bare pinned and real, but only single-connection — the modes differ precisely in cross-connection busy/blocking behaviour, which is unexercised. Flipping full would greenwash that distinction. |
| engine-wal-001/002 | new composed cards, **full** for exactly their frozen pins. |
| pragma-surface-001 | notes tightened (journal_mode + wal_checkpoint family); stays partial. |

## 7. Cargo + regressions

**cargo test: 508/508 PASS** (was 488; +16 golden twins, +4 interop/anti-cheat/guard).
All prior file/txn/upsert/index/UTF-16/UDF/collation suites green; journal_mode
delete/memory pins unchanged.

## 8. Scoreboard (impl_in_modern) — before → after

| State | Run 31 | Run 32 |
| --- | --- | --- |
| **full (converted)** | 92 | **94** |
| partial | 48 | 50 |
| none (remaining) | 103 | 101 |
| behaviours known | 243 | 245 |

legacy_green 153 → 155. parity_green 0. Nothing `verified`.

## 9. Not migrated — and not "WAL complete"

SQLite is **not migrated**, and this is **not** SQLite's WAL: no multi-process
coordination, no wal-index locking, no frame-level appends, no reader snapshots
across connections, no recovery matrix, no wal2. 94/245 behaviours run honestly in
modern for frozen scope only. Parity UNVERIFIED everywhere (COMPARE never run).

## 10. Next call

1. **Frame-level WAL commits** — replace full-image rewrites with per-page appends
   (keeps C interop, shrinks the wal-001 residual materially).
2. **errmsg16 / create_collation16 / create_function16** — finish the UTF-16 surface.
3. **RETURNING clause** — bounded, high-visibility DML card.
