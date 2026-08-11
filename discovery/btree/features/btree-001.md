# btree-001 — Btree handle and transaction boundary

Slice: `btree` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Open/close btree on a pager; read/write transaction nesting with schema-version checks.

## Entrypoints (citations)

- `sqlite3BtreeOpen()` (other) — `src/btree.c:2562`, `src/btree.c:3835`

## Inputs / outputs / observables

- Transaction state via sqlite3_txn_state; schema cookie checks; SQLITE_BUSY on lock escalation

## Behaviour (as implemented)

- sqlite3BtreeOpen (src/btree.c:2562) binds a Btree to a pager (shared-cache aware); BtreeBeginTrans (src/btree.c:3835) acquires read/write transactions with schema-version out-param, invoking busy handler on contention

## Validation rules found in code

- Write transaction requires non-readonly pager; nested begin upgrades read→write

## Edge cases found in code

- Shared-cache table-level locks (gated) vs default per-file locking

## Dependencies

- pager

## Assumptions / unknowns

- Storage-scope SME decision from run 1 stands (retained platform vs migrated)

## Evidence

- `src/btree.c:2562`
- `src/btree.c:3835`
