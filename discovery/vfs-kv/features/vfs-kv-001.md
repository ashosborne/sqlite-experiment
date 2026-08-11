# vfs-kv-001 — kvvfs virtual filesystem

Slice: `vfs-kv` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

VFS storing db+journal as key/value pairs with base85 page encoding; wasm/localStorage primary consumer.

## Entrypoints (citations)

- `sqlite3KvvfsInit()` (other) — `src/os_kv.c:1094`, `src/os_kv.c:114`, `src/os_kv.c:893`

## Inputs / outputs / observables

- vfs 'kvvfs' usable for main dbs named 'local'/'session' (browser storage classes); pages stored as K/V pairs

## Behaviour (as implemented)

- sqlite3KvvfsInit (src/os_kv.c:1094) registers sqlite3OsKvvfsObject (:114); kvvfsOpen (:893) binds a KV namespace; pages/journal serialized via base85-ish encoding into keys (kvvfs jrnl/pgsz/sz keys)

## Validation rules found in code

- Only one db per namespace; size limits from the backing store

## Edge cases found in code

- Journal stored as a single growing value — commit atomicity relies on the KV store's put atomicity

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/os_kv.c:1094`
- `src/os_kv.c:114`
- `src/os_kv.c:893`
