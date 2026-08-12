# MORNING BRIEF — engine v25: incremental blob I/O (run 35)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–34 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v25-blob-io, DEEPEN_WAL/VACUUM: false.
MAX_NEW_CASES 55 (used 22; stretch batches skipped — blob alone was the run).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @25 BOUND — blob I/O law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v24 → **v25**
(versions/1–25 retained; ADR `0023-engine-v25-blob-io.md`; schema VALID; 31 laws).
Plain language: open a handle on one cell, read/write bytes at an offset, the handle
dies if the row changes underneath it, and writes can never resize the cell. Serving
blob bytes from a canned map is a SCOPE_VIOLATION. WAL stays at v22; VACUUM at v24.

## 2. What blob handles now do in modern

| Behaviour | Evidence |
| --- | --- |
| Open on (schema, table, column, rowid) | C's exact validation order + error text: `cannot open view: v` → `no such table: main.nope` → `no such column: "zz"` → `cannot open indexed column for writing` (RW only; RO allowed) → `no such rowid: 99` → `cannot open value of type null` (all pinned) |
| rowid aliases INTEGER PRIMARY KEY | pinned (handles opened by id on IPK tables) |
| bytes | live length; 0 after expiry (pinned) |
| read | full + offset slices; out-of-range / negative → rc 1 "SQL logic error", buffer untouched, no partial transfer (pinned) |
| write | at offset, visible to SQL, length unchanged; read-only handle → rc 8; write past end refused; zeroblob(N) preallocate + interior write (all pinned) |
| reopen | repositions to another rowid; missing rowid errors + aborts the handle (pinned) |
| expiry | UPDATE or DELETE of the row on the same connection → rc 4 "query aborted", bytes → 0 (pinned both ways) |
| durable + interop | handle writes persist across reopen (integrity ok); WAL-mode files work without deepening WAL |
| text cells | open + read fine ("texty", pinned) |

One engine hole opened by the pins: `UPDATE ... SET d = zeroblob(4)` — constant
expressions on the right-hand side now evaluate (mirrors run-34's INSERT fix).

## 3. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| blob-io-api-001 | none | **partial** | attached-schema / UTF-16 name forms, WITHOUT ROWID targets, open-inside-transaction interactions — unpinned |
| blob-io-api-002 | none | **partial** | expiry granularity is connection-write, not per-row like C (every pinned case modifies the handle's own row, so the pins cannot tell — the card says so); TEXT-cell writes unpinned |
| engine-blob-001/002/003 | — | **new full ×3** | composed cards for exactly the frozen batches |

Stretch cards (close_v2, get_table, status counters) skipped per charter.

## 4. Anti-cheat + C interop + cargo

- `anti_cheat_blob_runtime` — pid-seeded table + payload, runtime-chosen offset
  write/read-back, then the expiry proof: touching the row kills the live handle
  (rc 4, bytes 0). A copied-buffer fake would survive; a script table could not
  produce the runtime bytes.
- `rust_blob_write_c_read` — a Rust file whose bytes were written ONLY through a
  handle is read by the pinned C CLI (integrity ok).
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 565/565 PASS** (was 541; +22 golden twins, +2 anti-cheat/interop).
  engine-vacuum / engine-wal / upsert-expr / collation / utf16 / harvest23 all green;
  528 pre-run goldens md5-verified intact.

## 5. Scoreboard (impl_in_modern) — before → after

| State | Run 34 | Run 35 |
| --- | --- | --- |
| **full (converted)** | 112 | **115** |
| partial | 46 | 48 |
| none (remaining) | 99 | **97** |
| behaviours known | 257 | 260 |

legacy_green 167 → 170. parity_green 0. Nothing `verified`. WAL/VACUUM claims unchanged.

## 6. Not migrated

SQLite is **not migrated**. 115/260 behaviours run honestly in modern for frozen
scope only. Blob expiry is connection-write granular (documented); no btree/VDBE
port. Parity UNVERIFIED everywhere (COMPARE never run).

## 7. Next call

1. **connection-lifecycle none crumbs** — close BUSY vs close_v2 zombie
   (single-process pins), get_table/free_table marshalling, status counters.
2. **analyze-stats none** — ANALYZE + sqlite_stat1 as real store output.
3. **vacuum deepen** — URI INTO targets + attached-schema forms to finish vacuum-002.
