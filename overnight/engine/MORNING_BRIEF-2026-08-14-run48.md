# MORNING BRIEF — engine v38: vtab lifecycle deepen (run 48, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–47 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v38-vtab-lifecycle, FORBID_GREENWASH_FULL,
NO_RECLAIM_VTAB002, NO_REHOME_SERIES. MAX_NEW_CASES 40 (used 9).

## 1. Pack @38 BOUND — VTAB LIFECYCLE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v37 → **v38**
(versions/1–38 retained; ADR `0036-engine-v38-vtab-lifecycle.md`; schema VALID; 44 laws).

## 2. Outcomes on the three named targets (+ two optional)

| Leftover | Outcome |
| --- | --- |
| **xConnect on schema reload** | CLEARED — the CREATE VIRTUAL TABLE row persists in the real file image (rootpage 0 + sql, C-readable); reopen without re-register errors `no such module: ser` exactly; re-register + SELECT runs **xConnect 1 / xCreate 0** with the argv convention; runtime-bound anti-cheat proves the argv round-trip through the reload |
| **sqlite3_drop_modules** | CLEARED — C's keep-list contract pinned: dropped names stop resolving, a live instance still scans, and `DROP TABLE` of it fails `no such module` (C needs the registration to destroy) |
| **xUpdate** | CLEARED — writable vtabs: INSERT with auto (argv[1] NULL → module assigns, `last_insert_rowid` follows *pRowid) and explicit rowids, UPDATE old/new rowid, DELETE argc=1; module storage drives subsequent scans; module `SQLITE_CONSTRAINT` → rc 19 `constraint failed`; the argv-shape log itself is pinned |
| deferred destructor (probe-together target) | CLEARED — registrations refcounted: replace-with-live-instance fires nothing (0), instance drop fires the deferred destructor (1), close fires the survivor's (2) |
| eponymous-only (optional) | CLEARED — cheap after the connect plumbing: bare-name SELECT connects, CREATE refuses `no such module`, no schema row |

**vtab-core-001 stays partial** per the law — the residual is now exactly one line:
the xRename / xSavepoint / xRelease / xRollbackTo family (not attempted, stayed named).

## 3. Modern changes

Durable vtab schema rows (dbfile `vtabs` + `Conn.vtab_schema`, both writers/reader);
pending-reconnect flow in the eval source path; refcounted module registrations
(`reg_new/addref/release`); `sqlite3_drop_modules`; writable-vtab DML intercept building
C's argv shapes; `sqlite3_last_insert_rowid` (new export, vtab inserts pinned); scans
carry real xRowid values (SELECT rowid / DML targeting); eponymous connect.
Under-claim noted in ADR: vtab DML WHERE forms beyond the pinned simple predicates.

## 4. Anti-cheat + goldens + cargo

- 9 new HUMAN_ACCEPTED goldens (engine-vtab38-001…005); prior 779 goldens untouched
  (v31/v36 vtab twins still green against the refcounted registry).
- 2 anti-cheat tests: a runtime series bound survives the file round-trip through
  xConnect argv (create=0/connect>0 asserted); a runtime payload written through
  xUpdate lands in the module's own storage and reads back.
- `cargo test` (modern): **835 passed / 0 failed** (was 827; +8). SCRIPT_TABLE.len()==0.

## 5. Scoreboard

before → after: **185 full / 48 partial / 78 none of 311** → **190 full / 48 partial / 78 none of 316**
(5 composed fulls; vtab-core-001 keeps an honest one-line residual; nones untouched).

## 6. Not migrated

SQLite is NOT migrated. vtab leftovers: xRename/xSavepoint family. Untouched per
charter: WAL multi-conn, planner/VDBE, pragma census, scalars, zlib, va_list,
mini-slots, attach-detach-003 TEMP fire, pager/btree/vfs/fts/wasm/jni/session/expert.

## 7. Next call (pick one)

1. **vtab xRename/xSavepoint family** — the last vtab-core-001 line (ALTER RENAME of a
   vtab + savepoint-wrapped vtab DML pins would close the card).
2. **auth action-code sweep** — the run-47 leftover (sqlite_master bookkeeping honesty
   decision).
3. **another one-hole sweep** — conn-lifecycle-004 STMT/PROFILE, pragma_module_list,
   blob per-row expiry, util PRNG APIs.
