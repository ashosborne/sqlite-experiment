# ADR 0018 — engine v20: create_collation registration + registry-driven COLLATE

Status: accepted (pack v20 BOUND, Ash Osborne, delegated autonomy run 30)

## Context

Built-in and hardcoded collations (BINARY/NOCASE plus closed rot13/uint/decimal
branches in eval) already served pinned scripts, but the application *registration*
surface — the sibling of pack v18's `create_function` — did not exist. COLLATE names
outside the closed list silently fell back to BINARY instead of erroring like C.

## Decision

1. **Per-connection registry, v18 shape.** `COLL_REG` (thread-local, keyed by db)
   stores name→{pArg, xCompare, xDestroy}. `sqlite3_create_collation` delegates to
   `_v2`. NULL xCompare deletes (old xDestroy fires); overwrite replaces (old xDestroy
   fires); close fires all remaining xDestroy — matching the pinned counter (0/1/2).
2. **eTextRep matrix as pinned:** 1 (UTF8), 2/3/4 (UTF16*) and 8 accepted; 0 and 99 →
   `SQLITE_MISUSE` (21). All registered callbacks receive UTF-8 bytes — the engine's
   internal encoding — regardless of declared rep (honest simplification; the frozen
   pins only exercise UTF-8 payloads).
3. **Eval resolves through the registry.** Equality, range and ORDER BY collation
   resolution goes builtins-first (`binary`, `nocase`, `rtrim` + pinned `rot13`,
   `uint`, `decimal` kept as built-ins — NOT re-homed, goldens unchanged), then the
   registry with a real xCompare invocation. Unknown → `no such collation sequence: X`
   at prepare (the dry-run probe surfaces it exactly where C's compile step does).
4. **Declared column collations are honest state.** `Col.coll` is parsed from
   CREATE TABLE (`COLLATE <name>`), persisted via `create_sql` (survives file reopen),
   and snapshotted into `Ctx.col_colls` so bare-column comparisons and ORDER BY pick
   up the declared sequence. Pinned residual: resolution is by column name (unique
   names in frozen scope); ambiguous multi-table name resolution is NOT claimed.
5. **Per-connection honesty pinned.** File reopen without re-registering reproduces
   C's `no such collation sequence` error; re-registering restores order.
6. **collation_needed lands (Batch D).** A lazy factory is consulted exactly on
   lookup miss; the factory declining leaves the C error standing (both pinned).

## Out of scope (unchanged)

ICU locale collations, `create_collation16` (UTF-16 name variant),
`sqlite3_collation_needed16`, collation-aware UNIQUE/INDEX b-tree keys on disk,
WAL, planner.

## Consequences

17 goldens (engine-collation-001 ×10, -002 ×5, -003 ×2) replay byte-identical;
anti-cheat proves a runtime-registered reverse collation flips ORDER BY and that
xCompare is really invoked. New cards engine-collation-001/002/003 flip full;
expr-codegen-001 stays partial (broader affinity/collation resolution not claimed).
