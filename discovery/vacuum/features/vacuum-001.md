# vacuum-001 — VACUUM full rebuild

Slice: `vacuum` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Rebuilds db into temp then swaps; applies pending page_size/auto_vacuum changes.

## Entrypoints (citations)

- `sqlite3RunVacuum()` (other) — `src/vacuum.c:105`, `src/vacuum.c:143`

## Inputs / outputs / observables

- File size shrink; page_size/auto_vacuum changes applied; rebuilt-from-scratch btrees (rowids preserved unless INTEGER PRIMARY KEY absent... rowids MAY change without ipk)

## Behaviour (as implemented)

- sqlite3RunVacuum (src/vacuum.c:143) attaches a temp db, copies schema + data via internal SQL, then back-copies pages with sqlite3BtreeCopyFile semantics; pending page_size/journal_mode changes take effect

## Validation rules found in code

- VACUUM inside a transaction → error 'cannot VACUUM from within a transaction'
- VACUUM with active statements → SQLITE_LOCKED paths

## Edge cases found in code

- Implicit rowids may be renumbered by VACUUM (documented); WITHOUT ROWID tables unaffected
- VACUUM <schema> form targets one attached db

## Dependencies

- pager
- btree

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vacuum.c:105`
- `src/vacuum.c:143`
