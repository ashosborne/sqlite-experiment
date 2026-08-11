# name-resolution-001 — Column/table name lookup and ambiguity rules

Slice: `name-resolution` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Nested-scope lookup with 'ambiguous column'/'no such column' errors; uses tree walker (src/walker.c:64).

## Entrypoints (citations)

- `sqlite3ResolveExprNames()` (other) — `src/resolve.c:2196`, `src/resolve.c:278`, `src/resolve.c:2295`, `src/walker.c:64`

## Inputs / outputs / observables

- Errors: 'no such column', 'ambiguous column name', 'no such table'; qualified name resolution db.table.column

## Behaviour (as implemented)

- sqlite3ResolveExprNames/SelectNames (src/resolve.c:2196,2295) walk expressions (walker src/walker.c:64) resolving names innermost-scope-first through FROM sources, then outer query scopes (correlated subqueries), USING/NATURAL join column merging
- lookupName (src/resolve.c:278) implements the search order incl. special names rowid/oid/_rowid_ and excluded./old./new. contexts

## Validation rules found in code

- Ambiguity across FROM sources at the same level → error; shadowing across levels is legal

## Edge cases found in code

- rowid resolves only if no user column claims the name; WITHOUT ROWID tables have no rowid → error
- Schema-qualified temp shadowing: temp.X wins unqualified lookups

## Dependencies

- select-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/resolve.c:2196`
- `src/resolve.c:278`
- `src/resolve.c:2295`
- `src/walker.c:64`
