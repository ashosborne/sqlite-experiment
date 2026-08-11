# ADR 0001 — Bound target: Rust SQLite spine, v1 licenses ten goldens only

Status: accepted (BOUND with the pack, Ash Osborne, 2026-08-11 Europe/London — the run-9 operator
charter is the human bind act).

## Decision

The migration destination is a Rust rewrite of the SQLite library surface (`whole-library-c-to-rust`,
style `incremental-engine-rewrite`, `cdylib`). **Version 1 licenses exactly the ten HUMAN_ACCEPTED
characterization goldens** (error-status-api-001 C001/C002; prepare-statement-api 001/002/003/005 ×
C001/C002) and nothing else. `prepare-statement-api-002-C003` stays BLOCKED and unmapped — stepping
a finalized handle is UAF; the Rust side gets no equivalent probe.

## Why this shape

- Standalone Rust this version: **no link against `sqlite3.c` or `src/`** — a mixed C/Rust link would
  make the parity boundary unfalsifiable. Pinned symbols on the ten paths are Rust; unpinned symbols
  are not exported, so nothing can silently fall through to fantasy behaviour.
- **This is not a VDBE.** v1 implements a recognizer + statement state machine for the pinned SQL
  only. Widening (real parse/execute) is v2+ work under a new pack version and re-bind.
- errmsg wording on ES-C001 is `wording_deferred` (shape-only); the NULL-handle `out of memory`
  string is a contract (sqlite3ErrStr/SQLITE_NOMEM). Third-step-after-DONE is ROW via autoreset on
  this pin — the pack encodes the recorded truth, not the manual.
- A rusqlite-shaped safe API is forbidden in v1: the parity oracle speaks raw C ABI.

## Process facts

One branch (`cursor/sqlite-estate-discovery-d22c`); no per-case branches or PRs; goldens read-only;
`parity_green` untouched (cargo test is a self-check, not factory Verification). Completeness of the
estate remains **incomplete** — this pack covers 10 cases over 5 of 185 documented behaviours.

## Amendment — v2 (run-11 oneshot, 2026-08-11)

v1 SUPERSEDED by v2 under the full-autonomy charter (ALLOW_PACK_SUPERSEDE: true; delegated bind).
v2 widens `in_scope` to 85 HUMAN_ACCEPTED cases (the run-11 batch: 44 slices, 75 new cases). The
recognizer approach is retained and now formalised as a **generated lookup table from frozen
goldens** plus minimal bespoke mirrors for API pins. Still not an engine, not a VDBE, not wasm.
versions/1.yaml retained; v2 frozen at end of the autonomous batch.
