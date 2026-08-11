# tokenizer-001 — Tokenizer and parser driver loop

Slice: `tokenizer` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Character-class token scanner feeding Lemon parser; error token handling.

## Entrypoints (citations)

- `sqlite3RunParser()` (other) — `src/tokenize.c:600`, `src/tokenize.c:197`

## Inputs / outputs / observables

- Token stream fed to parser; error messages 'unrecognized token: X' with error_offset

## Behaviour (as implemented)

- sqlite3RunParser (src/tokenize.c:600) loops getToken (src/tokenize.c:197) character-class scanner; keywords resolved via generated hash (source: tool/mkkeywordhash.c — out-of-scope path, cited as input only); handles quoting styles: 'string', "identifier-or-string" fallback, [bracket], `backtick`

## Validation rules found in code

- Unterminated strings/comments → error at end of input
- Blob literals X'hex' validated for even hex digits

## Edge cases found in code

- Double-quoted string fallback (DQS): "foo" treated as string literal when no matching identifier — legacy dialect gate (db_config DQS_DML/DDL)
- Numeric literals: hex 0x, underscores in digits (recent dialect), scientific notation

## Dependencies

- parser-grammar

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/tokenize.c:600`
- `src/tokenize.c:197`
