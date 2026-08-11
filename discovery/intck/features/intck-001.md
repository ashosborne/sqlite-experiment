# intck-001 — Incremental integrity-check lifecycle

Slice: `intck` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

open/step/unlock/message API running integrity checks in resumable units.

## Entrypoints (citations)

- `sqlite3_intck_open()` (api) — `ext/intck/sqlite3intck.c:801`, `ext/intck/sqlite3intck.c:849`, `ext/intck/sqlite3intck.c:907`

## Inputs / outputs / observables

- Per-step progress; intck_message text describing found corruption; unlock/suspend semantics

## Behaviour (as implemented)

- intck_open (ext/intck/sqlite3intck.c:801) targets one schema; step (:849) runs one generated check statement unit (equivalent coverage to PRAGMA integrity_check but resumable); message (:907) returns findings; unlock allows other writers between steps

## Validation rules found in code

- Requires no write transaction during a step burst

## Edge cases found in code

- Schema changes between steps restart affected object checks

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/intck/sqlite3intck.c:801`
- `ext/intck/sqlite3intck.c:849`
- `ext/intck/sqlite3intck.c:907`
