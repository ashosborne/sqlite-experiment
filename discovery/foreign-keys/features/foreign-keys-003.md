# foreign-keys-003 — FK bookkeeping on DROP TABLE

Slice: `foreign-keys` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

DROP TABLE triggers FK violation scans / deferred handling.

## Entrypoints (citations)

- `sqlite3FkDropTable()` (other) — `src/fkey.c:736`

## Inputs / outputs / observables

- DROP TABLE on a parent: either immediate FK errors or deferred-style checks; child table drops clean up FK registrations

## Behaviour (as implemented)

- sqlite3FkDropTable (src/fkey.c:736) wraps DROP TABLE in deferred-FK semantics: violations checked at statement end as if deferred, so self-referential tables drop cleanly

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Dropping a parent with RESTRICT children errors even with foreign_keys off? No — entire machinery gated by the pragma (documented)

## Dependencies

- ddl-schema

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/fkey.c:736`
