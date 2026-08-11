# backup-api-001 — Backup lifecycle (init/step/finish)

Slice: `backup-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Handle-to-handle page copy in nPage batches; BUSY/LOCKED propagation; finish returns first error.

## Entrypoints (citations)

- `sqlite3_backup_init()` (api) — `src/backup.c:141`, `src/backup.c:325`, `src/backup.c:598`, `src/sqlite.h.in:9822`

## Inputs / outputs / observables

- backup_init returns handle or NULL (error retrievable from DEST db errcode/errmsg); step returns SQLITE_OK/DONE/BUSY/LOCKED/READONLY/NOMEM; finish returns first persistent error

## Behaviour (as implemented)

- sqlite3_backup_init (src/backup.c:141) validates src≠dest and registers the backup on the source pager; step(nPage) (src/backup.c:325) copies up to nPage pages (negative = all), handling page-size differences by growing/truncating dest
- finish (src/backup.c:598) detaches and returns the sticky error (BUSY/LOCKED are not sticky)

## Validation rules found in code

- Destination must not be in WAL mode with a different page size (READONLY error path)
- Same-connection src==dest → SQLITE_ERROR at init

## Edge cases found in code

- If the ENTIRE source db is copied in one step() and the source is not locked by others, the copy is transactionally clean; incremental copies restart when the source changes (see 003)
- Dest connection must hold no other transaction — else SQLITE_BUSY

## Dependencies

- pager
- btree

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/backup.c:141`
- `src/backup.c:325`
- `src/backup.c:598`
- `src/sqlite.h.in:9822`
