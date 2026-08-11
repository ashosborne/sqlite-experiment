# misc-compress-001 — compress()/uncompress() zlib functions

Slice: `misc-compress` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Deflate-based blob compression with size-prefixed format

## Entrypoints (citations)

- `sqlite3_compress_init()` (other) — `ext/misc/compress.c:122`

## Inputs / outputs / observables

- compress(X) blob (varint size prefix + deflate); uncompress(Y) original blob; round-trip identity

## Behaviour (as implemented)

- init ext/misc/compress.c:122: zlib deflate with self-describing size prefix (same format sqlar uses)

## Validation rules found in code

- uncompress on non-compressed/corrupt input → error/NULL paths

## Edge cases found in code

- Incompressible input grows slightly (prefix + deflate overhead)

## Dependencies

- loadext-api

## Assumptions / unknowns

- zlib gate stands
- zlib dependency; format shared with sqlar

## Evidence

- `ext/misc/compress.c:122`
