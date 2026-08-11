# expr-codegen-003 — Expression equivalence testing

Slice: `expr-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Structural comparison used for indexed-expression matching and factoring.

## Entrypoints (citations)

- `sqlite3ExprCompare()` (other) — `src/expr.c:6591`

## Inputs / outputs / observables

- Indexed-expression matching (expression indexes used when query text matches structurally); constant factoring reuse

## Behaviour (as implemented)

- sqlite3ExprCompare (src/expr.c:6591) structural equality with iTab remapping; 0=identical, 1=differ only by COLLATE, 2=differ; drives index-on-expression usage and WHERE-term reuse

## Validation rules found in code

- Comparison is syntax-structural: equivalent-but-differently-written expressions do NOT match (e.g. a+0 vs a)

## Edge cases found in code

- Functions must be deterministic to participate in expression-index matching

## Dependencies

- where-optimizer

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/expr.c:6591`
