# Context gate — estate discovery loop (sqlite-experiment)

Run started: 2026-08-10T17:42:53Z
Loop prompt: `migration-factory/prompts/estate-discovery-loop-v0.1.md` (v0.1.2, stock)
Operator charter: `migration-factory/prompts/PASTE-estate-discovery-sqlite-experiment.md` (charter wins over loop-file example caps)

## Repo identity

| Field | Value |
| --- | --- |
| Remote | github.com/ashosborne/sqlite-experiment (token redacted) |
| HEAD SHA | db48b1a8ff77c24a6104765d14774d74c235b887 |
| Branch | cursor/sqlite-estate-discovery-d22c (non-default; `NO_COMMITS_TO_DEFAULT_BRANCH` honoured) |
| Estate | SQLite source tree (C library; Fossil-canonical upstream, GitHub mirror-style layout) |

## Context hard gate — required reading (all present, all read)

- [x] `migration-factory/docs/FIELD-GUIDE.md` (App Discovery + Portfolio inventory sections)
- [x] `migration-factory/docs/OPERATOR-RUNBOOK.md`
- [x] `migration-factory/prompts/estate-discovery-loop-v0.1.md`
- [x] `migration-factory/prompts/discovery-agent-v0.2.md`
- [x] `migration-factory/skills/operator/slice-scoping/SKILL.md`
- [x] `migration-factory/schemas/app-manifest.schema.md` + `app-manifest.schema.json`
- [x] `migration-factory/schemas/discovery-manifest.schema.md`

Factory pack complete → no `overnight/BLOCKED.md` required at gate.

## Charter in force (binding values)

```yaml
OPERATOR: Ash Osborne
APP_ID: sqlite-experiment
MAX_ITERATIONS: 40
MAX_NEW_SEEDS_PER_ITER: 4
PHASE_B: false
AUTO_BIND: false
AUTO_ACCEPT: false
ALLOW_CONVERSION: false
STOP_WHEN_NO_NEW_SURFACES: 4
INITIAL_SEEDS: []
ALLOWLIST_PATHS: [src, ext]
OUT_OF_SCOPE_HINTS: [art/, autoconf/, autosetup/, doc/, tool/, mptest/, test/, .fossil-settings/]
MAX_FILES_TOUCHED: 8000
MAX_RUNTIME_HINT_HOURS: 14
MAX_NEW_CANDIDATES: 250
MAX_SLICES_PHASE_A: 0   # 0 = uncapped (stock meaning; no invented default)
WRITE_SCOPE: factory-artefacts-only   # discovery/**, inventory/**, overnight/**
NO_COMMITS_TO_DEFAULT_BRANCH: true
COMMIT_AS: estate-discovery-loop
```

## Prior artefacts / resume state

None. No `discovery/`, `inventory/`, or `overnight/` existed at repo root before this run → fresh bootstrap (iteration 0), not a resume.

## Estate-shape notes for this run

- No HTTP listeners, message queues, or schedulers exist in this estate (C library). "Surface" here = callable seam: public C API family (contract authority `src/sqlite.h.in`), SQL-language surface (built-in function / PRAGMA / virtual table), CLI main (`src/shell.c.in`), language binding (`src/tclsqlite.c`, `ext/jni`, `ext/wasm`), or pluggable boundary (VFS, vtab module, collation, FTS tokenizer).
- APP_MANIFEST surface `kind` enum is closed; all C seams recorded as `kind: other` with descriptive `notes`. No invented enum values.
- `src/test_*.c` and `src/test[1-9].c` (~44 files) are TCL test-harness adapters inside the allowlist → recorded as skipped-with-reason, not product slices.
- Generated files (`sqlite3.c`, `parse.c`, `opcodes.*`, `pragma.h`, `keywordhash.h`) are build artefacts of `src/` inputs → evidence cites the source inputs, never the generated outputs.

## Mode confirmation

Phase A candidates only. No bind, no Phase B deepen, no test generation, no RECORD, no PACK BIND, no Conversion, no completeness claims. Abort cleanly if `overnight/stop.txt` appears; MORNING_BRIEF still written.

---

# Resume run 2 — residual hunt (2026-08-10, same day)

Resumed from branch `cursor/sqlite-estate-discovery-d22c` at `5f8992664` (end of run 1). Prior
`inventory/`, `discovery/`, `overnight/` artefacts retained unchanged. Run-1 brief preserved as
`overnight/MORNING_BRIEF-2026-08-10.md`.

## Why this run exists

Run 1 stopped on MAX_NEW_CANDIDATES 250/250 with the seed backlog exhausted — backlog exhaustion,
not estate mapped. METHOD_COVERAGE residuals 1 (compile-time option matrix), 4 (platform variants),
5 (ext/wasm JS breadth), 6 (ext/jni Java tree) are turned back into hints and walked.

## Resume charter in force (wins over run-1 paste and loop example)

```yaml
OPERATOR: Ash Osborne
APP_ID: sqlite-experiment
RESUME_FROM: cursor/sqlite-estate-discovery-d22c
MAX_ITERATIONS: 24
MAX_NEW_SEEDS_PER_ITER: 4
PHASE_B: false
AUTO_BIND: false
AUTO_ACCEPT: false
ALLOW_CONVERSION: false
STOP_WHEN_NO_NEW_SURFACES: 3
INITIAL_SEEDS: [compile-options-omit-enable, vfs-win, vfs-kv, vfs-unix-variants,
  global-init-config, wasm-js-api, wasm-opfs, jni-java-surface,
  misc-vtab-unbundle, misc-func-unbundle, misc-vfs-unbundle]
ALLOWLIST_PATHS: [src, ext]
OUT_OF_SCOPE_HINTS: [art/, autoconf/, autosetup/, doc/, tool/, mptest/, test/, .fossil-settings/]
MAX_FILES_TOUCHED: 8000
MAX_RUNTIME_HINT_HOURS: 10
MAX_NEW_CANDIDATES: 120   # THIS-RUN delta only; run-1's 250 rows do not count against it
MAX_SLICES_PHASE_A: 0     # uncapped
WRITE_SCOPE: factory-artefacts-only
NO_COMMITS_TO_DEFAULT_BRANCH: true
COMMIT_AS: estate-discovery-loop
```

## Resume rules acknowledged

- Merge by surface_id / locator / behaviour_id; no duplicate cards; no status downgrades (all run-1 rows stay `candidate`).
- No re-scan of run-1's 60 scanned_seeds absent a genuinely new entrypoint.
- Do not re-seed: test/ or src/test*.c; runtime-only contention/memory-pressure (METHOD_COVERAGE note only); tool/ or generated files; run-1 seeds.
- Umbrella leftovers (misc-*-packs, misc-vfs-shims) become thin NEW slices; old umbrella rows untouched.
- Keep `estate_scan=partial` in APP_MANIFEST notes. Empty hints after this run still ≠ complete.
- Candidates only; no bind/Phase B/test gen/conversion; abort on overnight/stop.txt (brief still written).
