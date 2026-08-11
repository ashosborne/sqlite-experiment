# loadext-api-002 — Auto-extension registry

Slice: `loadext-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Process-global list of init functions invoked for each new connection; cancel/reset APIs.

## Entrypoints (citations)

- `sqlite3_auto_extension()` (api) — `src/loadext.c:811`, `src/loadext.c:861`, `src/loadext.c:889`, `src/loadext.c:911`

## Inputs / outputs / observables

- Auto-extension functions invoked for every subsequently-opened connection; cancel/reset effects

## Behaviour (as implemented)

- sqlite3_auto_extension (src/loadext.c:811) appends to a process-global list under the master mutex; sqlite3AutoLoadExtensions (src/loadext.c:911) runs the list at openDatabase time
- cancel_auto_extension (src/loadext.c:861) removes one entry; reset_auto_extension (src/loadext.c:889) clears all

## Validation rules found in code

- Duplicate registration is a no-op (list de-dupes by pointer)

## Edge cases found in code

- Auto-extensions run before URI/vfs decisions complete — failures abort the open

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/loadext.c:811`
- `src/loadext.c:861`
- `src/loadext.c:889`
- `src/loadext.c:911`
