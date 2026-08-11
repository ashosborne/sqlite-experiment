# select-codegen-002 — Compound SELECT set operations

Slice: `select-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

UNION/UNION ALL/EXCEPT/INTERSECT incl. merge strategy and LIMIT/ORDER BY rules.

## Entrypoints (citations)

- `multiSelect()` (other) — `src/select.c:2978`, `src/select.c:2905`, `src/select.c:3443`

## Inputs / outputs / observables

- UNION/UNION ALL/INTERSECT/EXCEPT results incl. dedup semantics and column-count errors; ORDER BY/LIMIT apply to the compound result

## Behaviour (as implemented)

- multiSelect (src/select.c:2978): UNION ALL streams; UNION/INTERSECT/EXCEPT use ephemeral indexes for dedup/set-ops; merge strategy (src/select.c:3443) when both sides are sorted; VALUES fast path (src/select.c:2905)

## Validation rules found in code

- 'SELECTs to the left and right of X do not have the same number of result columns' error
- ORDER BY terms must resolve against the leftmost SELECT

## Edge cases found in code

- Collation for dedup comes from the leftmost SELECT's columns
- Compound depth capped by SQLITE_LIMIT_COMPOUND_SELECT

## Dependencies

- select-codegen-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/select.c:2978`
- `src/select.c:2905`
- `src/select.c:3443`
