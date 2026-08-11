# ddl-schema-003 — ALTER TABLE family (rename/add/rename-col/drop-col)

Slice: `ddl-schema` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Schema rewriting with cross-object reference fixup (views/triggers referencing renamed entities).

## Entrypoints (citations)

- `sqlite3AlterRenameTable()` (other) — `src/alter.c:124`, `src/alter.c:313`, `src/alter.c:599`, `src/alter.c:2250`

## Inputs / outputs / observables

- ALTER TABLE RENAME/ADD COLUMN/RENAME COLUMN/DROP COLUMN effects incl. rewritten sqlite_master text of referencing views/triggers

## Behaviour (as implemented)

- RenameTable (src/alter.c:124) rewrites references estate-wide via the rename-token machinery (views, triggers, FKs, indexes); AddColumn (src/alter.c:313) appends to the CREATE text (no table rewrite) with constant/NULL default restriction; RenameColumn (src/alter.c:599) token-rewrites all referencing SQL; DropColumn (src/alter.c:2250) rewrites the row data as needed and errors if the column is PK/indexed/referenced

## Validation rules found in code

- ADD COLUMN cannot add PRIMARY KEY/UNIQUE, non-constant default on non-empty table rules
- DROP COLUMN refuses columns used in indexes, PKs, triggers, views, CHECKs, FKs

## Edge cases found in code

- legacy_alter_table pragma restores pre-3.25 rename semantics (no reference fixup) — dialect gate (run-1 question)
- Errors mid-ALTER roll back the whole schema change

## Dependencies

- ddl-schema-001

## Assumptions / unknowns

- legacy_alter_table pragma compatibility mode relevant?

## Evidence

- `src/alter.c:124`
- `src/alter.c:313`
- `src/alter.c:599`
- `src/alter.c:2250`
