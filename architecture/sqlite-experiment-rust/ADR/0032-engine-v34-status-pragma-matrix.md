# ADR 0032 — engine v34: status/db_status op matrix + pragma dispatcher/TVF batch

Status: accepted · Date: 2026-08-13 · Operator: Ash Osborne · Pack: v34 (supersedes v33)

## Context

`error-status-api-003` carried the run-38 residual: only MEMORY_USED (global) and
LOOKASIDE/SCHEMA_USED (db) live; every other op was a default-zero (global: MISUSE!)
matrix. `pragma-surface-001/002` sat at ~27 of ~70 pragmas with the registry TVFs
deferred. Everything here is core on the bare amalgamation (src/status.c, src/pragma.c) —
presence confirmed by the run-44 probe before freezing.

## Pinning style (probe first)

Counter magnitudes are machine state (allocator layout, page sizes), so the pins are
**predicates + exact zeros + exact rc codes + exact pragma rows** — the same style the
run-38 status pins used. The probe on the pinned bare build fixed the shapes:

- global ops 0..9 all rc 0; op 10 / −1 → MISUSE 21. `PAGECACHE_USED`, `SCRATCH_*`
  (marked NOT USED in sqlite.h.in) and `PARSER_STACK` are exactly (0,0) on this pin.
  `MALLOC_SIZE`/`PAGECACHE_SIZE` have current==0 with positive highwater;
  `MALLOC_COUNT`/`PAGECACHE_OVERFLOW` move under ordinary work; resetFlag pulls
  highwater down to current; `sqlite3_status` (32-bit) agrees with `status64`.
- db ops 0..12 all rc 0; bad op → ERROR 1. `CACHE_USED(_SHARED)`, `SCHEMA_USED`,
  `STMT_USED`, `CACHE_HIT/MISS/WRITE`, `DEFERRED_FKS` all report **highwater 0** like C.
  `STMT_USED` goes 0 → positive (live stmt) → 0 (finalize). `DEFERRED_FKS` is exactly
  1 inside a deferred-violation transaction, 0 after ROLLBACK.
  `LOOKASIDE_MISS_SIZE/FULL` and `CACHE_SPILL` are zeros on the pinned workload.

## What modern now tracks (real, not fabricated)

- Allocator: outstanding allocation count + high (MALLOC_COUNT), largest allocation
  (MALLOC_SIZE hi) — wired into the existing counting allocator.
- Page images: bytes of database file images held/written per connection
  (PAGECACHE_OVERFLOW/PAGECACHE_SIZE global; CACHE_USED/CACHE_USED_SHARED per db =
  the connection's serialized image footprint, recomputed on demand).
- Statements: STMT_USED = real bytes of live prepared statements (struct + SQL text)
  from the live-handle registry.
- I/O events: CACHE_MISS = real file-image loads, CACHE_WRITE = real file flushes,
  CACHE_HIT = store reads served from memory on a file connection. **Magnitudes are
  not claimed** — C counts pages, modern counts its own real I/O events; the frozen
  predicates (zero before / positive after / hi==0) are the contract.
- DEFERRED_FKS: an on-demand scan of deferred FK constraints for child rows without
  parents inside an open transaction (exact 0/1 pinned).

## Deliberate no-claims

- **Lookaside**: C's default build runs a real lookaside allocator (probe: used 6/50,
  hits 249). Modern has none (malloc-subsystem-002 stays none) — LOOKASIDE_USED/HIT
  keep only the run-38 vacuous predicates; MISS_SIZE/FULL are pinned zeros (true on
  the pin's workload). Fabricating positive lookaside counters would be greenwash.
- **SCRATCH_***: NOT USED in sqlite.h.in — pinned zeros, no tracking invented.
- `pragma_module_list`: C's registry is populated lazily by pragma-vtab use (the probe
  listed exactly the five pragma vtabs the probe itself had touched) — fragile,
  deferred with this note.
- stmt_status / scanstatus: untouched (stretch skipped; the matrix ate the meeting).

## Pragma batch

Dispatcher: `data_version` (own writes don't bump; sibling commits do),
`schema_version` +1 per DDL (delta pinned, not the absolute), `freelist_count`,
`query_only` **enforced** (write → `attempt to write a readonly database`, rc 8),
`ignore_check_constraints` **enforced** (CHECK skipped while on),
`quick_check` now really validates CHECK constraints (`CHECK constraint failed in u`
pinned via a row smuggled in under ignore_check_constraints), `collation_list`
(seq,name; newest-first like C), `table_xinfo` / `index_info` / `index_xinfo` row
shapes, and unknown pragma names silently ignored (get and set forms — the classic trap).

TVFs: `pragma_collation_list` (live registry — a runtime-registered collation appears),
`pragma_table_xinfo`, `pragma_index_info`, `pragma_compile_options` (38-entry v32
fingerprint).

## Consequences

- error-status-api-003: residual shrinks to a named list (lookaside positives,
  CACHE_SPILL/TEMPBUF under pressure, stmt_status/scanstatus) — stays partial.
- pragma-surface-001: breadth ~27 → ~40 of ~70; stays partial.
- pragma-surface-002: registry TVF residual shrinks (module_list still deferred);
  stays partial.
- Composed engine-status34-* / engine-pragma34-* cards pin the exact batches.
