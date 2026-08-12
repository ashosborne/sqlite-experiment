# MORNING BRIEF — engine v24: VACUUM and VACUUM INTO (run 34)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–33 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v24-vacuum, DEEPEN_WAL: false.
MAX_NEW_CASES 55 (used 18; stretch batches skipped — VACUUM alone was the run).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @24 BOUND — VACUUM law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v23 → **v24**
(versions/1–24 retained; ADR `0022-engine-v24-vacuum.md`; schema VALID; 30 laws).
Plain language: VACUUM must really rebuild the content — copy everything into fresh
tightly-packed storage and drop the wasted space — and VACUUM INTO must copy the
rebuilt database into a brand-new file that C can open. Canned answers are a
SCOPE_VIOLATION. WAL stays at its v22 claim level. SCRIPT_TABLE stays 0.

## 2. What VACUUM now does in modern

| Behaviour | Evidence |
| --- | --- |
| Plain rowids renumber after deletes (1,3,5 → 1,2,3) | pinned before/after; skipping the rebuild would fail this |
| INTEGER PRIMARY KEY / WITHOUT ROWID keys preserved | pinned (ids 10,30 stay 10,30) |
| Free space reclaimed | `PRAGMA page_count` now models the freelist: grows with data, does **not** shrink on DELETE, drops only at VACUUM (pinned shrink booleans) |
| Cannot VACUUM inside a transaction | C's exact error; the transaction keeps working (pinned) |
| Files rewritten immediately, C-readable | `rust_vacuum_c_read`: pinned C CLI reads a Rust file after delete+VACUUM, integrity ok |
| WAL mode | VACUUM rewrites main **and** the -wal (stale frames cannot resurrect old rowids); journal_mode stays wal (pinned) |
| VACUUM INTO | fresh target with tables+indexes, C-readable, source untouched; "output file already exists" / rc-14 invalid path / txn error pinned; `:memory:` source exports to a file |

The old `vacuum-001-C001` golden (frozen in run 11, honestly deferred ever since —
`legacy_green ≠ done`) **now replays for real**: `zeroblob(1000)` insert, DROP, VACUUM,
SELECT — all executed, no script table.

## 3. Engine holes the pins forced open (all real fixes)

- **DELETE with any WHERE** (`n > 5`, `v % 2 = 0`, `IN (...)`) — per-row expression
  evaluation replaces the old col=int-only filter.
- **INSERT VALUES with constant expressions** (`zeroblob(1000)`) — computed, not
  literal-parsed (plus a one-paren-only VALUES fix).
- **`SELECT rowid` / ORDER BY rowid** — routed to the row store (evaluator rows carry
  no rowids), with rowid correctly aliasing the INTEGER PRIMARY KEY.
- **sqlite_master type/name projections** with multi-key ORDER BY.

## 4. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| vacuum-001 | none (legacy_green only) | **partial** | pending page_size / auto_vacuum apply; `VACUUM <schema>` attached forms — named in the card, so full would over-claim |
| vacuum-002 | none | **partial** | URI filename targets (`file:...?...`) — named in the card |
| engine-vacuum-001/002/003 | — | **new full ×3** | composed cards for exactly the frozen batches |

Stretch cards (get_table, status counters, close_v2) skipped, per charter.

## 5. Anti-cheat + C interop + cargo

- `anti_cheat_vacuum_runtime` — pid-seeded table name + payloads: rowids renumber
  1..3 after a modulo DELETE + VACUUM; a runtime-chosen `VACUUM INTO` target is read
  back by the pinned C CLI (sum + integrity ok).
- `rust_vacuum_c_read` — C reads the Rust file after in-place VACUUM.
- `legacy_vacuum_001_c001_replays` — the deferred golden replays byte-identical.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 541/541 PASS** (was 519; +18 golden twins, +4 anti-cheat/interop/legacy).
  engine-wal / upsert-expr / collation / utf16 / harvest23 suites all green; 510
  pre-run goldens md5-verified intact.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 33 | Run 34 |
| --- | --- | --- |
| **full (converted)** | 109 | **112** |
| partial | 44 | 46 |
| none (remaining) | 101 | **99** |
| behaviours known | 254 | 257 |

legacy_green 164 → 167. parity_green 0. Nothing `verified`. WAL claim unchanged.

## 7. Not migrated

SQLite is **not migrated**. 112/257 behaviours run honestly in modern for frozen scope
only. VACUUM does not apply pending page_size/auto_vacuum, and INTO takes plain paths
only. Parity UNVERIFIED everywhere (COMPARE never run).

## 8. Next call

1. **blob-io none** — sqlite3_blob_open/read/write incremental I/O: bounded, real
   engine work, another none→partial/full.
2. **VACUUM deepen** — URI INTO targets + attached-schema forms to finish vacuum-002.
3. **connection-lifecycle none crumbs** — close BUSY vs close_v2 zombie pins
   (single-process), get_table marshalling.
