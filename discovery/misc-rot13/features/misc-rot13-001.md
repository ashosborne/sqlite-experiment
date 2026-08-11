# misc-rot13-001 — rot13() function + collation

Slice: `misc-rot13` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

rot13(X) scalar + rot13 collation (demo-grade obfuscation)

## Entrypoints (citations)

- `sqlite3_rot_init()` (other) — `ext/misc/rot13.c:100`

## Inputs / outputs / observables

- rot13(X) text; COLLATE rot13 ordering

## Behaviour (as implemented)

- init ext/misc/rot13.c:100: ASCII-only rotation; non-alpha passthrough

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Involution: rot13(rot13(x))==x

## Dependencies

- loadext-api

## Assumptions / unknowns

- Demo-only; defer stands (bound by policy)
- Demo-only — recommend defer

## Evidence

- `ext/misc/rot13.c:100`
