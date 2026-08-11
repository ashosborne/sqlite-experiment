# serialize-memdb-api-001 — Serialize / deserialize byte-image round-trip

Slice: `serialize-memdb-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

serialize copies or references the db image; deserialize swaps a connection onto a memory image with FREEONCLOSE/RESIZEABLE flags.

## Entrypoints (citations)

- `sqlite3_serialize()` (api) — `src/memdb.c:750`, `src/memdb.c:841`, `src/sqlite.h.in:11248`, `src/sqlite.h.in:11326`

## Inputs / outputs / observables

- serialize returns malloc'd (or borrowed with NOCOPY) byte image + size; deserialize swaps the named schema onto the image, return code

## Behaviour (as implemented)

- sqlite3_serialize (src/memdb.c:750): for a memdb-backed schema returns the bytes directly (NOCOPY possible); else runs an internal copy of the database image
- sqlite3_deserialize (src/memdb.c:841): closes the current schema and reopens it as a memdb over the supplied buffer; FREEONCLOSE transfers ownership, RESIZEABLE allows growth

## Validation rules found in code

- deserialize on a connection inside a transaction → SQLITE_BUSY/misuse paths
- READONLY flag makes subsequent writes fail SQLITE_READONLY

## Edge cases found in code

- Corrupt image is accepted at deserialize time and fails later at page access (deferred validation)

## Dependencies

- pager

## Assumptions / unknowns

- Gate: built when SQLITE_ENABLE_DESERIALIZE-era default on (3.36+ default-on in-tree) — baseline fingerprint confirms
- SQLITE_ENABLE_DESERIALIZE gating in target build?

## Evidence

- `src/memdb.c:750`
- `src/memdb.c:841`
- `src/sqlite.h.in:11248`
- `src/sqlite.h.in:11326`
