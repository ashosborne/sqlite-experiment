# misc-spellfix-001 — spellfix1 fuzzy-search vtab + editdist functions

Slice: `misc-spellfix` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Fuzzy word search vtab with configurable edit-distance cost tables and phonetic hashing

## Entrypoints (citations)

- `sqlite3_spellfix_init()` (other) — `ext/misc/spellfix.c:3085`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE USING spellfix1; INSERT words (+rank, langid); SELECT word,rank,distance,score WHERE word MATCH ? [AND top=N]; aux functions editdist3(), spellfix1_phonehash(), spellfix1_translit()

## Behaviour (as implemented)

- init ext/misc/spellfix.c:3085: stores vocabulary with phonetic-hash clustering (K1/K2); MATCH searches by phonetic class then ranks by editdist3 (configurable cost table via editdist3 config table) blended with word rank

## Validation rules found in code

- Cost-table schema validated when configured

## Edge cases found in code

- Unicode transliteration to ASCII before hashing (translit); top=N pushdown limits work

## Dependencies

- vtab-core
- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/spellfix.c:3085`
