# ADR 0033 — engine v35: real lookaside pool (malloc-subsystem-002 none-cutter)

Status: accepted · Date: 2026-08-14 · Operator: Ash Osborne · Pack: v35 (supersedes v34)

## Context

`malloc-subsystem-002` (lookaside) was left `none` in run 39 with the note "not honestly
modellable" — modern had no real slab, and runs 39/44 drew the honesty line: **no fake
LOOKASIDE counters without a real pool**. This run builds the pool.

## Presence-check ledger (bare pin probes, run 45)

| Surface | Probe | Result |
| --- | --- | --- |
| lookaside + db_config(LOOKASIDE) | `sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE, …)` | **PRESENT** (default on; BUSY while live) |
| delta_create | `SELECT delta_create('','')` | ABSENT — `no such function: delta_create` → stays none |
| eval (misc-utilities) | `SELECT eval('SELECT 1')` | ABSENT → stays none |
| dbstat (introspection vtabs) | `SELECT * FROM dbstat` | ABSENT → stays none |
| sqlite_stmt | `SELECT * FROM sqlite_stmt` | ABSENT → stays none |
| median/percentile | `SELECT median(1)` | ABSENT → stays none |
| unlock-notify | `used("ENABLE_UNLOCK_NOTIFY")=0` (run 42) | ABSENT → stays none |

No absence "success" goldens were frozen; the run-39 absence pins stay untouched.

## What the C probe fixed (before freezing)

- Fresh connection: USED/HIT all zero; reconfig rc 0. Reconfig while a prepared
  statement is live → **SQLITE_BUSY (5)**; after finalize → 0 again.
- Counter shape: USED = (outstanding, highwater; reset pulls hi to cur);
  HIT / MISS_SIZE / MISS_FULL = (current always 0, counter in highwater; reset clears).
  HIT/MISS counters are **preserved across reconfig** (probe: monotonic across three
  configs); USED highwater resets with the new pool.
- Tiny slots (64) force MISS_SIZE; tiny pool (512×2) with six live statements forces
  MISS_FULL while USED stays positive; draining returns USED to 0; a freed slot HITs
  again. Disable (0,0) zeroes USED and freezes HIT. Negative/huge size and negative
  count normalize with rc 0. Unknown db_config verb → rc 1.

## Modern design vs C

- `LaPool` per connection: slab acquired via the **real counting allocator**
  (so MEMORY_USED accounts it exactly once, like C), slot size rounded down to 8
  (capped 65528), free-list of slot indices, `n_out/mx_out/hits/miss_size/miss_full`.
  Default pool at open: 1200 × 40.
- **What it serves**: modern's per-connection allocation with C's lifetime shape —
  the prepared-statement object. `sqlite3_prepare_v2` placement-allocates
  `Sqlite3Stmt` from a slot (hit) with heap fallback (size/full miss);
  `sqlite3_finalize` drops in place and returns the slot. C additionally routes
  parse-tree/value allocations through lookaside — magnitudes therefore differ, which
  is why every growth pin is a predicate and every zero pin is C-exact (run-38/44
  style). This is a real pool serving real allocations, not invented counters.
- Zombie safety: a connection closed via close_v2 with live statements moves its pool
  to an orphan list; late finalize returns slots there and the slab is freed when the
  last one drains.
- `sqlite3_db_config` is re-exported at a generic fixed arity (Rust stable has no C
  varargs — pack v2 known risk note): `(db, op, a, b, c)` dispatching ENABLE_FKEY
  (val/out) and LOOKASIDE (buf ignored — modern always allocates its own slab, like C
  when pBuf is NULL; sz, cnt), unknown verbs rc 1.

## LOOKASIDE status residuals cleared / left

- Cleared from error-status-api-003: LOOKASIDE_USED/HIT/MISS_SIZE/MISS_FULL are now
  real (run-38's vacuous predicates and run-44's honest zeros are superseded by
  moving counters under the same honesty line).
- Left (named): C's two-size mini-slot carving (modern pool is one-size; the pinned
  predicates hold either way), pBuf-supplied external buffers (modern always
  self-allocates), lookaside for parse-tree/value allocations beyond statement
  objects, SQLITE_CONFIG_LOOKASIDE process-wide default knob, CACHE_SPILL /
  stmt_status / scanstatus (unchanged).

## Nones deliberately left none

Everything in the presence table above, plus FTS/rtree/session/expert, wasm/jni/vfs
shims, and the btree/pager/vdbe/where mega internals. The pcache-001 / stmt_status
stretch was skipped — the lookaside pool consumed the meeting.
