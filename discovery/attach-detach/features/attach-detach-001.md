# attach-detach-001 — ATTACH DATABASE

Slice: `attach-detach` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

ATTACH compiles to internal function call; enforces SQLITE_LIMIT_ATTACHED, duplicate-name errors, URI handling.

## Entrypoints (citations)

- `sqlite3Attach()` (other) — `src/attach.c:449`, `src/attach.c:74`

## Inputs / outputs / observables

- PRAGMA database_list rows; error messages ('too many attached databases', 'database X is already in use'); schema qualifier resolution

## Behaviour (as implemented)

- ATTACH compiles to an internal function call (sqlite3Attach src/attach.c:449 → attachFunc src/attach.c:74): opens the file as a new Btree, loads its schema, appends to db->aDb under the given name
- Name resolution: 'main', 'temp' reserved; expression forms allowed for filename/name (evaluated at runtime)

## Validation rules found in code

- SQLITE_LIMIT_ATTACHED enforced (default 10, hard max 125)
- Duplicate schema name → error; same file attached twice is allowed but dangerous (documented)

## Edge cases found in code

- ATTACH inside a transaction is allowed; the new db joins the transaction with its own journal
- Encrypted/URI filenames honor the same open machinery as open_v2

## Dependencies

- connection-lifecycle-api
- ddl-schema

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/attach.c:449`
- `src/attach.c:74`
