# global-init-config-002 — Global configuration op matrix

Slice: `global-init-config` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_config with ~30 ops; only LOG/PCACHE_HDRSZ legal after init; defaults in sqlite3Config.

## Entrypoints (citations)

- `sqlite3_config()` (api) — `src/main.c:443`, `src/main.c:453`, `src/global.c:238`, `src/sqlite.h.in:1729`

## Inputs / outputs / observables

- Config op effects: SINGLETHREAD/MULTITHREAD/SERIALIZED, MALLOC/GETMALLOC, PAGECACHE, LOOKASIDE defaults, LOG callback, URI default, MMAP_SIZE bounds, MEMSTATUS toggle...

## Behaviour (as implemented)

- sqlite3_config (src/main.c:443) varargs-dispatches ~30 ops writing sqlite3Config defaults (struct src/global.c:238); only LOG and PCACHE_HDRSZ legal post-init (mask src/main.c:453)

## Validation rules found in code

- Unknown op → SQLITE_ERROR; type-unsafe varargs per op (caller contract)

## Edge cases found in code

- CONFIG_LOG callback runs with internal mutexes held on some paths — re-entrancy restrictions

## Dependencies

- global-init-config-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:443`
- `src/main.c:453`
- `src/global.c:238`
- `src/sqlite.h.in:1729`
