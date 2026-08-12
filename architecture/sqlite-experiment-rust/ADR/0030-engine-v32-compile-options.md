# ADR 0030 — engine v32: compile-option diagnostics (used/get, C + SQL, census pins)

Status: accepted · Date: 2026-08-13 · Operator: Ash Osborne · Pack: v32 (supersedes v31)

## Context

`compile-options-omit-enable-001/002/003` were the last untouched present-core `none`
cluster reachable without deepening vtab/attach/WAL/planner. The diagnostics pair
`sqlite3_compileoption_used` / `sqlite3_compileoption_get` (src/main.c) plus the SQL twins
(src/func.c) is the baseline-pinning oracle named by `overnight/BASELINE.md`.

## Presence check (mandatory, passed)

Probed the pinned bare amalgamation (`/tmp/sqlite-build/sqlite3.c`, `gcc -DSQLITE_CORE`,
no OMIT/ENABLE flags — the same harness build every run-38+ pin uses):

- `sqlite3_compileoption_get(0..37)` enumerates **38 options**
  (`ATOMIC_INTRINSICS=1` … `THREADSAFE=1`), then NULL; negative/out-of-range → NULL.
- `used`: `THREADSAFE`→1, `SQLITE_THREADSAFE`→1, `THREADSAFE=1`→1, `THREADSAFE=0`→0,
  unknown→0. Matching is case-insensitive with optional `SQLITE_` prefix and an
  `=`-boundary rule.
- Census: the enumeration contains **zero `OMIT_*` and zero `ENABLE_*` entries**;
  `ENABLE_API_ARMOR`=0 and `OMIT_AUTORESET`=0 confirm the charter PIN line.
- `ENABLE_UNLOCK_NOTIFY`=0 → **unlock-notify-api-001 presence check FAILED**; no
  unlock-notify work this run; the card stays `none` (requires
  SQLITE_ENABLE_UNLOCK_NOTIFY + shared-cache, absent from the pin build).

## Decision

Modern ships the pin build's option table verbatim as a static 38-entry list and answers
`sqlite3_compileoption_used` / `sqlite3_compileoption_get` (C ABI) and
`sqlite_compileoption_used` / `sqlite_compileoption_get` (SQL, in the evaluator's builtin
registry) from it, byte-for-byte against the frozen pins.

Pin decision — the `COMPILER=gcc-13.3.0` row: the table is a *pin fingerprint of the
baseline C build*, not a description of the Rust toolchain. Freezing it (and shipping it
in modern) is what makes the oracle answer identically on both sides; this is recorded
here explicitly so nobody later mistakes the row for a claim that modern was built with
gcc. Same for `MUTEX_PTHREADS`, `SYSTEM_MALLOC`, and the `MAX_*`/`DEFAULT_*` rows: they
restate the pinned baseline's build matrix (`MAX_ATTACHED=10`, `MAX_VARIABLE_NUMBER=32766`
agree with the limits already pinned in earlier runs).

## Census honesty for -002 / -003

The pinned census evidence is exactly what the diagnostics API can show:

- OMIT (-002): every probed `OMIT_*` gate reports 0 (`OMIT_LOAD_EXTENSION`, `OMIT_WAL`,
  `OMIT_VIRTUALTABLE`, `OMIT_TRIGGER`, `OMIT_ATTACH`, `OMIT_SUBQUERY`, `OMIT_VIEW`,
  `OMIT_AUTORESET`, `OMIT_COMPILEOPTION_DIAGS`) and a prefix count over the full
  enumeration finds zero `OMIT_` entries. Flip to **partial**; the 77-guard
  per-feature census (what each OMIT would remove) is NOT claimed.
- ENABLE (-003): same shape (`ENABLE_FTS5`, `ENABLE_FTS3`, `ENABLE_RTREE`,
  `ENABLE_GEOPOLY`, `ENABLE_STAT4`, `ENABLE_API_ARMOR`, `ENABLE_UNLOCK_NOTIFY`,
  `ENABLE_SESSION` all 0; zero `ENABLE_` entries enumerated). Flip to **partial**;
  the 51-guard per-feature census is NOT claimed.

No OMIT/ENABLE surface absent from the pin list is claimed in either direction beyond
"the gate is not active on this pin".

## Consequences

- compile-options-omit-enable-001 flips none → full-or-partial per residuals
  (C + SQL diagnostics real); -002/-003 flip none → partial (census pins only).
- unlock-notify-api-001 stays none (presence check failed; noted above).
- Composed `engine-compile32-*` cards pin the exact frozen batches.
- SCRIPT_TABLE stays empty; anti-cheat pins use runtime-generated option names.
