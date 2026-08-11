# misc-vtshim-001 — vtshim disposable-module wrapper

Slice: `misc-vtshim` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Wraps a vtab module so it can be safely unregistered while connections exist

## Entrypoints (citations)

- `sqlite3_vtshim_init()` (other) — `ext/misc/vtshim.c:546`

## Inputs / outputs / observables

- Modules registered through vtshim can be unregistered safely while connections still hold vtab instances

## Behaviour (as implemented)

- init ext/misc/vtshim.c:546: interposes a shim module that tracks all vtab/cursor instances and no-ops them after the real module is disposed (for app-controlled module lifecycles)

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Double-dispose guarded; per-db registration

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/vtshim.c:546`
