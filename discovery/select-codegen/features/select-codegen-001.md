# select-codegen-001 — SELECT orchestration and FROM expansion

Slice: `select-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Top-level SELECT compile: FROM expansion, aggregate/distinct handling, result output modes.

## Entrypoints (citations)

- `sqlite3Select()` (other) — `src/select.c:7642`, `src/select.c:5929`

## Inputs / outputs / observables

- Result sets for joins/aggregates/DISTINCT/LIMIT; column naming rules; EXPLAIN QUERY PLAN shape

## Behaviour (as implemented)

- sqlite3Select (src/select.c:7642) orchestrates: FROM expansion (views/CTEs/subqueries via sqlite3ExpandSubquery src/select.c:5929), aggregate analysis, DISTINCT via ephemeral index or covering-index dedup, ORDER BY via sorter unless index-satisfied, LIMIT/OFFSET counters
- Join codegen: nested loops in planner-chosen order; LEFT JOIN null-row completion; USING/NATURAL column coalescing

## Validation rules found in code

- Result column count limits (SQLITE_LIMIT_COLUMN); aggregate misuse errors ('misuse of aggregate')

## Edge cases found in code

- Bare columns in aggregate queries take values from an arbitrary row of the group EXCEPT min/max single-aggregate queries which pin the winning row — famous dialect quirk
- Column names of expressions are unspecified-but-stable ('short names' pragma era) — consumers must alias

## Dependencies

- where-optimizer
- expr-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/select.c:7642`
- `src/select.c:5929`
