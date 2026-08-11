# misc-vtablog-001 — vtablog logging wrapper vtab

Slice: `misc-vtablog` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Logs every vtab method call — debugging aid

## Entrypoints (citations)

- `sqlite3_vtablog_init()` (other) — `ext/misc/vtablog.c:712`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE USING vtablog(schema=..., rows=...); every vtab method call printed to stdout

## Behaviour (as implemented)

- init ext/misc/vtablog.c:712: wrapper-style demo vtab logging xCreate/xConnect/xBestIndex/xFilter/... invocations with args; generates synthetic rows

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- stdout logging — not for production

## Dependencies

- vtab-core

## Assumptions / unknowns

- Dev tooling; defer stands

## Evidence

- `ext/misc/vtablog.c:712`
