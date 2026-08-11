# parser-grammar-001 — Statement grammar productions

Slice: `parser-grammar` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

417 productions define the accepted SQL dialect; actions call into build/select/etc.

## Entrypoints (citations)

- `src/parse.y cmd productions` (other) — `src/parse.y:181`, `src/parse.y:190`, `src/parse.y:195`, `src/parse.y:207`

## Inputs / outputs / observables

- Accepted vs rejected SQL dialect; parse-error messages 'near "X": syntax error'

## Behaviour (as implemented)

- 417 Lemon productions (src/parse.y) define statement grammar; actions build AST/emit codegen calls directly (cmd productions src/parse.y:181-207); precedence/associativity declarations resolve expression ambiguity

## Validation rules found in code

- Depth/complexity guarded by SQLITE_LIMIT_EXPR_DEPTH and parser stack size

## Edge cases found in code

- Errors recover minimally: first syntax error aborts the statement (no multi-error reporting)

## Dependencies

- tokenizer

## Assumptions / unknowns

- Dialect-parity characterization via accept/reject SQL corpora (run-1 note); generated parse.c never cited

## Evidence

- `src/parse.y:181`
- `src/parse.y:190`
- `src/parse.y:195`
- `src/parse.y:207`
