# triggers-001 — Trigger DDL lifecycle

Slice: `triggers` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

CREATE/DROP TRIGGER incl. temp-trigger scoping and INSTEAD OF-on-view validation.

## Entrypoints (citations)

- `sqlite3BeginTrigger()` (other) — `src/trigger.c:104`, `src/trigger.c:324`, `src/trigger.c:659`

## Inputs / outputs / observables

- sqlite_master rows for triggers; errors: 'cannot create trigger on system table', INSTEAD OF only on views, BEFORE/AFTER only on tables

## Behaviour (as implemented)

- sqlite3BeginTrigger/FinishTrigger (src/trigger.c:104,324) parse-validate and persist trigger DDL; temp triggers on any-db tables live in temp schema; DropTrigger (src/trigger.c:659) removes

## Validation rules found in code

- Trigger on a view must be INSTEAD OF and vice versa
- Column list only valid for UPDATE OF triggers

## Edge cases found in code

- CREATE TEMP TRIGGER on main-db table: dropped with the connection, fires only for this connection's statements

## Dependencies

- ddl-schema

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/trigger.c:104`
- `src/trigger.c:324`
- `src/trigger.c:659`
