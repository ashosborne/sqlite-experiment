# compile-options-omit-enable-001 — Compile-option diagnostics API (C + SQL)

Slice: `compile-options-omit-enable` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

compileoption_used/get in C and SQL report the active build matrix; itself gated by SQLITE_OMIT_COMPILEOPTION_DIAGS.

## Entrypoints (citations)

- `sqlite3_compileoption_used()` (api) — `src/main.c:5220`, `src/main.c:5253`, `src/func.c:1051`, `src/func.c:1075`

## Inputs / outputs / observables

- sqlite3_compileoption_used('X') 0/1 (prefix 'SQLITE_' optional); compileoption_get(N) enumerates; SQL twins sqlite_compileoption_used/get

## Behaviour (as implemented)

- C APIs (src/main.c:5220,5253) read the compile-time option array assembled at build; SQL wrappers registered in func.c (src/func.c:1051,1075); the pair is the baseline-pinning oracle (overnight/BASELINE.md)

## Validation rules found in code

- Whole APIs removable via SQLITE_OMIT_COMPILEOPTION_DIAGS (self-referential gate)

## Edge cases found in code

- Option list contains only options known at amalgamation/ctime assembly — hand-added -D flags appear only if recognized

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Which exact option set is the migration baseline? Pin via this API before characterization.

## Evidence

- `src/main.c:5220`
- `src/main.c:5253`
- `src/func.c:1051`
- `src/func.c:1075`
