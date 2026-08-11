# util-primitives-001 — Shared primitives (UTF codecs, PRNG, hash)

Slice: `util-primitives` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

UTF-8/16 read/convert with invalid-sequence policy; ChaCha20-based randomness (public API); string hash tables.

## Entrypoints (citations)

- `sqlite3_randomness()` (api) — `src/random.c:59`, `src/utf.c:175`, `src/utf.c:475`, `src/hash.c:23`

## Inputs / outputs / observables

- sqlite3_randomness output (deterministic after seeding in test mode); UTF conversions in column/bind 16-bit APIs; invalid-UTF handling

## Behaviour (as implemented)

- sqlite3_randomness (src/random.c:59) ChaCha20 PRNG seeded from VFS xRandomness, process-global under mutex; N==0 reseeds
- UTF readers (sqlite3Utf8Read src/utf.c:175, Utf8CharLen src/utf.c:475) decode with over-length/invalid-sequence tolerance (replacement 0xFFFD policy); hash tables (src/hash.c:23) case-insensitive ASCII symbol maps

## Validation rules found in code

- UTF-16 byte-order handled per BOM/native flags in Mem conversions

## Edge cases found in code

- Invalid UTF-8 in LIKE/length() — permissive decoding is a parity trap vs strict engines (run-2 note stands)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Invalid-UTF handling is parity-relevant for text functions

## Evidence

- `src/random.c:59`
- `src/utf.c:175`
- `src/utf.c:475`
- `src/hash.c:23`
