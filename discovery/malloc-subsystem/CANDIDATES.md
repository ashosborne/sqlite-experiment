# CANDIDATES — malloc-subsystem (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Public malloc API + memory accounting | `sqlite3_malloc/malloc64` `src/malloc.c:316,322`, core `sqlite3Malloc` `:296`, `memory_used` `:197`, `release_memory` `:23`, init `:159` | All allocation funnels here (AGENTS.md invariant) |
| 002 | Lookaside fast-path allocator | lookaside checks `src/malloc.c:330-348` | Per-connection small-alloc optimization with config knobs |

Alternative allocators `mem0/mem1/mem2/mem3/mem5.c` (debug/system/dl variants) noted as compile-time selected — recorded here, not separate slices.
