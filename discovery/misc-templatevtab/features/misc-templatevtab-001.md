# misc-templatevtab-001 — template vtab skeleton

Slice: `misc-templatevtab` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Reference implementation for writing new vtabs; not a data-path surface

## Entrypoints (citations)

- `sqlite3_templatevtab_init()` (other) — `ext/misc/templatevtab.c:260`

## Inputs / outputs / observables

- templatevtab module registering and returning no interesting data (skeleton)

## Behaviour (as implemented)

- init ext/misc/templatevtab.c:260: canonical vtab skeleton for authors — every method stubbed with commentary

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- vtab-core

## Assumptions / unknowns

- Documentation-only role; defer recommendation stands (bound by ACCEPT_ALL policy)
- Documentation-only role — recommend defer

## Evidence

- `ext/misc/templatevtab.c:260`
