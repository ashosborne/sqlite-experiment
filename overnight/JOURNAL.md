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
