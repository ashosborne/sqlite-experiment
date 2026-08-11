# expert-001 — Index recommendation lifecycle

Slice: `expert` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Collect SQL workload, analyze against hypothetical indexes, report candidates + EQP.

## Entrypoints (citations)

- `sqlite3_expert_new()` (api) — `ext/expert/sqlite3expert.c:1988`, `ext/expert/sqlite3expert.c:2093`, `ext/expert/sqlite3expert.c:2142`, `ext/expert/sqlite3expert.c:2196`

## Inputs / outputs / observables

- expert_report outputs per statement: proposed CREATE INDEX statements + EQP before/after; candidates deduped across workload

## Behaviour (as implemented)

- expert_new (ext/expert/sqlite3expert.c:1988) shadows the schema in-memory; expert_sql (:2093) collects workload; expert_analyze (:2142) enumerates candidate indexes from WHERE/ORDER BY terms, plans each statement against hypothetical indexes (stat-faked), keeps winners; report (:2196) per REPORT_SQL/INDEXES/PLAN

## Validation rules found in code

- Statements with vtabs/complex features may be skipped with per-statement errors

## Edge cases found in code

- Recommendations are advisory: no cost guarantee on real data (sampling option exists)

## Dependencies

- where-optimizer

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/expert/sqlite3expert.c:1988`
- `ext/expert/sqlite3expert.c:2093`
- `ext/expert/sqlite3expert.c:2142`
- `ext/expert/sqlite3expert.c:2196`
