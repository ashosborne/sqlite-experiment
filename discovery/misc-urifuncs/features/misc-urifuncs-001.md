# misc-urifuncs-001 — URI-parameter SQL functions

Slice: `misc-urifuncs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

SQL access to URI open-parameters of the current database file

## Entrypoints (citations)

- `sqlite3_urifuncs_init()` (other) — `ext/misc/urifuncs.c:180`

## Inputs / outputs / observables

- sqlite_uri_parameter(F,P), sqlite_uri_boolean(F,P,D), sqlite_uri_int64(F,P,D), sqlite_uri_key(F,N) over the URI used to open schema F

## Behaviour (as implemented)

- init ext/misc/urifuncs.c:180: wraps sqlite3_uri_* C APIs exposing open-URI parameters to SQL

## Validation rules found in code

- Non-URI-opened databases → NULLs

## Edge cases found in code

- Only sees parameters of the ORIGINAL open (not attach-time changes elsewhere)

## Dependencies

- loadext-api
- connection-lifecycle-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/urifuncs.c:180`
