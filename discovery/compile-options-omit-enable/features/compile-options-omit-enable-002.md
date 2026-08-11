# compile-options-omit-enable-002 — OMIT-gate census (surfaces removed per build)

Slice: `compile-options-omit-enable` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

77 SQLITE_OMIT_* guard references in sqliteInt.h remove surfaces (auth, vtab, wal, window, ...) run 1 carded as default-present.

## Entrypoints (citations)

- `SQLITE_OMIT_* guards` (other) — `src/sqliteInt.h`

## Inputs / outputs / observables

- Presence/absence of whole API+SQL surfaces per OMIT flag (e.g. OMIT_ATTACH removes ATTACH; OMIT_VIRTUALTABLE removes vtab APIs; OMIT_WAL removes WAL modes)

## Behaviour (as implemented)

- CENSUS CARD (per charter — no per-flag explosion): 77 SQLITE_OMIT_* guard references in src/sqliteInt.h gate feature compilation; OMIT flags are only supported when building from canonical sources (not the pre-built amalgamation — upstream caveat); the default baseline enables none

## Validation rules found in code

- Cross-flag dependencies exist (some OMIT combos are invalid) — documented upstream, not re-derived here

## Edge cases found in code

- OMIT flags change the SQL dialect silently (parser rules compiled out)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Confidence downgraded to inferred (run 4 hygiene): file-only evidence, no symbol:line cite

- Per-flag surface diffs become cards only after a non-default baseline is actually proposed (BASELINE.md)

## Evidence

- `src/sqliteInt.h`
