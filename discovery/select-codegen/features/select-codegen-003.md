# select-codegen-003 — Subquery flattening rewrite

Slice: `select-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Flattening under a long restriction list; changes plans invisibly — parity-relevant for corner cases.

## Entrypoints (citations)

- `flattenSubquery()` (other) — `src/select.c:4334`

## Inputs / outputs / observables

- Identical results with different plans when flattening applies; plan differences visible via EQP

## Behaviour (as implemented)

- flattenSubquery (src/select.c:4334) merges a FROM-clause subquery into the parent under a documented restriction list (~20 rules: no aggregate+LIMIT mixes, LEFT JOIN constraints, no DISTINCT conflicts...); when blocked, co-routine or materialization chosen instead

## Validation rules found in code

- Restrictions conservative — behaviour must be identical by construction

## Edge cases found in code

- LIMIT in inner+outer interacts (restriction rules); window functions block flattening entirely

## Dependencies

- select-codegen-001

## Assumptions / unknowns

- Run-1 note stands: corner-case parity risk if a migration target replans differently

## Evidence

- `src/select.c:4334`
