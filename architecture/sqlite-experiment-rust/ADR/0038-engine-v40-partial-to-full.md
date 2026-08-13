# ADR 0038 — engine v40: partial-to-full harvest (vtab lifecycle, legacy trace, db_config toggles, json_tree, completion)

Status: accepted (run 50, pack v40)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

The v36/v37/v39 waves took the easy one-holes. Two cards still had a residual
list that could honestly empty in one focused implement (Tier A), plus three
probe-then-maybe cards whose residuals turned out to be genuinely closable. This
run probed C first, froze 23 goldens (engine-harvest40-001..008), implemented
the behaviours, and flipped only where the residual list is empty. SQLite is NOT
migrated.

## Probes (all on the pinned bare build, two-run deterministic)

### vtab xRename / xSavepoint family (vtab-core-001)

| Probe | C answer | Pin |
|---|---|---|
| `ALTER TABLE vt RENAME TO wt` with an xRename module | xRename fires with `wt`; `sqlite_master.sql` becomes `CREATE VIRTUAL TABLE "wt" USING ser`; old name gone | 40-001-C001 |
| rename a module WITHOUT xRename | still renames, no callback | 40-001-C002 |
| `BEGIN; INSERT; SAVEPOINT s1; INSERT; SAVEPOINT s2; INSERT; ROLLBACK TO s1` | `[begin][ins][svpt:0][ins][svpt:1][ins][rbto:0]` then only the pre-s1 row survives; RELEASE s1 = `[release:0]`; COMMIT = `[sync][commit]` | 40-002-C001 |
| a vtab that first writes AFTER two SAVEPOINTs are open | catch-up `[begin][svpt:0]`; the txn savepoint is excluded from numbering; ROLLBACK TO below the join passes `-1`; RELEASE of the txn savepoint commits (`[sync][commit]`, no xRelease) | 40-002-C002 |
| autocommit vtab DML then explicit `ROLLBACK` | statement txn `[begin][ins][sync][commit]`; ROLLBACK fires `[rollback]` and the module restores its base | 40-002-C003 |

### legacy trace/profile + WITHOUT ROWID / truncate (connection-lifecycle-api-004)

`sqlite3_trace` / `sqlite3_profile` **are exported** on this pin. They share one
slot: installing profile displaces trace; trace_v2 displaces both; clearing v2
silences everything. Legacy trace text has bound parameters **expanded**
(`SELECT a FROM t WHERE a = 1`). The `update_hook` fires 3 events on a rowid
table and **0** on a WITHOUT ROWID table; `DELETE FROM r` (no WHERE) fires **0**
hook events with `changes()==3` (truncate fast-path), while `DELETE ... WHERE`
fires per row. Pins 40-003 / 40-004.

### json_each / json_tree full columns (json-funcs-004)

Eight vtab columns (key/value/type/atom/id/parent/fullkey/path). `id` is the
node's **byte offset in C's JSONB encoding** of the whole document (single-digit
ints = 2 bytes after a 1-byte array header); `parent` references the parent
ROW's id; container `value`s are minified; a second path argument roots the
walk. Pins 40-005 + a runtime-document id-offset anti-cheat.

### db_config leftover toggles (global-init-config-003)

Each named leftover has a real, observable effect on the pin (40-006):
DQS_DDL (CHECK string gate), WRITABLE_SCHEMA (sqlite_master UPDATE gate),
**DEFENSIVE enforced** (no-ops PRAGMA writable_schema and keeps the gate shut),
LEGACY_ALTER (skips the view-SQL rename rewrite), RESET_DATABASE (VACUUM wipe),
TRUSTED_SCHEMA (refuses non-innocuous app functions inside views; INNOCUOUS and
direct calls allowed), and the load-extension C-API vs SQL-function gate split.

### live completion phases (misc-completion-001)

Live C's `completionNext` emits phases **1|7|8|9** only — keywords (147-entry
census), databases, tables+views, columns. The pragma/function/collation/index/
trigger phases (2–6) are **never entered** and there is no ranking. The residual
"functions/pragmas/collations phases, wildcard ranking" is therefore PIN-ABSENT,
not merely unimplemented. Pins 40-007 + a runtime-named-table phase-8 anti-cheat.

### pragma_index_xinfo (Tier B, pragma-surface-002 stays partial)

index_xinfo adds desc/coll/key and the trailing rowid entry (cid -1, key 0),
one row more than index_info. Pins 40-008.

## Presence / pin-absent record

- legacy `sqlite3_trace`/`sqlite3_profile`: **present** on the pin → implemented.
- completion phases 2–6 and ranking: **pin-absent in live C** → residual deletable.
- JSONB storage form (json-funcs-002): still named, not a json_each surface.
- memdb URI attach (serialize-memdb-api-002): USE_URI off — untouched (ADR 0035).
- STAT4 / scanstatus / VM_STEP magnitudes: pin-absent / no VDBE — untouched.

## Estate outcome

| Card | Outcome |
|---|---|
| vtab-core-001 | **partial → full** — xRename + xSavepoint/xRelease/xRollbackTo family cleared; residual empty |
| connection-lifecycle-api-004 | **partial → full** — WITHOUT ROWID suppression + truncate fast-path + legacy trace/profile cleared |
| json-funcs-004 | **partial → full** — json_tree + full vtab columns |
| global-init-config-003 | **partial → full** — the whole toggle family with real effects (DEFENSIVE now enforced) |
| misc-completion-001 | **partial → full** — live-C phases pinned; dead phases + ranking pin-absent |
| pragma-surface-002 | stays partial — index_xinfo added; remaining result pragmas remain |
| engine-harvest40-001..008 | composed full for the frozen batches |

Deliberate stay-partials (structural, untouched or deepened-only): WAL, planner/
VDBE, prepare-006 EXPLAIN bytecode, pragma-surface-001 census, auth remaining
codes (no faked sqlite_master sequences), error-status CACHE_SPILL/scanstatus,
malloc mini-slots, series/prefixes/wholenumber (ADR 0034 no re-home), zlib,
va_list, dlopen, attach mazes, hash tables.

Engine bug fixed en route: kitchen UPDATE/DELETE against `sqlite_master` and the
DQS/DEFENSIVE gates are now enforced pre-parse; the run-49 `WHERE rowid=N` fix
stands.

Scoreboard: 195 full / 47 partial / 78 none of 320 → **208 full / 42 partial /
78 none of 328** (+8 composed behaviours). `SCRIPT_TABLE.len()==0`. Prior
harvest36/37/39, vtab38, lookaside35, status34/pragma34 suites all green.
