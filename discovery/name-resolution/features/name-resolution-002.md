# name-resolution-002 — ORDER BY / GROUP BY alias and ordinal resolution

Slice: `name-resolution` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Result-alias and 1-based ordinal handling with compat quirks.

## Entrypoints (citations)

- `sqlite3ResolveOrderGroupBy()` (other) — `src/resolve.c:1748`

## Inputs / outputs / observables

- ORDER BY 2 (ordinal), ORDER BY alias, GROUP BY alias resolution; errors for out-of-range ordinals

## Behaviour (as implemented)

- sqlite3ResolveOrderGroupBy (src/resolve.c:1748): ORDER BY terms resolve against result-set aliases first, then FROM columns; integer literals are 1-based ordinals; GROUP BY supports the same with compat quirks

## Validation rules found in code

- Ordinal out of range → 'ORDER BY term out of range' style error
- Compound SELECT ORDER BY must match result columns of the leftmost SELECT

## Edge cases found in code

- Alias shadowing a real column: alias wins in ORDER BY (documented compat behaviour, differs from standard SQL)

## Dependencies

- name-resolution-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/resolve.c:1748`
