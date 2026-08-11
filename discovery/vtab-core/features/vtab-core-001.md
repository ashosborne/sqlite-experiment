# vtab-core-001 — Module registration and vtab lifecycle

Slice: `vtab-core` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

create_module(+v2 destructor); CREATE VIRTUAL TABLE → xCreate; reconnect → xConnect.

## Entrypoints (citations)

- `sqlite3_create_module()` (api) — `src/vtab.c:108`, `src/vtab.c:123`, `src/vtab.c:774`, `src/vtab.c:699`, `src/sqlite.h.in:8002`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE errors/success; module lookup failures 'no such module'; destructor invocation for _v2

## Behaviour (as implemented)

- sqlite3_create_module(_v2) (src/vtab.c:108,123) registers module name→methods on the connection; CREATE VIRTUAL TABLE → VtabCallCreate (src/vtab.c:774) invoking xCreate; schema reload / reopen → VtabCallConnect (src/vtab.c:699) invoking xConnect; DROP → xDestroy vs disconnect xDisconnect

## Validation rules found in code

- Redefining a module name replaces (with destructor call for _v2)
- Eponymous modules (xCreate==NULL or ==xConnect) usable without CREATE

## Edge cases found in code

- xCreate failure leaves no schema entry; xConnect failure marks table unusable ('vtable constructor failed')

## Dependencies

- ddl-schema

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vtab.c:108`
- `src/vtab.c:123`
- `src/vtab.c:774`
- `src/vtab.c:699`
- `src/sqlite.h.in:8002`
