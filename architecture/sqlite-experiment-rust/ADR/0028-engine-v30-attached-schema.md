# ADR 0028 — engine v30: attached-schema ownership

Status: accepted (pack v30 BOUND, Ash Osborne, delegated autonomy run 40)

## Context (plain language)

Until now ATTACH was just a name in a list; attached schemas could not own tables, so
attach-detach-001/002 were partial and attach-detach-003 could only be proven on the main
schema. This run makes a second schema a real object namespace: ATTACH opens a schema that
can CREATE/INSERT/SELECT tables, DETACH drops it, and qualified names resolve across schemas.

## Baseline presence

ATTACH/DETACH and cross-db name fixation are CORE (bare-amalgamation) behaviour — the C
probe (`/tmp/attach_harness.c` against `sqlite3.c` alone) drives every pin. No extensions.

## What landed

- **Ownership:** attached tables live under `schema.table` store keys. `ident()` accepts
  `schema.table` (main/temp normalize to the bare key). CREATE/INSERT/SELECT/UPDATE/DELETE
  on `aux.t` all work; two attached schemas each own their own tables.
- **Resolution:** `eval_snapshot` exposes each table under its key plus a qualified/
  unqualified alias, so `aux.t`, bare `t` (when main has none), `main.m` and `m` all
  resolve. A name in both main and aux resolves to main (pinned).
- **Errors:** duplicate / reserved ATTACH → "database X is already in use"; DETACH main →
  "cannot detach database main"; DETACH missing → "no such database: X".
- **Teardown:** DETACH removes the schema and all its `schema.*` tables/indexes/catalog
  entries; later qualified access fails. Re-ATTACH gives a fresh empty schema.
- **Durability:** a file-backed attachment loads on ATTACH and is written back on
  DETACH/close via the shared dbfile writer, so create-then-reopen (re-ATTACH) sees the
  rows. The main-db image excludes attached tables.
- **attach-003 aux residual reclaimed:** a non-TEMP trigger in/for an attached schema
  still rejects qualified DML, and an attached-schema VIEW referencing another schema
  errors "view vv cannot reference objects in database main".
- **Incidental fix:** `ORDER BY <unknown/non-projected column>` no longer mis-sorts by
  column 0 (it is skipped); this made `SELECT name FROM pragma_database_list ORDER BY seq`
  order correctly, and is the honest general behaviour.

## Deliberate residuals

Firing a trigger whose body targets an attached table (unqualified body resolution during
trigger execution) is not implemented — dropped `engine-attach30-003-C003`. DETACH
"database is locked" while a statement is open, the URI/encryption attach maze, and
transaction-join semantics across schemas are not claimed.

## Consequences

14 goldens replay byte-identical; anti-cheat proves a runtime schema/table round-trips and
that a detached schema's table is gone; cargo 668/668; all prior goldens intact.
attach-detach-001 → tighter partial (ownership real; URI/lock/txn edges residual);
attach-detach-002 → tighter partial (teardown real; lock edges residual); attach-detach-003
→ partial (aux cross-db fixation reclaimed; trigger-firing-on-attached residual).
