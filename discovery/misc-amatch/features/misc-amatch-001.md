# misc-amatch-001 — approximate-match vtab

Slice: `misc-amatch` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Edit-distance based approximate matching with user-supplied cost rules

## Entrypoints (citations)

- `sqlite3_amatch_init()` (other) — `ext/misc/amatch.c:1512`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE USING approximate_match(vocabulary_table=..., edit_distances=...); SELECT word,distance WHERE word MATCH ? ORDER BY distance

## Behaviour (as implemented)

- init ext/misc/amatch.c:1512: A*-style search over a sorted vocabulary using a user-supplied edit-cost table (insert/delete/substitute per char pair)

## Validation rules found in code

- Cost-rule table format validated at create

## Edge cases found in code

- Language-specific costs enable phonetic-ish matching; unbounded distance queries can be slow (documented)

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/amatch.c:1512`
