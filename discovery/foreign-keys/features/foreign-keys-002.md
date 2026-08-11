# foreign-keys-002 — Cascading referential actions

Slice: `foreign-keys` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

ON DELETE/UPDATE actions implemented as synthesized triggers.

## Entrypoints (citations)

- `fkActionTrigger()` (other) — `src/fkey.c:1217`, `src/fkey.c:1419`

## Inputs / outputs / observables

- ON DELETE/UPDATE CASCADE/SET NULL/SET DEFAULT/RESTRICT/NO ACTION effects on child rows

## Behaviour (as implemented)

- fkActionTrigger (src/fkey.c:1217) synthesizes an internal trigger program per FK action, cached on the FK; sqlite3FkActions (src/fkey.c:1419) fires them after the parent-row operation; RESTRICT behaves like an immediate constraint even inside deferred mode

## Validation rules found in code

- Cascade depth constrained by trigger-depth limit

## Edge cases found in code

- SET DEFAULT re-resolves the default against the current schema; CASCADE UPDATE of the same column chain is order-dependent through the synthesized trigger sequence (run-1 async-chain flag)

## Dependencies

- triggers

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/fkey.c:1217`
- `src/fkey.c:1419`
