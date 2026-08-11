# vacuum-002 — VACUUM INTO target file

Slice: `vacuum` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Rebuild into named URI target; source unchanged.

## Entrypoints (citations)

- `sqlite3Vacuum(INTO)` (other) — `src/vacuum.c:147`, `src/vacuum.c:245`, `src/vacuum.c:250`

## Inputs / outputs / observables

- New file at target path containing the rebuilt db; source untouched; error if target exists

## Behaviour (as implemented)

- VACUUM INTO (src/vacuum.c:147,245,250) runs the same rebuild writing to the named URI target; honors journal-mode selection for the target; works on read-only sources

## Validation rules found in code

- Target must not already exist (SQLITE_ERROR)

## Edge cases found in code

- INTO target inherits page size of the source's pending settings

## Dependencies

- vacuum-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vacuum.c:147`
- `src/vacuum.c:245`
- `src/vacuum.c:250`
