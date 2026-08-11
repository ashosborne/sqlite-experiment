# foreign-keys-001 — Immediate vs deferred FK checking

Slice: `foreign-keys` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-statement vs per-transaction constraint counters; foreign_keys pragma gate.

## Entrypoints (citations)

- `sqlite3FkCheck()` (other) — `src/fkey.c:889`, `src/fkey.c:1145`

## Inputs / outputs / observables

- SQLITE_CONSTRAINT_FOREIGNKEY at statement end (immediate) or COMMIT (deferred); PRAGMA foreign_keys gate; foreign_key_check/foreign_key_list pragmas

## Behaviour (as implemented)

- sqlite3FkCheck (src/fkey.c:889) emits lookup probes on parent/child per DML; counters increment on violation and decrement on cure; deferred constraints check the counter at COMMIT, immediate at statement end; sqlite3FkRequired (src/fkey.c:1145) decides if a given UPDATE touches FK columns

## Validation rules found in code

- foreign_keys defaults OFF (run-1 flag: downstream must opt in)
- Parent must have a UNIQUE/PK matching the referenced columns else 'foreign key mismatch' at prepare

## Edge cases found in code

- NULL child values always satisfy (MATCH SIMPLE semantics only)
- Deferred+immediate mix: defer_foreign_keys pragma flips per-transaction

## Dependencies

- dml-codegen

## Assumptions / unknowns

- foreign_keys pragma default off — is it enabled downstream?

## Evidence

- `src/fkey.c:889`
- `src/fkey.c:1145`
