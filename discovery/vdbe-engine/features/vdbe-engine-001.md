# vdbe-engine-001 — Opcode interpreter loop

Slice: `vdbe-engine` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:27:49Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

199 OP_ cases; interruption, progress-callback and error unwinding semantics.

## Entrypoints (citations)

- `sqlite3VdbeExec()` (other) — `src/vdbe.c:902`, `src/vdbe.c:1349`, `src/vdbe.c:3031`

## Inputs / outputs / observables

- Statement execution effects; sqlite3_stmt_status counters (fullscan_step, sort, autoindex, vm_step); progress handler callbacks every N opcodes; EXPLAIN listing of 199 opcode programs

## Behaviour (as implemented)

- sqlite3VdbeExec (src/vdbe.c:902) is the dispatch loop over 199 OP_ cases (census — per charter stays one card); error unwinding sets p->rc and jumps to abort handling; interrupt checked at jump opcodes; OP_Halt (src/vdbe.c:1349) encodes constraint-conflict semantics; OP_Column (src/vdbe.c:3031) decodes record format lazily with header caching

## Validation rules found in code

- Opcode stream validated by asserts in debug builds; register typing invariants

## Edge cases found in code

- Progress-handler abort → SQLITE_INTERRUPT; statement subprograms (triggers) share the same loop via OP_Program frames

## Dependencies

- btree
- pager

## Assumptions / unknowns

- Opcode-level parity is NOT the migration contract — SQL-level characterization recommended (run-1 SME brief)

## Evidence

- `src/vdbe.c:902`
- `src/vdbe.c:1349`
- `src/vdbe.c:3031`
