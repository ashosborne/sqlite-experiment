# backup-api-003 — Concurrent source-write coordination

Slice: `backup-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Writes to source db mid-backup patch pages already copied (BackupUpdate) or restart the copy (BackupRestart).

## Entrypoints (citations)

- `sqlite3BackupUpdate()` (other) — `src/backup.c:715`, `src/backup.c:730`

## Inputs / outputs / observables

- Backup restarts (progress resets) when the source db is written by another connection; same-page patches when written through the source connection

## Behaviour (as implemented)

- sqlite3BackupUpdate (src/backup.c:715) patches already-copied pages when the source connection itself writes mid-backup
- sqlite3BackupRestart (src/backup.c:730) resets the copy when a different connection commits to the source

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Backup handle attached to source pager list — close ordering enforced by connection-lifecycle close rules

## Dependencies

- pager

## Assumptions / unknowns

- Characterization needs a two-connection harness (run-1 flag stands)
- Characterizable only via concurrent-writer harness — flag for test design

## Evidence

- `src/backup.c:715`
- `src/backup.c:730`
