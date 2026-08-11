# analyze-stats-002 — Statistics load into query planner

Slice: `analyze-stats` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

AnalysisLoad parses stat rows into Index cost fields at schema load.

## Entrypoints (citations)

- `sqlite3AnalysisLoad()` (other) — `src/analyze.c:1942`

## Inputs / outputs / observables

- Query-plan changes after ANALYZE; sqlite_stat1 manual edits honoured on next schema load

## Behaviour (as implemented)

- sqlite3AnalysisLoad (src/analyze.c:1942) parses stat rows into Index.aiRowLogEst at schema init; missing stats → default estimates (1M rows heuristics)

## Validation rules found in code

- Malformed stat text ignored (falls back to defaults)

## Edge cases found in code

- Stats loaded per-schema; ATTACH loads that db's stats independently

## Dependencies

- where-optimizer

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/analyze.c:1942`
