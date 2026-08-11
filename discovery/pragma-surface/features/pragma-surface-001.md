# pragma-surface-001 — PRAGMA dispatcher (~70 pragmas)

Slice: `pragma-surface` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Name lookup via generated table (source: tool/mkpragmatab.tcl, out-of-scope path); per-pragma flags control schema-load and result shape.

## Entrypoints (citations)

- `sqlite3Pragma()` (other) — `src/pragma.c:425`, `src/pragma.c:311`

## Inputs / outputs / observables

- Per-pragma result rows or silent set; unknown pragma silently ignored (no error) unless PRAGMA function form

## Behaviour (as implemented)

- sqlite3Pragma (src/pragma.c:425) resolves the name via binary search (pragmaLocate src/pragma.c:311) over the generated PragmaName table; per-entry flags decide schema-load requirement, result columns, and whether a schema qualifier applies
- Families: journal/cache (journal_mode, cache_size, synchronous, wal_checkpoint...), schema introspection (table_info/xinfo, index_list, foreign_key_list, database_list...), behaviour toggles (foreign_keys, recursive_triggers, case_sensitive_like, defer_foreign_keys...), maintenance (integrity_check, quick_check, optimize, incremental_vacuum)

## Validation rules found in code

- Value parsing per pragma type (boolean forms: 1/0/on/off/true/false/yes/no)
- Some pragmas are no-ops without their compile gate

## Edge cases found in code

- Unknown pragma name is silently ignored (classic trap: typos do nothing)
- Some pragmas return their old value when setting, others return nothing

## Dependencies

- (none found in code)

## Assumptions / unknowns

- ~70-pragma inventory stays a census on this card; per-family clustering at bind is a human decision (run-1 note)
- Pragma inventory needs sub-clustering at bind (journal/cache/schema/debug families)

## Evidence

- `src/pragma.c:425`
- `src/pragma.c:311`
