# ADR 0037 — engine v39: mixed one-hole wave (STMT/PROFILE per statement, pragma_module_list, per-row blob expiry, sqlite3_randomness)

Status: accepted (run 49, pack v39)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

Run 48 left four named one-holes on partial estate cards. Pack v39 binds the
MIXED ONE-HOLE LAW: a card flips `impl_in_modern: full` only when every named
residual on it clears on the pin (or is proven pin-absent here). This run probed
all four, froze 10 goldens (engine-harvest39-001..004), and implemented them in
modern. No planner/WAL claims; SQLite is NOT migrated.

## Probe results (all on the pinned bare build, two-run deterministic)

| Probe | C answer | Pinned |
|---|---|---|
| `sqlite3_exec("SELECT 1; SELECT 2;")` with trace mask STMT\|PROFILE | 2 STMT + 2 PROFILE, texts `SELECT 1;` / `SELECT 2;` (terminator kept, leading space dropped) | 39-001-C001 |
| three-statement exec | 3 STMT + 3 PROFILE in statement order | 39-001-C002 |
| prepared `SELECT 3` stepped to completion | 1 STMT at execution + 1 PROFILE at completion, text without terminator (PROFILE callback reads it via `sqlite3_sql` on the handle) | 39-001-C003 |
| `pragma_module_list` on a fresh connection | exactly one row: `pragma_module_list` — its own query lazily instantiates its pragma vtab; NOT a module census | 39-002-C001 |
| after `sqlite3_create_module("mymod")`, then after touching `pragma_collation_list` | `mymod\|pragma_module_list`, then `mymod\|pragma_collation_list\|pragma_module_list` — user registry is live, pragma vtabs join on first use (ADR 0032's fragility resolved by pinning the lazy contract itself) | 39-002-C002 |
| blob handle on row 1; UPDATE row 2 | handle still reads (rc 0, bytes 5, correct byte) AND still writes (rc 0) — expiry is PER ROW, not connection-write | 39-003-C001 |
| then UPDATE row 1 | expired: rc 4 (SQLITE_ABORT), bytes 0 | 39-003-C001 |
| DELETE other row / own row | live rc 0 / expired rc 4 bytes 0 | 39-003-C002 |
| blob_open on a TEXT cell, write 5 bytes | open rc 0 bytes 11, write rc 0, cell reads `HELLO world` with `typeof` still `text` | 39-003-C003 |
| `sqlite3_randomness`: N=8 fill, second draw, N=0 | exactly N bytes written (tail untouched), draws differ, N=0 writes nothing and the stream continues | 39-004-C001 |
| `sqlite3_test_control` PRNG ops presence | SAVE(5)/RESTORE(6) rc 0 — restore replays the 8-byte draw, next draw differs; PRNG_SEED(28) rc 0 — same seed replays the draw | 39-004-C002 |

Honesty notes:
- **No unseeded PRNG bytes were frozen.** All 004 pins are predicates (fill
  width, difference, within-run replay equality). Modern's generator is a
  seeded xorshift stream, not C's ChaCha20 — raw byte parity is intentionally
  unpinned and the card says so.
- **module_list is the lazy contract, not a census.** The pins freeze fresh /
  after-create_module / after-first-TVF-use transitions with a harness module;
  no ext/misc force-link, no freezing of "the TVFs the probe already touched".

## Implementation (modern)

- `execute_script` fires STMT before and PROFILE after each split statement
  (terminated text keeps its `;` like C); prepared statements fire their own
  events from `sqlite3_step` with the real handle, and a suppression guard
  stops the engine path double-firing under them. `sqlite3_sql` added.
- **Engine bug found by the blob probe:** kitchen `UPDATE`/`DELETE` silently
  dropped `WHERE rowid = N` (no such column ⇒ every row matched). Fixed:
  `rowid`/`_rowid_`/`oid` aliases hit the row's effective rowid (IPK-aware).
- Blob handles snapshot their cell at open; liveness compares the current cell
  (per-row by construction), own-write refreshes the snapshot, reopen
  re-snapshots. TEXT cells accept in-place byte patches and stay TEXT
  (non-UTF-8 results refused — unpinned edge, under-claim).
- `pragma_module_list` TVF returns the live `create_module` registry plus the
  pragma vtabs this connection has actually touched (recorded at TVF use,
  including itself). Nothing is pre-seeded.
- `sqlite3_randomness` is a real stateful stream (lazy entropy seed, then pure
  xorshift); `sqlite3_test_control` implements PRNG SAVE/RESTORE/SEED over it.

## Estate outcome

| Card | Outcome |
|---|---|
| blob-io-api-002 | **partial → full** — both named residuals (per-row expiry, TEXT-cell writes) cleared with pins + runtime other-row anti-cheat; residual list empty |
| connection-lifecycle-api-004 | stays partial — per-statement STMT/PROFILE cleared; WITHOUT ROWID suppression / truncate fast-path unpinned and legacy `sqlite3_trace`/`sqlite3_profile` not implemented |
| pragma-surface-002 | stays partial — pragma_module_list cleared (lazy contract); `index_xinfo` TVF form and remaining result pragmas remain |
| util-primitives-001 | stays partial — public `sqlite3_randomness` + testctrl PRNG cleared; string hash tables / internal hash primitives remain, and byte-stream parity with C's ChaCha20 is deliberately unclaimed |
| engine-harvest39-001..004 | composed full for the frozen batches |

Scoreboard: 190 full / 48 partial / 78 none of 316 → **195 full / 47 partial /
78 none of 320**. `SCRIPT_TABLE.len()==0` enforced by suite. Prior harvest36 /
harvest37 / vtab38 / lookaside35 / status34 / pragma34 suites all green.
