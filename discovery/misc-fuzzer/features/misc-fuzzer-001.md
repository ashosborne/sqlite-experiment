# misc-fuzzer-001 — fuzzer vtab (string mutation search)

Slice: `misc-fuzzer` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Generates cost-ranked string mutations from a rules table (typo-correction searches)

## Entrypoints (citations)

- `sqlite3_fuzzer_init()` (other) — `ext/misc/fuzzer.c:1184`

## Inputs / outputs / observables

- fuzzer vtab over a rules table; SELECT word,distance,ruleid ... WHERE word MATCH ? emits cost-ordered mutations

## Behaviour (as implemented)

- init ext/misc/fuzzer.c:1184: generates variant strings of the query term by applying weighted rewrite rules, priority-queue ordered by cumulative cost

## Validation rules found in code

- Rule table columns (ruleset,cFrom,cTo,cost) validated

## Edge cases found in code

- Distinct from amatch: generates variants of the INPUT (for probing another table), not matches from a vocabulary

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/fuzzer.c:1184`
