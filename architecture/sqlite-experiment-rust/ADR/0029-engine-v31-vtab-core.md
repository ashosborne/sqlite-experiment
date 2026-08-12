# ADR 0029 — engine v31: vtab core (real module registration, declare_vtab, cursor SELECT)

Status: accepted · Date: 2026-08-13 · Operator: Ash Osborne · Pack: v31 (supersedes v30)

## Context

After run 40 (attached-schema ownership) the highest-leverage still-`none` core seam was
virtual tables: `vtab-core-001` (module lifecycle) and `vtab-core-002` (declare_vtab shape)
were both `impl_in_modern: none`. harvest28 gave `wholenumber`/`completion` pins rows via a
private generator keyed off `conn.vtabs` — CREATE VIRTUAL TABLE only recorded a name; no
module methods existed, no declare_vtab, no cursor. That shortcut is explicitly *not* the
Done bar: this run makes registration + CREATE VIRTUAL TABLE + declare_vtab + the cursor
SELECT path real.

Baseline honesty: `sqlite3_create_module`/`_v2`, `sqlite3_declare_vtab` and the whole
`sqlite3_module` method table are core C API on the bare amalgamation (`src/vtab.c`), so no
extension force-linking is involved. The pinned module is a tiny in-harness test module
(`intseries` — a bounded integer series with a HIDDEN `lim` column), compiled into the C
characterization harness and re-implemented as `extern "C"` functions in the Rust twin, the
same on both sides.

## Decision

Modern (Rust) grows a real C-ABI vtab seam:

- `Sqlite3Module` — `#[repr(C)]` function-pointer table matching C's `sqlite3_module`
  (iVersion, xCreate, xConnect, xBestIndex, xDisconnect, xDestroy, xOpen, xClose, xFilter,
  xNext, xEof, xColumn, xRowid, xUpdate, … v1 tail). `Sqlite3Vtab` / `Sqlite3VtabCursor`
  match C's base-struct layouts (module pointer + nRef + zErrMsg; cursor → pVtab) so module
  code can embed them as first members exactly as in C.
- `sqlite3_create_module(db, name, module, client_data)` and
  `sqlite3_create_module_v2(..., xDestroy)` register on a per-connection registry.
  Redefining a name replaces the entry and calls the previous `_v2` destructor;
  `sqlite3_close` runs remaining destructors.
- `CREATE VIRTUAL TABLE t USING m(args…)` looks the module up; unknown name errors
  `no such module: m` (C-exact). A found module gets `xCreate(db, pAux, argc, argv, &pVtab,
  &pzErr)` with C's argv convention (module, db-name, table-name, raw args). Constructor
  failure surfaces the module error and leaves no schema entry. Success records a real
  schema entry (visible to `sqlite_master` counting pins) plus the instance (vtab pointer +
  declared shape).
- `sqlite3_declare_vtab(db, "CREATE TABLE x(…)")` is only legal while an xCreate/xConnect
  is on the stack (else `SQLITE_MISUSE`); it parses the column list and fixes the vtab's
  column names/types. `HIDDEN` columns are recorded and excluded from `SELECT *` expansion
  while remaining selectable by name.
- SELECT through a vtab drives the module cursor: `xOpen` → `xBestIndex` (offered zero
  constraints — full scan) → `xFilter(idxNum=0)` → loop `xEof`/`xColumn` (per declared
  column, results delivered through the existing `Sqlite3Context` result API)/`xNext` →
  `xClose`. WHERE/ORDER BY/aggregates/JOIN run engine-side on the materialized scan.
- `DROP TABLE t` on a vtab invokes `xDestroy` and removes instance + schema entry.

## Methods landed / residuals

Landed: xCreate, xConnect (only as create alias for the pinned module — schema-reload
xConnect on reopen is NOT pinned; :memory: pins re-register per connection), xBestIndex
(invoked, zero-constraint full scan only), xOpen, xClose, xFilter, xNext, xEof, xColumn,
xDestroy, `_v2` destructor on replace and close.

Deliberate residuals (documented, not claimed):
- xBestIndex constraint pushdown / cost solving; HIDDEN-column constraints via
  xFilter argv (`vtab-core-002` residual).
- `sqlite3_vtab_config` negotiation (CONSTRAINT_SUPPORT / INNOCUOUS / DIRECTONLY) — not
  pinned, not claimed.
- xUpdate (writes through vtabs), xRowid-dependent paths, xRename, xSavepoint family,
  eponymous-only modules (xCreate==NULL), xConnect on schema reload for file DBs.
- Absent ext/misc modules (csv, zipfile, dbstat, …) stay unclaimed; misc-vtab-packs-001
  umbrella stays `none`.

## Relationship to harvest28 generators

`wholenumber`/`completion` harvest28 pins keep passing unchanged (their generator path is
still keyed off `conn.vtabs` for the `wholenumber` module name). They are NOT re-homed onto
the new registry this run — the C harness cannot register the real ext/misc wholenumber
module without force-linking an extension, so their pins stay as recorded and the cards
keep their harvest28 notes. The new law only governs user-registered modules.

## Consequences

- vtab-core-001 / vtab-core-002 flip none → partial with precise residuals.
- Composed cards `engine-vtab31-*` pin the exact frozen batches.
- SCRIPT_TABLE stays empty; anti-cheat pins prove runtime module/table names and payloads.
