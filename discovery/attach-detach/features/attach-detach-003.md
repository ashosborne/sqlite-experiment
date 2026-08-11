# attach-detach-003 — Cross-database name fixation for DDL

Slice: `attach-detach` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3FixInit/sqlite3Fix* reject cross-db references inside triggers/views of attached dbs.

## Entrypoints (citations)

- `sqlite3FixInit()` (other) — `src/attach.c:533`, `src/attach.c:561`

## Inputs / outputs / observables

- Error 'view/trigger references objects in database X' style failures at DDL time in attached dbs

## Behaviour (as implemented)

- sqlite3FixInit + sqlite3Fix* walkers (src/attach.c:533) rewrite/validate names in DDL parsed from attached-db schemas so triggers/views cannot reference other databases

## Validation rules found in code

- Cross-db references inside CREATE TRIGGER/VIEW bodies loaded from a non-main schema → error at schema parse

## Edge cases found in code

- TEMP objects may reference any attached db (exempt from fixation)

## Dependencies

- ddl-schema

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/attach.c:533`
- `src/attach.c:561`
