# session-001 — Session lifecycle and change recording

Slice: `session` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Create/attach/generate-changeset over tracked tables; indirect-change flags.

## Entrypoints (citations)

- `sqlite3session_create()` (api) — `ext/session/sqlite3session.c:2362`, `ext/session/sqlite3session.c:2473`, `ext/session/sqlite3session.c:3213`

## Inputs / outputs / observables

- sqlite3session object lifecycle; changeset blob sizes; indirect flag; table filtering via attach patterns

## Behaviour (as implemented)

- session_create (ext/session/sqlite3session.c:2362) hooks the connection's preupdate hooks; attach (:2473) selects tables (NULL=all); changes recorded as before/after images keyed by PK; changeset (:3213) serializes INSERT/UPDATE/DELETE per table with PK-ordered values

## Validation rules found in code

- Tables without declared PK are ignored (documented limitation)
- Session must be on the connection making changes

## Edge cases found in code

- Indirect changes (triggers/FK actions) flagged when session->indirect set; patchset omits old-values for smaller payloads

## Dependencies

- connection-lifecycle-api

## Assumptions / unknowns

- Requires SQLITE_ENABLE_SESSION + PREUPDATE_HOOK — gate documented vs baseline

## Evidence

- `ext/session/sqlite3session.c:2362`
- `ext/session/sqlite3session.c:2473`
- `ext/session/sqlite3session.c:3213`
