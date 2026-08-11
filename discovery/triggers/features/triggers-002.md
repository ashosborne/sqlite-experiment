# triggers-002 — Row-trigger firing semantics

Slice: `triggers` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Trigger sub-programs compiled per (trigger,orconf); BEFORE/AFTER ordering; WHEN clause; recursive-trigger gate.

## Entrypoints (citations)

- `sqlite3CodeRowTrigger()` (other) — `src/trigger.c:1469`, `src/trigger.c:1232`, `src/trigger.c:870`, `src/trigger.c:1397`

## Inputs / outputs / observables

- Trigger program side effects; RAISE(ABORT/FAIL/ROLLBACK/IGNORE) effects; recursion depth errors

## Behaviour (as implemented)

- sqlite3TriggersExist (src/trigger.c:870) gates codegen; codeRowTrigger (src/trigger.c:1232) compiles each (trigger,orconf) pair once into a subprogram cached on the trigger; sqlite3CodeRowTrigger (src/trigger.c:1469) emits BEFORE then operation then AFTER with WHEN-clause guards; old/new register mapping per operation

## Validation rules found in code

- Recursive triggers only when PRAGMA recursive_triggers=ON (default off; delete-cascade counts) — depth capped by SQLITE_LIMIT_TRIGGER_DEPTH
- RAISE(IGNORE) skips the current row silently incl. the triggering operation in BEFORE

## Edge cases found in code

- BEFORE trigger modifying the same row being updated: the modification is invisible to the outer statement (documented undefined-ish area — flag)
- INSTEAD OF triggers make view DML succeed with 0 direct changes(); changes counted via trigger body

## Dependencies

- vdbe-engine
- dml-codegen

## Assumptions / unknowns

- recursive_triggers pragma default in target build?

## Evidence

- `src/trigger.c:1469`
- `src/trigger.c:1232`
- `src/trigger.c:870`
- `src/trigger.c:1397`
