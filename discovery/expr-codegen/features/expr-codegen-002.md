# expr-codegen-002 — Boolean jump codegen (3-valued logic)

Slice: `expr-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

IfTrue/IfFalse with jumpIfNull variants implement SQL NULL semantics in control flow.

## Entrypoints (citations)

- `sqlite3ExprIfTrue()` (other) — `src/expr.c:6147`, `src/expr.c:6313`, `src/expr.c:6509`

## Inputs / outputs / observables

- WHERE clause truthiness: expressions evaluate to true/false/NULL; NULL rows filtered; IS vs = on NULLs

## Behaviour (as implemented)

- sqlite3ExprIfTrue/IfFalse (src/expr.c:6147,6313) emit conditional jumps with explicit jumpIfNull variants implementing 3-valued logic (AND/OR short-circuit with NULL tracking); IS/IS NOT treat NULLs as comparable

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- x IN (empty) is false not NULL; x IN (set with NULL) can yield NULL — classic parity trap (run-1 flag)
- BETWEEN expands to two comparisons sharing the operand evaluation

## Dependencies

- expr-codegen-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/expr.c:6147`
- `src/expr.c:6313`
- `src/expr.c:6509`
