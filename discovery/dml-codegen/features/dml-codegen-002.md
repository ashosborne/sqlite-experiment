# dml-codegen-002 — Constraint checks + ON CONFLICT resolution matrix

Slice: `dml-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Generated checks for NOT NULL/CHECK/UNIQUE/PK with 5 conflict-resolution modes.

## Entrypoints (citations)

- `sqlite3GenerateConstraintChecks()` (other) — `src/insert.c:1901`

## Inputs / outputs / observables

- Constraint failure error codes/messages per ON CONFLICT mode; partial completion under FAIL; REPLACE deleting conflicting rows

## Behaviour (as implemented)

- sqlite3GenerateConstraintChecks (src/insert.c:1901) emits NOT NULL (with per-column override modes), CHECK, UNIQUE/PK probes in deterministic order; conflict resolution per mode: ROLLBACK aborts txn, ABORT (default) undoes statement, FAIL stops mid-statement keeping prior rows, IGNORE skips row, REPLACE deletes conflicting rows then proceeds

## Validation rules found in code

- CHECK constraint NULL result counts as pass (3-valued)
- NOT NULL with ON CONFLICT REPLACE substitutes the default if any

## Edge cases found in code

- REPLACE fires delete triggers only when recursive_triggers is on (documented quirk; run-1 flag)
- Multiple UNIQUE violations resolve in index-iteration order — observable via which row REPLACE removes

## Dependencies

- dml-codegen-001

## Assumptions / unknowns

- REPLACE-mode side effects (delete-then-insert) on triggers/FKs — parity trap

## Evidence

- `src/insert.c:1901`
