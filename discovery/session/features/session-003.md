# session-003 — Changeset algebra (iterate / invert / concat)

Slice: `session` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Iterator API over binary changeset format; invert and concat composition; patchset subset.

## Entrypoints (citations)

- `sqlite3changeset_start()` (api) — `ext/session/sqlite3session.c:3407`, `ext/session/sqlite3session.c:4308`, `ext/session/sqlite3session.c:6814`

## Inputs / outputs / observables

- Iterator walk (op,table,old/new values); inverted changeset semantics; concat ordering effects

## Behaviour (as implemented)

- changeset_start (ext/session/sqlite3session.c:3407) iterates the binary format; invert (:4308) swaps INSERT↔DELETE and old↔new (patchsets cannot invert); concat (:6814) merges preserving later-wins-per-row semantics via internal hash

## Validation rules found in code

- Iterating a patchset exposes only available fields

## Edge cases found in code

- concat of changesets touching the same row collapses to net effect

## Dependencies

- session-001

## Assumptions / unknowns

- Binary changeset format = the interchange contract (run-1 flag) — format-level characterization advised
- Binary changeset format stability is the migration contract

## Evidence

- `ext/session/sqlite3session.c:3407`
- `ext/session/sqlite3session.c:4308`
- `ext/session/sqlite3session.c:6814`
