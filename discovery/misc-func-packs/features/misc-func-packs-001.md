# misc-func-packs-001 — Loadable SQL-function extension pack (~18)

Slice: `misc-func-packs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

base64/base85, sha1/sha3, decimal, ieee754, percentile, totype, uint collation, regexp, spellfix, nextchar, rot13, uuid, fossildelta, compress, urifuncs.

## Entrypoints (citations)

- `ext/misc function packs` (other) — `ext/misc/decimal.c:922`, `ext/misc/regexp.c:901`, `ext/misc/spellfix.c:3085`, `ext/misc/uuid.c:213`

## Inputs / outputs / observables

- Per-extension SQL function results (see 16 thin cards)

## Behaviour (as implemented)

- Cluster card fully refined by 16 thin run-2 slices (misc-basexx, misc-sha1, misc-shathree, misc-decimal, misc-ieee754, misc-percentile, misc-totype, misc-uint, misc-regexp, misc-spellfix, misc-nextchar, misc-rot13, misc-uuid, misc-fossildelta, misc-compress, misc-urifuncs) — each carries its own Phase B card
- Common shape: loadable extension registering scalar/aggregate functions or collations via sqlite3_create_function/collation in sqlite3_X_init

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- loadext-api

## Assumptions / unknowns

- Umbrella card delegates; bind decisions per deployed extension (audit note stands)
- Which are compiled into the downstream build vs loaded at runtime?

## Evidence

- `ext/misc/decimal.c:922`
- `ext/misc/regexp.c:901`
- `ext/misc/spellfix.c:3085`
- `ext/misc/uuid.c:213`
