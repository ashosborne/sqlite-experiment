# builtin-scalar-agg-funcs-002 — Aggregate function family

Slice: `builtin-scalar-agg-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

count/sum/total/avg/min/max/group_concat with step/finalize/inverse (window-capable) semantics.

## Entrypoints (citations)

- `aBuiltinFunc[] aggregates` (other) — `src/func.c:3408`, `src/func.c:1929`, `src/func.c:3357`, `src/func.c:3360`

## Inputs / outputs / observables

- Aggregate results over grouped rows; NULL treatment (count(*) vs count(X)); sum() error on integer overflow vs total() float result

## Behaviour (as implemented)

- Aggregates registered with step/finalize (+inverse/value for window use, e.g. WAGGREGATE(sum...) src/func.c:3408); sumStep (src/func.c:1929) keeps integer sum until overflow then errors for sum() while total() switches to float
- min/max as aggregates (2-arg scalar variants share the FuncDef rows src/func.c:3357,3360); group_concat with optional separator, avg = sum/count as float

## Validation rules found in code

- sum() over only-NULL rows → NULL; total() → 0.0; count() never NULL

## Edge cases found in code

- sum() integer overflow → 'integer overflow' error (SQLITE_ERROR), not wraparound
- min/max aggregate ignores NULLs; scalar min/max returns NULL on any NULL

## Dependencies

- (none found in code)

## Assumptions / unknowns

- sum vs total overflow/NULL divergence is behaviour-rich

## Evidence

- `src/func.c:3408`
- `src/func.c:1929`
- `src/func.c:3357`
- `src/func.c:3360`
