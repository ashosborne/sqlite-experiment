# misc-sha1-001 — sha1() hash functions

Slice: `misc-sha1` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sha1(X) and sha1_query(SQL) digests

## Entrypoints (citations)

- `sqlite3_sha_init()` (other) — `ext/misc/sha1.c:418`

## Inputs / outputs / observables

- sha1(X) hex text; sha1_query(SQL) digest over all result rows of SQL

## Behaviour (as implemented)

- init ext/misc/sha1.c:418; sha1_query runs the SQL internally hashing typed row images

## Validation rules found in code

- sha1_query is DIRECTONLY-ish risk (executes SQL text)

## Edge cases found in code

- NULL → NULL; blob vs text input hashed as raw bytes

## Dependencies

- loadext-api

## Assumptions / unknowns

- Legacy-hash retire question stands
- Legacy hash — retire vs keep

## Evidence

- `ext/misc/sha1.c:418`
