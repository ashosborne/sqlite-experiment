# misc-shathree-001 — sha3() hash functions

Slice: `misc-shathree` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sha3(X,SIZE) and sha3_query() digests (Keccak)

## Entrypoints (citations)

- `sqlite3_shathree_init()` (other) — `ext/misc/shathree.c:830`

## Inputs / outputs / observables

- sha3(X[,SIZE]) with SIZE in {224,256,384,512} default 256; sha3_query(SQL[,SIZE])

## Behaviour (as implemented)

- init ext/misc/shathree.c:830: Keccak implementation; query variant hashes result-set row images like sha1_query

## Validation rules found in code

- Invalid SIZE → error

## Edge cases found in code

- Returns BLOB (unlike sha1's hex TEXT) — type asymmetry between the two packs

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/shathree.c:830`
