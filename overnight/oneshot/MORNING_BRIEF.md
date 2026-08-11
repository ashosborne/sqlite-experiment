# MORNING BRIEF — sqlite-experiment run 12: oneshot leftovers (full autonomy)

Run: 2026-08-11 · from `44406b4be` · committed as `sqlite-oneshot-leftovers` · same branch, no PR.
(Run-11 brief preserved as `MORNING_BRIEF-2026-08-11-run11.md`.)

## 1. Leftover table

| Item | Outcome |
| --- | --- |
| misc-compress | **frozen** (hex round-trip; zlib linked into legacy harness only) |
| misc-fossildelta | **frozen** (delta_apply/create + output_size) |
| misc-nextchar | **frozen** (indexed 'ca' → 'rt') |
| misc-sha1 | **frozen** (sha1('abc') full digest) |
| misc-shathree | **frozen** (sha3-256('abc') hex digest) |
| misc-urifuncs | **frozen** (sqlite3_uri_parameter/boolean — first capture used wrong fn names, caught pre-freeze and fixed) |
| misc-utilities | **frozen** (eval(); fileio deliberately NOT frozen — fs side effects) |
| misc-zorder | **frozen** (zorder/unzorder ints) |
| misc-func-packs | **frozen** (umbrella: decimal+regexp together — thin members already carded, no explosion) |
| misc-stmt | **DEFERRED** — needs SQLITE_ENABLE_STMTVTAB; the harness pin is the bare-default amalgamation and compile-flag flips are forbidden |
| pragma-surface (Job B) | **14 new cases** (application_id, schema_version, cache_size, recursive_triggers, defer_foreign_keys, query_only, temp_store, automatic_index, ignore_check_constraints, case_sensitive_like-via-LIKE, integrity/quick_check, table_info/fk_list/index_list counts, compile_options count, function/module/pragma_list counts) — all :memory:, no WAL/file pragmas, ~50 others still unenumerated |
| attach-detach-003 | **frozen** (cross-db trigger DDL rejected — error-script, wording_deferred) |
| loadext-api-002 | **frozen** (auto_extension registry: called-on-open / cancel rc=1 / not-called-after) |
| serialize-memdb-api-002 | **frozen without URI** (deserialize the 4096B empty image → OK; re-serialize same size) |
| backup-api-003 | **frozen single-threaded** (source write between steps: step1=OK/remaining 1/pagecount 2 → DONE; rc sequence only, no concurrent-writer circus) |
| vacuum-002, analyze-002, error-status-003, malloc-002, auth-002 | still deferred (charter) |
| blob-io, unlock-notify, session, vtab-core | not retried (charter) |

27 new cases; two-run determinism gate 27/27, 0 flaky. One capture bug (double-free from FREEONCLOSE ownership) fixed in the harness before any freeze.

## 2. Two-level pin finding (honest, important)

The BASELINE fingerprint was dumped from the **CLI**, which carries `SHELL_OPT` enables. The
harnesses link the **bare-default amalgamation** — so run-11's `introspection-vtabs-001-C001` and
`misc-percentile-001-C001` goldens actually pin **feature absence** (rc=1) on the harness lib.
Golden bytes untouched; TRACEABILITY rows annotated; `overnight/BASELINE.md` amended with the
two-level pin. API_ARMOR/OMIT_AUTORESET verified off on both levels — all rc pins stand.

## 3. Pack + goldens + C003

Pack `sqlite-experiment-c-to-rust@3` **BOUND** (SUPERSEDE v2; versions/1+2+3 retained; schema-valid;
**112 in-scope cases**). All 90 prior goldens verified **byte-identical**. C003 still BLOCKED.

## 4. Tests + flags

`cargo test`: **104/104 green** (8 spine + 82 generated script compares + 11 run-11 bespoke + 3 run-12 bespoke).
Exported symbols: 45 — all frozen-path, still no `sqlite3.c` link. `legacy_green` = **95**. `parity_green` = **0**.

## 5. completeness: incomplete

112 cases over 95 of 185 documented behaviours. Not a database. SQLite is not migrated.

## 6. What is left (as predicted: engine/wasm/vtab/gated only)

- Engine: vdbe, btree, pager, pcache, wal, where, vfs-*, recover, rbu
- Wasm/bindings: wasm-*, jni-*, tcl, shell-cli
- Gated/index engines: fts3/5, rtree, geopoly, icu, expert, intck, qrf, session, unlock-notify
- Vtab-protocol slices: vtab-core, blob-io (row store), misc vtab/vfs packs, misc-stmt (flag-gated)
- Census: compile-options; plus the standing partial defers (counters, EQP, file side-effects)
