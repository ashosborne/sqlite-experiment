# prepare-statement-api-003 — Parameter binding (typed)

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

bind_blob/double/int/int64/null/text/value/zeroblob + clear_bindings.

## Entrypoints (citations)

- `sqlite3_bind_text()` (api) — `src/vdbeapi.c:1892`, `src/vdbeapi.c:1816`, `src/vdbeapi.c:155`

## Inputs / outputs / observables

- Return codes SQLITE_OK/SQLITE_RANGE/SQLITE_MISUSE/SQLITE_TOOBIG; queryability via sqlite3_bind_parameter_count/name

## Behaviour (as implemented)

- Typed binders (src/vdbeapi.c:1816-1961) set parameter i (1-based) on a reset statement; text/blob take destructor (SQLITE_STATIC/TRANSIENT) controlling copy semantics
- clear_bindings (src/vdbeapi.c:155) sets all parameters back to NULL; reset() alone preserves bindings
- bind_value deep-copies an sqlite3_value; bind_zeroblob allocates a zero-filled blob of length n lazily

## Validation rules found in code

- Index out of range → SQLITE_RANGE
- Binding while the statement is busy (mid-step) → SQLITE_MISUSE
- text/blob longer than SQLITE_LIMIT_LENGTH → SQLITE_TOOBIG

## Edge cases found in code

- Unbound parameters evaluate as NULL
- bind_text with negative length reads to NUL; embedded NULs preserved only with explicit length

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeapi.c:1892`
- `src/vdbeapi.c:1816`
- `src/vdbeapi.c:155`
