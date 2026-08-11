# misc-regexp-001 — regexp() pattern matching

Slice: `misc-regexp` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

NFA regex engine backing the REGEXP operator; own dialect (not PCRE)

## Entrypoints (citations)

- `sqlite3_regexp_init()` (other) — `ext/misc/regexp.c:901`

## Inputs / outputs / observables

- X REGEXP Y / regexp(Y,X) boolean; regexpi(Y,X) case-insensitive; supported syntax: ^$.*+?{m,n}[...] \b\w\s etc., no backrefs

## Behaviour (as implemented)

- init ext/misc/regexp.c:901: NFA (Thompson-style, no backtracking) — linear-time guarantee; anchors optimize via prefix scan

## Validation rules found in code

- Pattern compile errors → SQL error with position

## Edge cases found in code

- Own dialect — NOT PCRE (no backreferences, no lookaround); case-insensitivity ASCII+limited unicode

## Dependencies

- loadext-api

## Assumptions / unknowns

- Dialect-parity flag stands
- Dialect parity if downstream assumes PCRE

## Evidence

- `ext/misc/regexp.c:901`
