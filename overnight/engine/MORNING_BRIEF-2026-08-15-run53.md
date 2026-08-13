# MORNING BRIEF — engine v43: the kitchen 12-hour close (run 53, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–52 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v43-kitchen-12h, FORBID_TEMP_ALIASED_TO_MAIN,
FORBID_FAKE_AUTH_MASTER, AUTH001_NO_FULL, DEEPEN_JSONB probe-then-maybe. MAX_NEW_CASES 40 (used 10).

## 1. Pack @43 BOUND — KITCHEN-12H law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v42 → **v43**
(versions/1–43 retained; ADR `0041-engine-v43-kitchen-12h.md`; schema VALID; 49 laws).

## 2. JSONB probe

`jsonb('[]')` and `jsonb_extract('[1]','$')` both work → **JSONB is PRESENT on the pin**.
Per the charter, JSONB is NOT implemented this pack; json-funcs-002 stays partial with JSONB
as its single residual. Only the array-path half lands.

## 3. Headlines landed

| Slice | Outcome |
| --- | --- |
| **JSON array-path** (json-funcs-002) | `$.a[N]`/`$[N]` set/replace overwrite, out-of-range is a no-op for set/insert/replace, `json_remove` shifts the remainder, `$.a[#]` append / `$.a[#-K]`, nested `$.a[N].b`, NULL-doc → NULL. Card stays **partial** (JSONB residual). |
| **pragma index_list / foreign_key_list** (pragma-surface-002 → **FULL**) | real projections (seq/name/unique/origin/partial and id/seq/table/from/to/on_update/on_delete/match) for PRAGMA + TVF forms; reverse order, synthesized autoindexes, composite FK seq, IPK zero-rows. Last residual cleared. |
| **TEMP schema** (composed) | CREATE TEMP/TEMPORARY TABLE under schema `temp` (never aliased to main), temp-first unqualified resolution, `main.`/`temp.` qualified, `sqlite_temp_master` real SELECT, unqualified-drop temp precedence, **TEMP triggers fire** on TEMP DML. |
| **auth TEMP codes** (auth-001 stays partial) | CREATE_TEMP_TABLE (4) / DROP_TEMP_TABLE (13) dispatched (s1=table, s3=temp, DENY rc 23), no catalog tail. |

## 4. TEMP scope

TEMP is a real per-connection `temp.` schema: isolated from main both ways, shadows same-named
main tables for unqualified access, excluded from the durable file image (so a file reopen loses
TEMP). Cross-schema TEMP-trigger fire and non-trivial trigger bodies stay named on attach-003.

## 5. Parked list unchanged

VDBE/EXPLAIN (prepare-006), planner/flattening (select-codegen-001/003), pager/btree/WAL-multi,
parse.y (parser-grammar-001/tokenizer-001), va_list, dlopen, compile-option other-builds,
xBestIndex trio (series/prefixes/wholenumber), zlib-byte compress, full NFA regexp, mutex plugins,
util hash/ChaCha20, CACHE_SPILL/scanstatus, FTS/rtree/session, ENABLE-off nones — all untouched.

## 6. Anti-cheat

A runtime JSON array index mutates the matching element; a runtime table name appears in
pragma_index_list; a runtime TEMP table is isolated from a same-named main table (unqualified
resolves temp, `main.` resolves main). All in `modern/tests/engine_harvest43.rs`.

## 7. Cargo

`cargo test` (53 binaries): **all green**, including the JSON, pragma-TVF, attach33 qualified-name,
harvest41 auth-filter and harvest42 WITH/RECURSIVE suites. `SCRIPT_TABLE.len()==0`.

## 8. Freezes

10 new cases under `engine-harvest43/` (JSON ×3, pragma ×2) and `engine-temp43/` (TEMP ×3, auth ×2),
two-run deterministic, delegated HUMAN_ACCEPTED, legacy_green 250, prior golden md5s untouched.

## 9. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 220 | **225** |
| partial | 42 | 41 |
| none | 78 | 78 |
| behaviours | 340 | 344 (+4 composed) |

partial→full: pragma-surface-002 (estate) + engine-harvest43-002 / engine-temp43-* (composed).
Still partial: json-funcs-002 (JSONB), auth-callback-api-001 (TEMP index/trigger/view + master),
attach-detach-003 (cross-schema TEMP fire + bodies).

## 10. Not migrated

SQLite is **not migrated**. JSONB binary format, planner/flattening, VDBE, pager/btree/WAL-multi,
parse.y, the DDL authorizer bookkeeping walks and cross-schema TEMP remain partial or absent.

## 11. Next call

VFS-001 none-cut (wave 2): the in-memory VFS shim surface — probe which sqlite3_vfs entry points
the bare pin exposes and whether a honest register/find/default round-trip is pinnable without a
real OS backend. Then TEMP index/view auth codes once TEMP DDL widens.
