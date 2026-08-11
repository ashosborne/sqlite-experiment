# auth-callback-api-001 — Authorizer registration and dispatch

Slice: `auth-callback-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-connection callback consulted at compile time per action code; DENY errors the statement, IGNORE downgrades.

## Entrypoints (citations)

- `sqlite3_set_authorizer()` (api) — `src/auth.c:70`, `src/auth.c:238`

## Inputs / outputs / observables

- Authorizer invoked at prepare time per action (code + 4 string args); SQLITE_OK/DENY/IGNORE effects on the compiled statement

## Behaviour (as implemented)

- sqlite3_set_authorizer (src/auth.c:70) sets the per-connection callback; sqlite3AuthCheck (src/auth.c:238) consults it during code generation; DENY → SQLITE_AUTH error ('not authorized'), invalid return → SQLITE_MISUSE error at prepare

## Validation rules found in code

- Setting a new authorizer expires prepared statements (they re-prepare and re-authorize)

## Edge cases found in code

- Authorizer disabled during schema parsing/initialization (init.busy) and for some internal statements

## Dependencies

- prepare-statement-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/auth.c:70`
- `src/auth.c:238`
