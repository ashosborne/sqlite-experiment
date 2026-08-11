# MORNING BRIEF — sqlite-experiment run 11: oneshot 50-slice batch (full autonomy)

Run: 2026-08-11 · from `bd92ddbc4` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-oneshot-50-slices`
Charter: FULL_AUTONOMY, delegated stamp, ≤3 cases/ID (used ≤1 mostly), ≤150 new cases (used 75), NO_ENGINE, NO_WASM.

## 1. The 50 slices

| # | Slice | Frozen this run | Deferred (reason) |
| --- | --- | --- | --- |
| 1 | connection-lifecycle-api | run-10 pair stamped (delegated) | URI/open16/flag-MISUSE (charter list) |
| 2 | exec-convenience-api | run-10 trio stamped (delegated) | multi-statement scope (charter) |
| 3 | error-status-api | 002-C001 (limit protocol) | 003 (status counters nondeterministic — unsafe to freeze) |
| 4 | prepare-statement-api | 004-C001 (coercion), 006-C001 (introspection) | 002-C003 stays BLOCKED (UAF) |
| 5 | auth-callback-api | 001-C001 (DENY→23) | 002 (column-IGNORE plumbing = engine-grade in Rust) |
| 6 | backup-api | 001-C001, 002-C001 (empty pair: DONE/0/0) | 003 (needs concurrent-writer harness) |
| 7 | blob-io-api | — | whole slice (row-store in Rust = engine-grade; NO_ENGINE) |
| 8 | serialize-memdb-api | 001-C001 (empty serialize = 4096B) | 002 (named memdb needs URI — deferred class) |
| 9 | unlock-notify-api | — | whole slice (ENABLE_UNLOCK_NOTIFY off on pin — verified 0) |
| 10 | loadext-api | 001-C001 (disabled → 1/'not authorized') | 002 (registry callback marshalling) |
| 11 | global-init-config | 001/002/003 (init twice, config-MISUSE 21, fkey toggle) | — |
| 12 | pragma-surface | 001 (user_version get/set), 002 (database_list count) — 2 of ≤3 cap | ~70-pragma enumeration (charter forbids) |
| 13 | attach-detach | 001 (attach→2), 002 (detach→1) | 003 (attached-file DDL fixture) |
| 14 | malloc-subsystem | 001 (malloc/msize/free) | 002 (lookaside counters unstable) |
| 15 | mutex-subsystem | 001 (alloc/enter/leave/free) | — |
| 16 | printf-format | 001 (SQL %q), 002 (mprintf %Q NULL), 003 (str builder) | — |
| 17 | util-primitives | 001 (randomness draws differ — derived bool) | — |
| 18 | json-funcs | 001–004 (extract/->/->>; set/remove/patch; valid/type; json_each agg) | — |
| 19 | date-time-funcs | 001–004 (conversions, strftime, modifiers, timediff — TZ-free inputs) | localtime (host-TZ dependent) |
| 20 | builtin-scalar-agg-funcs | 001–003 (scalars; sum/total/count/group_concat; LIKE/GLOB/ESCAPE) | — |
| 21 | ddl-schema | 001–003 (table/view lifecycle, unique index, ALTER chain) | — |
| 22 | dml-codegen | 001 (changes/total_changes), 002 (OR REPLACE/IGNORE) | — |
| 23 | expr-codegen | 001–003 (arith/CAST; IN-NULL 3VL; CASE/iif) | — |
| 24 | select-codegen | 001–003 (subquery ORDER, UNION dedupe, flatten-equivalent) | — |
| 25 | name-resolution | 001 (qualified/rowid), 002 (alias ORDER BY) | — |
| 26 | tokenizer | 001 (hex/exp/blob/bracket), 002 (sqlite3_complete trio) | — |
| 27 | parser-grammar | 001 (keyword fallback), 002 (syntax reject, wording_deferred) | — |
| 28 | analyze-stats | 001 (stat1 row '2 1') | 002 (plan-level observable — EQP-fragile per SME note) |
| 29 | foreign-keys | 001 (violation 19), 002 (CASCADE), 003 (DROP parent 19) | — |
| 30 | triggers | 001 (DDL), 002 (AFTER INSERT fires) | — |
| 31 | upsert | 001 (DO NOTHING), 002 (DO UPDATE excluded.*) | — |
| 32 | vacuum | 001 (VACUUM rc) | 002 (VACUUM INTO file side-effect outside recognizer) |
| 33 | window-functions | 001 (row_number), 002 (ROWS frame sum) | — |
| 34 | introspection-vtabs | 001 (dbstat count>0 — gate ON on pin) | dbpage writes (defensive risk) |
| 35 | vtab-core | — | whole slice (module protocol in Rust = engine-grade) |
| 36 | session | — | whole slice (ENABLE_SESSION off on pin — verified 0) |
| 37–50 | misc-uuid/regexp/series/csv/decimal/basexx/rot13/totype/uint/ieee754/percentile/completion/prefixes/wholenumber | all 14 frozen (static init w/ -DSQLITE_CORE — **no load_extension needed**, so the loadable-defer trigger never fired; uuid pinned as derived shape 36/'4') | — |

**Totals: 46 of 50 slices frozen (75 new cases + the 5 stamped run-10 cases), 4 slices wholly deferred, 0 flaky (two-run determinism gate, zero failures).**

## 2. Engine / wasm untouched

vdbe/btree/pager/pcache/wal/where, all VFS slices, recover/rbu, wasm/jni/tcl/shell, compile-options,
fts/rtree/icu/expert/intck/qrf, remaining misc packs: **not started** (charter out-of-scope list).
`src/ ext/ test/` zero diffs (ext/misc *.c were *compiled* into the legacy harness, never edited).

## 3. Pack

`sqlite-experiment-c-to-rust@2` — **BOUND** (SUPERSEDE v1; bound_by Ash Osborne, delegated; schema-valid).
`in_scope` = **85 HUMAN_ACCEPTED cases**. `versions/1.yaml` + `versions/2.yaml` retained. v2 frozen at end of this batch.

## 4. Branch / goldens / C003

Same branch throughout, zero new branches, zero PRs. The prior 15 goldens (incl. the original ten)
verified **byte-identical** (md5). C003 still BLOCKED — no probe, no step(NULL), harness and crate have
no path that touches a finalized handle.

## 5. Flags

`legacy_green` = **82** (7 prior + 75 newly stamped behaviours). `parity_green` = **0** everywhere —
cargo test is a branch self-check, not Verification COMPARE.

## 6. Rust

`modern/` grew: generated `script_table.rs` (59 frozen scripts → pinned rows) + bespoke mirrors
(limit, authorizer-DENY, backup-empty, serialize-empty=4096, malloc/msize, mutex, randomness,
init/config-MISUSE, db_config-fkey, load_extension-disabled, stmt introspection, coercion, complete,
mprintf/str at frozen arities — varargs caveat noted in lib docs). 42 exported `sqlite3_*` symbols, all frozen-path.
**`cargo test`: 77/77 green** (8 original spine + 58 generated script compares + 11 bespoke).
No `sqlite3.c` link; zero deps.

## 7. completeness: incomplete

85 cases over 82 of 185 documented behaviours; the engine, wasm, bindings, platform VFS, and 4
deferred slices remain. SQLite is **not** migrated; the Rust crate is a pinned-contract recognizer,
not a database. Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes).

## 8. Honest leftover

- Wholly deferred: blob-io-api, unlock-notify-api (gate off), vtab-core, session (gate off).
- Partially deferred: error-status-003, auth-002, backup-003, serialize-002, loadext-002,
  attach-003, malloc-002, analyze-002, vacuum-002, pragma (the other ~68 pragmas), dbpage writes.
- All engine/wasm/binding/platform slices per charter.
- Next operator calls: RECORD the deferred-but-freezable ones with better fixtures, or begin
  Verification COMPARE design (parity is still entirely unverified).
