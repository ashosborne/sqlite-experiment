# ADR 0036 — engine v38: vtab lifecycle deepen (xConnect reload, drop_modules, xUpdate)

Status: accepted · Date: 2026-08-14 · Operator: Ash Osborne · Pack: v38 (supersedes v37)

## Probe results (bare pin, before freezing)

- **xConnect vs xCreate on reopen**: a file db keeps the CREATE VIRTUAL TABLE schema row
  (rootpage 0 + sql). Reopen without re-registering → `no such module: ser` at prepare.
  Re-register + SELECT → **xConnect 1 / xCreate 0** with the C argv convention. Close
  runs xDisconnect (never xDestroy); DROP runs xDestroy.
- **drop_modules**: the argument is the KEEP list (C drops everything else). After a
  drop, new `CREATE VIRTUAL TABLE` fails `no such module`; a live instance **still
  scans**; and `DROP TABLE` of that instance also fails `no such module` (C needs the
  registration to destroy). rc 0 in all pinned shapes.
- **destructor deferral**: replacing a `_v2` registration while an instance is live
  fires **nothing** (0); the deferred destructor fires when the instance drops (1) and
  the replacement's at close (2). Modern's registrations are now refcounted (base ref +
  one per live instance) to reproduce exactly this.
- **xUpdate argv shapes** (pinned via an in-module log): INSERT argc=N+2 with argv[0]
  NULL and argv[1] NULL (module assigns the rowid, `last_insert_rowid` follows *pRowid)
  or an explicit integer; UPDATE argv[0]=argv[1]=rowid with new column values; DELETE
  argc=1 rowid. A module `SQLITE_CONSTRAINT` surfaces rc 19 `constraint failed`.
- **eponymous-only** (xCreate == NULL): bare-name SELECT connects (xConnect 1); `CREATE
  VIRTUAL TABLE ... USING epo` refuses `no such module: epo`; no schema row.

## What cleared on vtab-core-001

| Leftover (run 41) | Status |
| --- | --- |
| xConnect on schema reload for file DBs | **CLEARED** — durable schema rows in real file images + pending reconnect |
| sqlite3_drop_modules | **CLEARED** — keep-list contract incl. live-instance edges |
| deferred destructor while instances hold the module | **CLEARED** — refcounted registrations, pinned 0/1/2 |
| xUpdate (of the xUpdate/xRename/xSavepoint family) | **CLEARED** — writable vtabs with C argv shapes |
| eponymous-only modules (xCreate==NULL) | **CLEARED** — the optional target was cheap after the connect plumbing |
| xRename / xSavepoint / xRelease / xRollbackTo | **STAY NAMED** — not attempted (charter default) |

vtab-core-001 therefore stays **partial** (the law forbids full while any family
member remains named): residual is now exactly the xRename/xSavepoint family.

## Modern design notes

- Vtab schema rows live in `Conn.vtab_schema` and persist through the shared dbfile
  writer/reader as type-table rows with rootpage 0 (C's shape, C-readable).
- Reopened entries are PENDING until first use: eval's source path connects on demand
  (xConnect), surfacing C's exact unregistered error; sqlite_master answers from the
  durable row even before the reconnect.
- Writable-vtab DML routes through a dedicated intercept (INSERT via the shared parser;
  UPDATE/DELETE via the pinned simple predicate forms) building the exact argv shapes;
  under-claim: complex WHERE forms on vtab DML fall outside the pinned scope.
- Scans now carry real xRowid values (SELECT rowid, DML row targeting).

## Scoreboard

before 185 full / 48 partial / 78 none of 311 → after **190 full / 48 partial / 78 none
of 316** (5 composed engine-vtab38 cards; vtab-core-001 stays an honest partial with a
one-line residual). SQLite is NOT migrated.
