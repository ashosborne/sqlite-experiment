# backup-api-002 — Backup progress introspection

Slice: `backup-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

remaining()/pagecount() report progress after each step.

## Entrypoints (citations)

- `sqlite3_backup_remaining()` (api) — `src/backup.c:654`, `src/backup.c:668`

## Inputs / outputs / observables

- remaining() and pagecount() values updated only by step() calls

## Behaviour (as implemented)

- sqlite3_backup_remaining (src/backup.c:654) = pages still to copy after last step; pagecount (src/backup.c:668) = total source pages at last step

## Validation rules found in code

- Values are stale between steps by design (no locking to read them)

## Edge cases found in code

- Source growth/shrink between steps updates pagecount at the next step

## Dependencies

- backup-api-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/backup.c:654`
- `src/backup.c:668`
