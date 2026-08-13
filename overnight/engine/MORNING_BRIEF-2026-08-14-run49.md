# MORNING BRIEF — engine v39: mixed one-hole wave (run 49, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–48 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v39-oneholes, FORBID_GREENWASH_FULL,
NO_UNSEEDED_PRNG_GOLDEN, BLOB_EXPIRY_PER_ROW, STMT_PER_PREPARED, MODULE_LIST_LAZY.
MAX_NEW_CASES 40 (used 10).

## 1. Pack @39 BOUND — MIXED ONE-HOLE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v38 → **v39**
(versions/1–39 retained; ADR `0037-engine-v39-oneholes.md`; schema VALID; 45 laws).

## 2. The four probes (all landed; C answered on every one)

| Named one-hole | Probe answer on the pin | Outcome |
| --- | --- | --- |
| **STMT/PROFILE per statement** (connection-lifecycle-api-004) | `"SELECT 1; SELECT 2;"` via exec → **2 STMT + 2 PROFILE**, texts `SELECT 1;` / `SELECT 2;` (terminator kept); 3-statement script → 3+3; prepared stmt → 1+1 without terminator, PROFILE callback reads text via `sqlite3_sql` on the handle | CLEARED — card **stays partial** (WITHOUT ROWID / truncate fast-path unpinned; legacy `sqlite3_trace`/`sqlite3_profile` not implemented) |
| **pragma_module_list** (pragma-surface-002) | LAZY, exactly as ADR 0032 warned: fresh connection lists only `pragma_module_list` itself; `create_module("mymod")` joins immediately; `pragma_collation_list` joins after first use. Pinned the contract, not a census | CLEARED — card **stays partial** (`index_xinfo` TVF form; remaining result pragmas) |
| **per-row blob expiry + TEXT writes** (blob-io-api-002) | handle on row 1 survives UPDATE/DELETE of row 2 (reads AND writes, rc 0); own-row write/delete expires (rc 4, bytes 0); TEXT cell opens (11 bytes), takes a 5-byte patch, stays `text` | CLEARED — **partial → FULL** (residual list empty) |
| **sqlite3_randomness** (util-primitives-001) | fills exactly N (tail untouched), draws differ, N=0 writes nothing; testctrl PRNG SAVE/RESTORE replays the stream, same-seed PRNG_SEED replays — all predicates, **zero unseeded bytes frozen** | CLEARED — card **stays partial** (string hash tables / internal hash primitives; C ChaCha20 byte parity deliberately unclaimed) |

## 3. Modern changes

Per-statement trace: `execute_script` fires STMT/PROFILE per split statement with
C's terminator text; prepared statements fire from `sqlite3_step` with the real
handle (suppression guard stops double-firing); `sqlite3_sql` export added.
`pragma_module_list` TVF = live create_module registry + lazily-touched pragma
vtabs. Blob handles snapshot their cell (per-row expiry by construction; own
write refreshes; reopen re-snapshots); TEXT cells accept in-place patches and
stay TEXT. `sqlite3_randomness` is a real seeded xorshift stream;
`sqlite3_test_control` PRNG SAVE(5)/RESTORE(6)/SEED(28) landed.

**Engine bug the blob probe flushed out:** kitchen UPDATE/DELETE silently dropped
`WHERE rowid = N` and hit every row. Fixed (rowid/_rowid_/oid aliases, IPK-aware).

## 4. Anti-cheat

Runtime OTHER-row id that must NOT expire the handle (and own-row that must);
runtime-length script whose STMT count must match; runtime-named module that must
appear in pragma_module_list. All in `modern/tests/engine_harvest39.rs`.

## 5. Freezes

10 new cases under `tests/characterization/engine-harvest39/` (001 trace ×3,
002 module_list ×2, 003 blob ×3, 004 PRNG ×2), two-run deterministic, delegated
HUMAN_ACCEPTED. legacy_green 226. Prior golden md5s untouched.

## 6. Cargo

`cargo test` (workspace, 47 binaries): **all green**, including harvest36/37,
vtab38, lookaside35, status34/pragma34 suites. `SCRIPT_TABLE.len()==0` enforced.

## 7. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 190 | **195** |
| partial | 48 | 47 |
| none | 78 | 78 |
| behaviours | 316 | 320 (+4 composed) |

Partial→full: **blob-io-api-002** (estate), engine-harvest39-001..004 (composed).
Deepen-still-partial: connection-lifecycle-api-004, pragma-surface-002,
util-primitives-001 (residuals rewritten precisely).

## 8. Not migrated

SQLite is **not migrated**. WAL concurrency, planner/VDBE bytecode, pager/btree
internals, FTS, sessions, most pragmas and the wider C API remain unimplemented
or partial. This run closed four named one-holes; nothing more is claimed.

## 9. Next call

Options, in rough value order: (a) connection-lifecycle-api-004's remaining
families (legacy sqlite3_trace/profile shims + WITHOUT ROWID/truncate pins) —
would flip the card; (b) pragma-surface-002 `index_xinfo` TVF; (c) auth-callback
remaining action codes; (d) error-status-api-003 CACHE_SPILL/scanstatus.
