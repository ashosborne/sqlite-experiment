# misc-zipfile-sqlar-001 — zipfile vtab + sqlar functions (one optional pack)

Slice: `misc-zipfile-sqlar` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Read/write zip archives as tables; sqlar_compress/uncompress build the SQLite-archive format on it (genuinely one pack, kept clustered)

## Entrypoints (citations)

- `sqlite3_zipfile_init()` (other) — `ext/misc/zipfile.c:2294`, `ext/misc/sqlar.c:109`

## Inputs / outputs / observables

- zipfile('a.zip') TVF rows (name,mode,mtime,sz,rawdata,data,method); INSERT/UPDATE/DELETE into zipfile vtab writes archives; sqlar_compress/sqlar_uncompress

## Behaviour (as implemented)

- zipfile vtab (init ext/misc/zipfile.c:2294) reads/writes zip central directory incl. deflate via zlib; sqlar functions (ext/misc/sqlar.c:109) compress/uncompress per SQLite-archive conventions (only stores if smaller)

## Validation rules found in code

- Unsupported compression methods error on read of data (rawdata still available)

## Edge cases found in code

- zip64 not fully supported (size caps); UPDATE of an entry rewrites the archive

## Dependencies

- vtab-core
- loadext-api

## Assumptions / unknowns

- zlib linkage gate stands
- zlib dependency in target build

## Evidence

- `ext/misc/zipfile.c:2294`
- `ext/misc/sqlar.c:109`
