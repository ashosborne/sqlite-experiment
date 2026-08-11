# misc-qpvtab-001 — query-plan introspection vtab

Slice: `misc-qpvtab` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Eponymous vtab echoing xBestIndex constraints/orderby — debugging aid for vtab authors

## Entrypoints (citations)

- `sqlite3_qpvtab_init()` (other) — `ext/misc/qpvtab.c:451`

## Inputs / outputs / observables

- SELECT ... FROM qpvtab WHERE flags/vn/... — rows echo the xBestIndex inputs (constraints, orderby, colUsed)

## Behaviour (as implemented)

- Debug vtab (init ext/misc/qpvtab.c:451) exposing query-planner interaction for vtab authors

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Output shape tracks xBestIndex API evolution — dev tool, not stable contract

## Dependencies

- vtab-core

## Assumptions / unknowns

- Dev-tooling; defer recommendation stands

## Evidence

- `ext/misc/qpvtab.c:451`
