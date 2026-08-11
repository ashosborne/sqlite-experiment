# expr-codegen-001 — Value expression code generation

Slice: `expr-codegen` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Expr tree to VDBE ops with affinity/collation propagation and constant factoring.

## Entrypoints (citations)

- `sqlite3ExprCodeTarget()` (other) — `src/expr.c:4972`, `src/expr.c:5931`

## Inputs / outputs / observables

- Computed SQL expression values; affinity application on comparisons; collation selection on text comparison

## Behaviour (as implemented)

- sqlite3ExprCodeTarget (src/expr.c:4972) translates Expr trees to VDBE ops with affinity propagation (comparison operands get each other's affinity per the affinity rules) and collation resolution (explicit COLLATE > column collation > BINARY)
- Constant expressions factored into the program prologue

## Validation rules found in code

- Function arity checked at resolve; unknown function → 'no such function'

## Edge cases found in code

- CAST semantics differ from column-affinity coercion (CAST truncates/clamps per target)
- Integer overflow in literals → stored as REAL

## Dependencies

- vdbe-engine

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/expr.c:4972`
- `src/expr.c:5931`
