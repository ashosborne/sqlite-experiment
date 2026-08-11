# parser-grammar-002 — Keyword fallback (keywords-as-identifiers)

Slice: `parser-grammar` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

%fallback lets many keywords double as identifiers — silent dialect compatibility.

## Entrypoints (citations)

- `src/parse.y %fallback` (other) — `src/parse.y:31`

## Inputs / outputs / observables

- Keywords usable as identifiers in most positions (e.g. a column named 'key', 'abort', 'replace')

## Behaviour (as implemented)

- %fallback directives (src/parse.y:31 region) let ~80 keywords fall back to ID when the grammar allows; %wildcard/%token_class group tokens

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Non-fallback reserved words (e.g. WHERE, SELECT) genuinely reserved — quoting required
- Fallback interacts with DQS: unknown double-quoted identifier may silently become a string (tokenizer card)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Which fallback keywords does downstream SQL rely on?

## Evidence

- `src/parse.y:31`
