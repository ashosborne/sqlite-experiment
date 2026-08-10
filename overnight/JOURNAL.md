# Journal — estate discovery loop (sqlite-experiment)

Run started 2026-08-10T17:42:53Z · HEAD db48b1a8 · branch cursor/sqlite-estate-discovery-d22c
Charter: MAX_ITERATIONS=40, MAX_NEW_SEEDS_PER_ITER=4, MAX_NEW_CANDIDATES=250, MAX_SLICES_PHASE_A=0 (uncapped), STOP_WHEN_NO_NEW_SURFACES=4.
Running tally format: candidates = surfaces + behaviours in APP_MANIFEST (all agent-written rows are candidate/unknown; run started from zero, so totals == new-this-run).

---

## Iteration 0 — bootstrap

- stop.txt check: absent → proceed.
- Context gate passed: factory pack complete; wrote `overnight/CONTEXT_GATE.md`.
- Cheap structural index written: `overnight/STRUCTURAL_INDEX.md` — public C API families (evidence: `src/sqlite.h.in` declaration lines), SQL-function/pragma registration tables, 64 `sqlite3_*_init` loadable-extension entrypoints in `ext/`, 7 CLI mains, 3 language bindings. No HTTP/messaging/schedulers exist in this estate (observed).
- Created `inventory/sqlite-experiment/APP_MANIFEST.yaml` stub: status=in_progress, completeness=incomplete, 60 unscanned_hints (full seed backlog: 43 src seams + 17 ext seams).
- Tooling under `overnight/bin/`: `upsert_manifest.py` (slice MANIFEST → APP_MANIFEST upsert; merge by locator/behaviour_id; never touches human statuses; jsonschema-validates) + `gen_coverage.py` (COVERAGE.md generator — never hand-edited).
- Ran upsert: 0 slices, 0 candidates (expected — nothing scanned yet). Schema validation: PASS.
- Skipped-with-reason (recorded in STRUCTURAL_INDEX §6): `src/test*.c` TCL harness adapters (~44 files), generator/fuzz tooling in ext subtrees, generated build outputs.
- New candidates this run so far: 0 / 250. Slices created: 0 (no cap).

## Iteration 1 — seeds: connection-lifecycle-api, prepare-statement-api, exec-convenience-api, backup-api

- stop.txt: absent. Batch picked from top of hint backlog (callable API boundaries first).
- connection-lifecycle-api: 4 candidates (open/URI-parse, deferred close, busy handling, hooks/trace). Evidence src/main.c. Noted global init/config as open boundary question — kept in backlog implicitly via error-status seed notes.
- prepare-statement-api: 6 candidates (prepare family, step contract, bind, column coercions, reset/finalize/reprepare, stmt introspection). Flagged auto-reprepare as hidden retry seam.
- exec-convenience-api: 2 candidates (exec callback loop, get_table). Recommended defer (prose) for get_table — legacy API.
- backup-api: 3 candidates; concurrent-write coordination flagged as needing two-connection harness.
- Skipped: nothing new; src/test*.c remain skipped-with-reason (iteration 0).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 30/250 (15 surfaces + 15 behaviours). Slices: 4. Zero-new-streak: 0.

## Iteration 2 — seeds: blob-io-api, serialize-memdb-api, loadext-api, unlock-notify-api

- stop.txt: absent.
- blob-io-api: 2 candidates (handle lifecycle; read/write bounds + expiry).
- serialize-memdb-api: 2 candidates (byte-image round-trip; memdb VFS). Compile-flag caveat noted.
- loadext-api: 2 candidates (dlopen gate — security flag raised; auto-extension registry).
- unlock-notify-api: 1 candidate; fire-and-forget async seam flagged for split at bind.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 44/250 (22 surfaces + 22 behaviours). Slices: 8. Zero-new-streak: 0.

## Iteration 3 — seeds: auth-callback-api, attach-detach, error-status-api, builtin-scalar-agg-funcs

- stop.txt: absent.
- auth-callback-api: 2 candidates (registration/dispatch; column-read IGNORE→NULL).
- attach-detach: 3 candidates (ATTACH, DETACH, cross-db name fixation).
- error-status-api: 3 candidates (error family, limits, status counters). Recommended early bind — other slices' characterization depends on the error contract.
- builtin-scalar-agg-funcs: 3 clustered candidates over ~111 registry rows (scalars, aggregates, LIKE/GLOB). Refused card-per-function noise; sub-clustering recommended at bind.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 66/250 (33 surfaces + 33 behaviours). Slices: 12. Zero-new-streak: 0.

## Iteration 4 — seeds: date-time-funcs, json-funcs, printf-format, pragma-surface

- stop.txt: absent.
- date-time-funcs: 4 candidates; localtime tz-dependence flagged for test pinning.
- json-funcs: 4 clustered candidates over 33 registry rows (extract, mutate, validate, each/tree vtabs); JSONB format-stability question raised.
- printf-format: 3 candidates (SQL func, C API, str builder); %q/%Q/%w quoting flagged as injection-safety relevant.
- pragma-surface: 2 candidates (dispatcher + pragma vtabs); pragma table is generated from tool/mkpragmatab.tcl (out-of-scope path) — cited src/pragma.c only.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 92/250 (46 surfaces + 46 behaviours). Slices: 16. Zero-new-streak: 0.

## Iteration 5 — seeds: window-functions, upsert, triggers, foreign-keys

- stop.txt: absent.
- window-functions: 2 candidates (built-in family, frame execution).
- upsert: 2 candidates (conflict-target resolution, DO UPDATE path).
- triggers: 2 candidates; cascade chains flagged as async-like for bind-time card splitting.
- foreign-keys: 3 candidates; pragma-gated default (off) raised as SME question.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 110/250 (55 surfaces + 55 behaviours). Slices: 20. Zero-new-streak: 0.

## Iteration 6 — seeds: ddl-schema, analyze-stats, vacuum, tokenizer

- stop.txt: absent.
- ddl-schema: 3 candidates (table/view, index, ALTER family — schema-rewrite risk flagged).
- analyze-stats: 2 candidates (ANALYZE write path, stats load); plan-stability caveat for characterization.
- vacuum: 2 candidates (rebuild, VACUUM INTO).
- tokenizer: 2 candidates (driver loop, sqlite3_complete); keywordhash.h is generated — cited src inputs only.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 128/250 (64 surfaces + 64 behaviours). Slices: 24. Zero-new-streak: 0.

## Iteration 7 — seeds: parser-grammar, name-resolution, expr-codegen, select-codegen

- stop.txt: absent.
- parser-grammar: 2 candidates (417 productions as dialect umbrella; keyword fallback). Never cited generated parse.c.
- name-resolution: 2 candidates (lookup/ambiguity; ORDER/GROUP BY alias rules).
- expr-codegen: 3 candidates (value codegen, 3-valued boolean jumps — NULL-semantics parity trap flagged, expr equivalence).
- select-codegen: 3 candidates (orchestration, compound selects, subquery flattening — silent plan-shape risk). Fixed one evidence line (sqlite3Select at select.c:7642) after verification.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 148/250 (74 surfaces + 74 behaviours). Slices: 28. Zero-new-streak: 0.

## Iteration 8 — seeds: dml-codegen, where-optimizer, vdbe-engine, btree

- stop.txt: absent.
- dml-codegen: 2 candidates (3 write paths + xfer opt; conflict-resolution matrix — REPLACE side effects flagged).
- where-optimizer: 2 candidates; plan-vs-result characterization level raised for SME.
- vdbe-engine: 2 deliberately coarse candidates (interpreter, Mem-cell typing); SQL-level characterization recommended; sorter noted as sub-seam.
- btree: 2 coarse candidates; SME question raised whether storage layer is migration scope or retained platform.
- Generated headers (opcodes.h etc.) never cited; evidence from src/vdbe.c and src/btree.c only.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 164/250 (82 surfaces + 82 behaviours). Slices: 32. Zero-new-streak: 0.
