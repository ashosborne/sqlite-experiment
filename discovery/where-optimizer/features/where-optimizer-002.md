# where-optimizer-002 — Access-path enumeration and cost-based solver

Slice: `where-optimizer` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Candidate WhereLoops per table (full scan / index / covering); wherePathSolver picks cheapest join order.

## Entrypoints (citations)

- `wherePathSolver()` (other) — `src/where.c:5834`, `src/where.c:4003`, `src/where.c:3219`

## Inputs / outputs / observables

- Chosen join order/index per query given stats; EQP text; query time as stats change

## Behaviour (as implemented)

- whereLoopAddBtree(+Index) (src/where.c:4003,3219) enumerate candidate loops with LogEst costs from stats (analyze-stats) or defaults; wherePathSolver (src/where.c:5834) beam-searches join orders minimizing estimated cost with an interstage heuristic (src/where.c:6262 region)

## Validation rules found in code

- Solver deterministic given identical stats/schema (stable plans)

## Edge cases found in code

- Plan flips at stat boundaries are the classic performance cliff — characterize at result level (run-1 SME note), pin stats in fixtures

## Dependencies

- analyze-stats

## Assumptions / unknowns

- Plan parity vs result parity: agree characterization level (EQP text is version-fragile)

## Evidence

- `src/where.c:5834`
- `src/where.c:4003`
- `src/where.c:3219`
