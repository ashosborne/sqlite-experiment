# btree-002 — Cursor seek/insert/delete operations

Slice: `btree` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Cursor positioning and mutation incl. page balancing; file-format invariants.

## Entrypoints (citations)

- `sqlite3BtreeCursor()` (other) — `src/btree.c:4799`, `src/btree.c:5837`, `src/btree.c:9441`, `src/btree.c:9873`

## Inputs / outputs / observables

- Cursor positioning results (exact/leftmost/rightmost); insert/delete effects on the file (page splits/merges invisible except via dbstat)

## Behaviour (as implemented)

- BtreeCursor (src/btree.c:4799) opens read/write cursors with saved-position restore semantics; TableMoveto (src/btree.c:5837) rowid seek; BtreeInsert (src/btree.c:9441) inserts cells triggering balance-siblings; BtreeDelete (src/btree.c:9873) with balancing and overflow-page cleanup

## Validation rules found in code

- File-format invariants asserted (cell sizes, pointers); corruption → SQLITE_CORRUPT bubbles up

## Edge cases found in code

- Cursors on the same table during writes: position saving/restoring rules; delete-then-next semantics

## Dependencies

- btree-001

## Assumptions / unknowns

- Real contract = on-disk format (fileformat2.html); characterize via integrity_check + dbstat (run-1 note)
- File-format parity (fileformat2.html) is the real contract — characterize via db-file byte comparisons?

## Evidence

- `src/btree.c:4799`
- `src/btree.c:5837`
- `src/btree.c:9441`
- `src/btree.c:9873`
