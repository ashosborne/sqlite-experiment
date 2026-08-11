# pcache-001 — Pluggable pcache2 interface boundary

Slice: `pcache` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:29:07Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

sqlite3_pcache_methods2 plugin seam installed via sqlite3_config.

## Entrypoints (citations)

- `sqlite3PcacheInitialize()` (other) — `src/pcache.c:299`

## Inputs / outputs / observables

- sqlite3_config(SQLITE_CONFIG_PCACHE2) replacement; PCACHE_OVERFLOW/PAGECACHE status counters

## Behaviour (as implemented)

- sqlite3PcacheInitialize (src/pcache.c:299) installs default methods if none configured; sqlite3_pcache_methods2 defines xCreate/xFetch/xUnpin/xRekey/xTruncate contract between pager and cache plugin

## Validation rules found in code

- Fetch createFlag semantics (0=only-if-present, 1=allocate-if-easy, 2=must-allocate)

## Edge cases found in code

- Purgeable vs non-purgeable caches (temp dbs); spill callback when cache is full mid-transaction

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Defer recommendation stands unless custom pcache exists downstream (run-1)
- Does downstream install a custom pcache?

## Evidence

- `src/pcache.c:299`
