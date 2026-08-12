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

## Iteration 9 — seeds: pager, wal, pcache, vfs-os-abstraction

- stop.txt: absent.
- pager: 2 candidates (txn lifecycle/2-phase commit, journal-mode state machine). Crash-safety needs fault-injection VFS — the upstream harness lives under test/ (out of allowlist) → recorded as blind spot.
- wal: 2 candidates (write path/snapshots, checkpoint modes). Multi-process behaviour flagged for harness design.
- pcache: 2 candidates (plugin boundary, default LRU impl). Defer recommended unless custom pcache downstream.
- vfs-os-abstraction: 3 candidates (registry, unix impl, win/kv variants). Platform-scope bind question raised.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 182/250 (91 surfaces + 91 behaviours). Slices: 36. Zero-new-streak: 0.

## Iteration 10 — seeds: malloc-subsystem, mutex-subsystem, util-primitives, vtab-core

- stop.txt: absent.
- malloc-subsystem: 2 candidates; mem0..mem5 alternates recorded in-card (compile-time variants, not slices).
- mutex-subsystem: 1 candidate (platform-retained; defer recommended in prose).
- util-primitives: 1 clustered candidate; bitvec/rowset skipped-with-reason (no callable boundary).
- vtab-core: 2 candidates; early bind recommended (all ext vtab slices depend on it).
- Deliberately lean batch (6 features) to stretch the remaining candidate budget across the full ext/ backlog.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 194/250 (97 surfaces + 97 behaviours). Slices: 40. Zero-new-streak: 0.

## Iteration 11 — seeds: introspection-vtabs, shell-cli, tcl-binding, fts5

- stop.txt: absent.
- introspection-vtabs: 1 clustered candidate (dbstat/dbpage/bytecode); dbpage writability flagged as security/scope question.
- shell-cli: 2 candidates (entry+input loop; dot-command surface — refused card-per-command, cluster at bind). shell.c.in preprocessed by tool/mkshellc.tcl (out-of-scope path) — cited template only.
- tcl-binding: 1 candidate; SME question: product adapter vs test-only.
- fts5: 3 candidates (vtab module, tokenizer API, aux funcs/fts5_api). ext/fts5/tool/ skipped-with-reason.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 208/250 (104 surfaces + 104 behaviours). Slices: 44. Zero-new-streak: 0.

## Iteration 12 — seeds: fts3, rtree, geopoly, session

- stop.txt: absent.
- fts3: 1 umbrella candidate; retire-vs-migrate is the bind question.
- rtree: 2 candidates (vtab module, geometry/query callback C API). Shadow-table format flagged.
- geopoly: 1 candidate (rtree-dependent).
- session: 3 candidates (record, apply/conflict, changeset algebra). Binary changeset format = interchange contract, flagged.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 222/250 (111 surfaces + 111 behaviours). Slices: 48. Zero-new-streak: 0.

## Iteration 13 — seeds: rbu, recover, intck, expert

- stop.txt: absent.
- rbu: 2 candidates (resumable apply lifecycle — multi-stage async-like workflow flagged; vacuum mode).
- recover: 2 candidates (recover-to-db, SQL-stream mode). dbdata vtab cited as dependency.
- intck: 1 candidate (evidence corrected to public API lines 801/849/907 after verification).
- expert: 1 candidate (advisory tooling).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 234/250 (117 surfaces + 117 behaviours). Slices: 52. Zero-new-streak: 0.

## Iteration 14 — seeds: icu, qrf, jni-binding, wasm-binding

- stop.txt: absent.
- icu: 1 candidate; ICU-version collation-order parity risk flagged.
- qrf: 1 candidate (single-entrypoint formatter; impl qrf.c:3068).
- jni-binding: 1 umbrella candidate (macro-generated JNI bridges); consumer-evidence SME question.
- wasm-binding: 1 umbrella candidate (JS APIs + OPFS VFS); fiddle UI deprioritized per loop rules; emscripten build tooling skipped-with-reason.
- Lean batch by design: remaining candidate budget reserved so all four ext/misc clusters can still be scanned before the cap.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 242/250 (121 surfaces + 121 behaviours). Slices: 56. Zero-new-streak: 0.

## Iteration 15 — seeds: misc-vfs-shims, misc-func-packs, misc-vtab-packs, misc-utilities

- stop.txt: absent.
- misc-vfs-shims: 1 cluster candidate (9 shims); cksumvfs file-format impact flagged.
- misc-func-packs: 1 cluster candidate (~18 loadable function extensions).
- misc-vtab-packs: 1 cluster candidate (~18 loadable vtabs); generate_series build-config question flagged.
- misc-utilities: 1 cluster candidate (~14 utilities); fileio/eval security posture flagged for audit.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- New candidates this run: 250/250 (125 surfaces + 125 behaviours). Slices: 60. unscanned_hints: 0.

## STOP — run-level stop condition reached

- **MAX_NEW_CANDIDATES (250) reached** at iteration 15 (of MAX_ITERATIONS 40). Loop halts per charter.
- Seed backlog simultaneously exhausted (unscanned_hints: 0) — but the estate scan is still **incomplete by design**: residuals live in METHOD_COVERAGE blind spots (compile-time option matrix, fault-injection behaviour, generated-code surfaces, platform variants), not in the hint list.
- Proceeding to end-of-run artefacts: METHOD_COVERAGE.md + MORNING_BRIEF.md.

## End of run — closing artefacts + validation

- Wrote overnight/METHOD_COVERAGE.md (checklist addressed; absent families recorded observed-absent; residuals 1-8 first-class) and overnight/MORNING_BRIEF.md (10 sections, ESTATE_SCAN_INCOMPLETE banner, completeness: incomplete, closing bind ask).
- Final validation: APP_MANIFEST schema VALID; 60 slice manifests parse, 125 features all candidate with evidence+confidence; statuses = {candidate} only; no completion-% strings anywhere in factory artefacts.
- git working tree touched only discovery/, inventory/, overnight/ throughout (checked before every commit). Product source untouched.
- Final tally: 60 slices, 125 surfaces + 125 behaviours = 250 candidates (cap), 0 hints left in list, residuals recorded in METHOD_COVERAGE.

---

# RESUME RUN 2 — residual hunt (2026-08-10, resumed from 5f8992664)

Charter: MAX_ITERATIONS=24, MAX_NEW_SEEDS_PER_ITER=4, MAX_NEW_CANDIDATES=120 (this-run delta; run-1's 250 rows retained and excluded), STOP_WHEN_NO_NEW_SURFACES=3. See CONTEXT_GATE.md resume section.

## Resume re-seed (once, before iteration 1)

- stop.txt: absent.
- Preserved run-1 brief as overnight/MORNING_BRIEF-2026-08-10.md.
- Re-seeded APP_MANIFEST unscanned_hints with exactly the 11 charter residual seeds (METHOD_COVERAGE residuals 1/4/5/6 + global-init-config open question + three umbrella unbundles). Run-1's 60 scanned seeds NOT re-added.
- Not re-seeded per charter: test/ + src/test*.c (still skipped-with-reason), runtime-only behaviour (stays a METHOD_COVERAGE note), tool//generated files, run-1 seeds.
- Schema validation: PASS. COVERAGE regenerated (hints 0 → 11).
- Delta tally: 0/120 new candidates this run (totals 250 baseline).

## Resume iteration 1 — seeds: compile-options-omit-enable, vfs-win, vfs-kv, vfs-unix-variants

- stop.txt: absent.
- compile-options-omit-enable: 3 candidates (diagnostics API C+SQL; OMIT census 77 refs; ENABLE census 51 refs). Per-option carding deferred until the baseline build is pinned — no fake per-flag cards.
- vfs-win: 2 candidates (winVfs registration/open; win shm+mmap for WAL). Refines run-1 vfs-os-abstraction-003; umbrella row untouched.
- vfs-kv: 2 candidates (kvvfs; pluggable sqlite3_kvvfs_methods seam).
- vfs-unix-variants: 3 candidates (locking-style matrix; proxy/conch locking; VxWorks). These are exactly the entrypoints run-1's unix card left out.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 20/120 (10 surfaces + 10 behaviours). Totals 270. Slices this run: 4 (total 64). Zero-new-streak: 0.

## Resume iteration 2 — seeds: global-init-config, wasm-js-api, wasm-opfs, jni-java-surface

- stop.txt: absent.
- global-init-config: 3 candidates (init/shutdown, config op matrix, db_config toggles — flagged highest-value bind: defaults shape every other slice). Closes the run-1 open question.
- wasm-js-api: 3 candidates (capi projection, oo1, worker1/promiser — async protocol flagged). Refines run-1 wasm-binding umbrella; row untouched.
- wasm-opfs: 2 candidates (async-proxy VFS, sahpool VFS) — the two OPFS strategies with different concurrency/header requirements.
- jni-java-surface: 3 candidates (capi Java layer, wrapper1 OO layer, fts5 bindings). Refines run-1 jni-binding umbrella; row untouched.
- One YAML authoring error (stray key in global-init-config MANIFEST) caught by parse check and fixed before upsert; sqlite3_initialize evidence pinned to src/main.c:360 after verification.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 42/120 (21 surfaces + 21 behaviours). Totals 292. Slices this run: 8 (total 68). Zero-new-streak: 0.

## Resume iteration 3 — hint expansion + seeds: misc-series, misc-csv, misc-zipfile-sqlar, misc-unionvtab

- stop.txt: absent.
- Hint expansion (journaled per re-seed rules): walked `misc-vtab-unbundle` — replaced the meta-hint with 17 concrete thin hints (new SLICE_IDs, prefixed misc-). Run-1 umbrella row misc-vtab-packs-001 untouched.
- Thin-slice generator added (overnight/bin/gen_thin_slices.py) — refuses to overwrite existing slices; each thin slice = 1 evidence-cited candidate feature.
- misc-series (built-in-assumption flag), misc-csv (fs-access security flag), misc-zipfile-sqlar (kept clustered: sqlar genuinely builds on zipfile — one optional pack), misc-unionvtab (incl. swarmvtab, same file).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 50/120 (25 surfaces + 25 behaviours). Totals 300. Slices this run: 12 (total 72). Zero-new-streak: 0.

## Resume iteration 4 — seeds: misc-qpvtab, misc-completion, misc-closure, misc-amatch

- stop.txt: absent. Thin vtab unbundle continues (4 of 17 remaining walked).
- closure carded with "superseded by recursive CTE?" bind question.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 58/120 (29+29). Totals 308. Slices this run: 16 (total 76). Zero-new-streak: 0.

## Resume iteration 5 — seeds: misc-fuzzer, misc-prefixes, misc-wholenumber, misc-stmt

- stop.txt: absent. Thin vtab unbundle continues.
- wholenumber carded with retire-in-favour-of-series bind question.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 66/120 (33+33). Totals 316. Slices this run: 20 (total 80). Zero-new-streak: 0.

## Resume iteration 6 — seeds: misc-templatevtab, misc-vtablog, misc-vtshim, misc-btreeinfo

- stop.txt: absent. Thin vtab unbundle continues.
- templatevtab flagged documentation-only (defer recommendation, prose).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 74/120 (37+37). Totals 324. Slices this run: 24 (total 84). Zero-new-streak: 0.

## Resume iteration 7 — hint expansion + seeds: misc-zorder, misc-basexx, misc-sha1, misc-shathree

- stop.txt: absent.
- misc-zorder completes the 17-slice vtab unbundle (umbrella misc-vtab-packs-001 untouched throughout).
- Hint expansion (journaled): walked `misc-func-unbundle` — replaced with 16 concrete thin hints. basexx kept as the one genuine cluster (basexx.c registers base64+base85 together).
- misc-basexx, misc-sha1 (retire-vs-keep flag), misc-shathree walked.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 82/120 (41+41). Totals 332. Slices this run: 28 (total 88). Zero-new-streak: 0.

## Resume iteration 8 — seeds: misc-decimal, misc-ieee754, misc-percentile, misc-totype

- stop.txt: absent. Thin func unbundle continues.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 90/120 (45+45). Totals 340. Slices this run: 32 (total 92). Zero-new-streak: 0.

## Resume iteration 9 — seeds: misc-uint, misc-regexp, misc-spellfix, misc-nextchar

- stop.txt: absent. Thin func unbundle continues.
- regexp carded with dialect-parity flag (own NFA dialect, not PCRE).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 98/120 (49+49). Totals 348. Slices this run: 36 (total 96). Zero-new-streak: 0.

## Resume iteration 10 — seeds: misc-rot13, misc-uuid, misc-fossildelta, misc-compress

- stop.txt: absent. Thin func unbundle continues.
- rot13 flagged demo-only (defer recommendation, prose).
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 106/120 (53+53). Totals 356. Slices this run: 40 (total 100). Zero-new-streak: 0.

## Resume iteration 11 — hint expansion + seeds: misc-urifuncs, misc-appendvfs, misc-cksumvfs, misc-vfsstat

- stop.txt: absent.
- misc-urifuncs completes the 16-slice func unbundle (umbrella misc-func-packs-001 untouched throughout).
- Hint expansion (journaled): walked `misc-vfs-unbundle` — replaced with 9 concrete thin hints.
- misc-appendvfs, misc-cksumvfs (file-format impact re-flagged from run 1), misc-vfsstat walked.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 114/120 (57+57). Totals 364. Slices this run: 44 (total 104). Zero-new-streak: 0.
- Budget note: 6 rows remain → next batch capped at 3 seeds; remaining vfs hints will stay in the residual register when the cap trips (honest cap stop, per charter).

## Resume iteration 12 — seeds: misc-vfstrace, misc-vfslog, misc-tmstmpvfs (3-seed batch, budget-capped)

- stop.txt: absent. Batch limited to 3 seeds — only 6 rows of the 120-delta budget remained.
- vfstrace (vfstrace_register at vfstrace.c:1138), vfslog (sqlite3_register_vfslog at vfslog.c:755 — evidence pinned after verification), tmstmpvfs walked.
- Upsert + schema validation: PASS. COVERAGE regenerated.
- Delta this run: 120/120 (60 surfaces + 60 behaviours). Totals 370. Slices this run: 47 (total 107). Zero-new-streak: 0.

## STOP — run-level stop condition reached (resume run 2)

- **MAX_NEW_CANDIDATES delta cap (120) reached** at resume iteration 12 (of MAX_ITERATIONS 24). Loop halts per charter.
- Residual register (honest, per charter — empty hints would still ≠ complete): 3 thin vfs-shim hints remain unscanned: misc-mmapwarm, misc-memtrace, misc-pcachetrace (all ext/misc trace/warm helpers).
- Run-1 rows untouched throughout: 250 baseline rows still candidate; umbrella rows (misc-vtab-packs, misc-func-packs, misc-vfs-shims, wasm-binding, jni-binding, vfs-os-abstraction) not modified — refined by 47 new thin/residual slices instead.
- Proceeding to end-of-run artefacts: METHOD_COVERAGE resume section + new MORNING_BRIEF.

## End of resume run 2 — closing artefacts + validation

- METHOD_COVERAGE.md: resume addendum appended (residuals 1/4/5/6 + open question carded; residuals 2/3/7 restated; new residuals: 3 unscanned trace-helper hints, per-option carding awaits baseline pinning).
- MORNING_BRIEF.md: new brief for this resume (run-1 brief preserved as MORNING_BRIEF-2026-08-10.md).
- Final validation: APP_MANIFEST schema VALID; 107 slice manifests, 185 features all candidate+evidence; statuses {candidate} only; estate_scan=partial retained; run-1 rows spot-checked untouched; no completion-% strings.
- Final tally: this run +47 slices, +60 surfaces, +60 behaviours = 120/120 delta (stop). Estate totals: 107 slices, 370 candidates, 3 hints in residual register.

## Run 17 — 2026-08-11 — engine v8: kill script_table (pack v8)

- Pack v7→v8 BOUND (+versions/8.yaml, ADR 0006): cheat-sheet ban, expression/SELECT law,
  function law, anti-cheat law. Schema-validated. completeness: incomplete.
- SCRIPT_TABLE 70 → **0** entries. lib.rs fallback deleted; unknown SQL errors honestly.
- New executor: eval.rs (tokenizer/Pratt parser/typed values, SELECT/UNION/ORDER BY/WHERE,
  aggregates, 2 window shapes, pragmas, ATTACH/DETACH, ~25 computed functions incl. real
  sha1/sha3/json), json.rs (real JSON parse/path/serialize). store.rs execute_script now
  dispatches per-statement (kitchen first, then eval) so mixed scripts run.
- Verdicts: 69 catalogue pins → 50 IMPLEMENT (bytes match frozen goldens via computation),
  19 DEFER (pins + tests removed, reasons in PACK boundaries.deferred_cases_v8).
- Notable honesty point: misc-percentile golden is rc=1 (bare build lacks ENABLE_PERCENTILE);
  the engine deliberately errors on median/percentile to match reality, not to fake success.
- cargo test 136/136; all prior goldens md5-identical; anti-cheat 5/5; parity_green 0.

## Run 18 — 2026-08-11 — engine v9: joins + scalar subqueries (pack v9)

- Pack v8→v9 BOUND (+versions/9.yaml, ADR 0007): subquery law (scalar/EXISTS/correlated,
  LIMIT inside), join law (nested-loop INNER/comma/LEFT, 2-3 tables, no planner claim).
- 24 new cases (engine-subquery 10, engine-join 14) frozen on pinned C, 2-run determinism,
  delegated HUMAN_ACCEPTED, legacy_green 104. ALL 24 replay byte-identical via the executor
  on first pass — zero deferrals in the new batch.
- eval.rs: subquery extraction → Ex::Subq/Ex::Exists with correlated outer-row threading;
  FROM parser with aliases + ON; nested-loop joins incl. real LEFT JOIN NULL-extension;
  real GROUP BY; multi-key ORDER BY ASC/DESC; LIMIT/OFFSET at every level.
- v8 defer pragma-surface-002-C001 RECLAIMED (golden untouched, replays via executor).
- Anti-cheat: runtime-keyed join + runtime scalar subquery + SCRIPT_TABLE still 0.
- cargo 164/164; 155 prior goldens md5-identical; parity_green 0; C003 BLOCKED.

## Run 19 — 2026-08-11 — factory fix: operator progress scoreboard

- Factory gap: no schema signal for "done in modern"; COVERAGE buried conversion progress.
- Core factory: app-manifest schema +impl_in_modern (none|partial|full; converted⇒full;
  partial⇒not converted + notes; verified⇒full); schema.md, Field Guide, Operator runbook,
  conversion-agent + overnight-conductor prompts, conversion-pr + architecture-pack skills
  all require the inventory bump in the same change set; gen_coverage SoT moved to
  migration-factory/inventory/ (overnight wrapper), COVERAGE now leads with Operator progress.
- sqlite-experiment catch-up: 194 behaviours → 11 full (engine slices + exec loop + zorder;
  now converted + parity UNVERIFIED + pack@9), 75 partial (gap notes), 108 none.
  No goldens touched; parity_green 0; no verified; schema VALID.

## Run 20 — 2026-08-11 — engine v10: completion sweep (pack v10)

- Pack v9→v10 BOUND (+versions/10, ADR 0008): completion-sweep law; defers 18→13.
- datetime.rs: real julian-day engine (computeJD/YMD/HMS, modifiers incl. month-overflow,
  weekday, start-of, unixepoch; strftime; timediff). All 4 date/time v8 defers + misc-func-packs
  reclaimed, goldens untouched.
- eval/store: INTERSECT/EXCEPT; CHECK/NOT NULL/OR FAIL (rc19); FK SET NULL/RESTRICT; views
  (create/drop/expand/write-reject); trigger matrix (B/A x I/U/D, old/new, WHEN, per-row);
  printf flags/width/precision; scalar batch; rot13 collation; pragma batch (27);
  eager name resolution (ambiguous/no-such-column at prepare time).
- 37 new goldens (7 sweep slices), 2-run gate, delegated stamp, all replay via executor.
- Inventory bump in-commit: full 11→26, partial 75→72 (8 tightened notes), none 108→103;
  legacy_green 111; parity 0; schema VALID; COVERAGE regenerated.
- cargo 208/208; anti-cheat 10/10; prior goldens md5-identical.

## Run 21 — 2026-08-12 — engine v11: thin-gap harvest (pack v11)

- Pack v10→v11 BOUND (+versions/11, ADR 0009). 33 new goldens (7 harvest slices), 2-run gate,
  delegated stamp, zero deferrals, all replay via executor.
- Implemented for real: DISTINCT/FILTER/HAVING aggregates; ORDER BY COLLATE + NULLS placement;
  sha1_query/sha3_query row-hash protocol; base85/is_base85; ieee754 blobs; decimal(X)/pow2/
  collation; totype/uuid edges; printf %w/#; FK ON UPDATE + SET DEFAULT; INSTEAD OF triggers;
  DROP TRIGGER; RAISE(ABORT) rc19; UPDATE OF; recursive_triggers; RENAME/DROP COLUMN; upsert WHERE.
- Golden-caught bugs: '_' word-boundary in find_kw_top; kitchen SELECT dropping unparseable ORDER BY.
- Scoreboard: full 26→45, partial 72→60 (4 tightened, none greenwashed), none 103; behaviours 208;
  legacy_green 118; parity 0. cargo 243/243; anti-cheat 12/12; prior goldens md5-identical.

## Run 22 — 2026-08-12 — engine v12: disk debt (pack v12)

- Pack v11→v12 BOUND (+versions/12, ADR 0010): overflow law + index/UNIQUE durability law
  (UNIQUE-stripping banned). 20 goldens (engine-overflow 8, engine-indexes 12), 2-run gate,
  delegated stamp, zero deferrals, all Rust-replayed.
- dbfile.rs: real overflow chains (local/spill formula, chain pointers, reader reassembly),
  index b-trees (0x0a; autoindex NULL-sql schema rows, UNIQUE(a,b), explicit indexes),
  Val::Blob serials. store.rs: UNIQUE persists; reopen re-derives enforcement; DROP INDEX;
  NULL-distinct unique fix (C009 golden caught conflict_row NULL bug).
- HONESTY GATES PASS: C integrity_check=ok + exact payload on Rust overflow files; C itself
  rejects duplicates against Rust autoindex b-trees; runtime marker/key anti-cheats.
- ddl-schema-002 kept partial (expression/multi-col explicit indexes, index lookups absent).
- Scoreboard full 45→47; cargo 268/268; 236 prior goldens md5-identical; parity 0.

## Run 23 — 2026-08-12 — engine v13: prepare/bind through the real engine (pack v13)

- Pack v12→v13 BOUND (+versions/13, ADR 0011): statement-API / bind / column laws.
- lib.rs statement rewrite: recognizer DELETED; prepare slices first stmt (pzTail), scans
  ?/?N/:name, resolves names at prepare (dry-run SELECT w/ NULL params; DML catalog check);
  step binds typed values into SQL and executes via the shared store/eval engine; full
  typed bind + column matrices; autoreset real; Val::Real end-to-end (file serial 7).
- 26 bespoke goldens (engine-prepare 001/002/003) frozen on C, 2-run gate, delegated stamp,
  all replayed byte-identical; legacy recognizer pins pass through the real path unchanged.
- Flips: prepare-statement-api-002/003/004 → full; 001/005/006 partial tightened (UTF-16/
  prepFlags, auto-reprepare, EXPLAIN honestly absent). +3 engine-prepare fulls.
- Scoreboard full 47→53; cargo 297/297; anti-cheat 15/15; 256 prior goldens md5-identical.

## Run 24 — 2026-08-12 — engine v14: thin-gap harvest #2 (pack v14)

- Pack v13→v14 BOUND (+versions/14, ADR 0012). 29 goldens (17 script + 12 bespoke);
  EQP join case deliberately NOT frozen (C planner artifacts — no fake planner essay).
- Implemented: general window engine (PARTITION/ORDER/RANGE-peers/ROWS/GROUPS; 10 fns);
  RAISE(IGNORE/FAIL/ROLLBACK) with per-row skip; INSTEAD OF UPDATE/DELETE on view
  projections; upsert multi-assignment (excluded env); printf comma+%p; decimal_exp;
  prepare_v3 prepFlags; auto-reprepare on schema change (DROP->error); EQP honest SCAN;
  EXPLAIN column shape; populated serialize/deserialize via shared dbfile writer/reader;
  sqlite3_str completion. Val-restore incident mid-run: uncommitted store patches lost to
  a git restore and re-applied — checkpoint-commit discipline tightened.
- Flips: +5 partial→full (prepare-005, upsert-002, triggers-002, printf-001, serialize-001),
  6 tightened, +8 new fulls. Scoreboard full 53→66 / partial 52 / none 103.
- cargo 329/329; anti-cheat 18/18; 269 prior goldens md5-identical; parity 0.

## Run 25 — 2026-08-12 — engine v15: transactions (pack v15)

- Pack v14→v15 BOUND (+versions/15, ADR 0013): txn/savepoint/OR ROLLBACK/durability laws.
- Snapshot-model undo (plainly documented — not a pager journal, not WAL): BEGIN/SAVEPOINT
  capture full store snapshots; ROLLBACK [TO] restores (named savepoint survives);
  RELEASE outermost-implicit commits; close auto-rolls-back; counters not rolled back.
- INSERT OR ROLLBACK aborts the whole txn (__TXNROLLBACK__ marker through the error path);
  RAISE(ROLLBACK) unwinds the real txn; sqlite3_get_autocommit exported.
- C truth pinned: DELETE OR ROLLBACK is a SYNTAX ERROR (DELETE has no conflict clause).
- 27 goldens (17 script + 10 bespoke incl. 4 file-reopen), all replayed; anti-cheat 22/22;
  cargo 360/360; 297 prior goldens md5-identical.
- Scoreboard full 66→71 (+5 txn behaviours); dml-codegen-002 honestly kept partial
  (CHECK-on-UPDATE); triggers-002 note aligned with real txns.

## Run 26 — 2026-08-12 — engine v16: CHECK on UPDATE (pack v16)

- Focused single-gap loop. Pack v15→v16 BOUND (+versions/16, ADR 0014): CHECK-on-UPDATE law.
- check_row() evaluates column + (newly captured) table-level CHECKs + NOT NULL against the
  post-update row image; UPDATE OR IGNORE/ABORT/FAIL/ROLLBACK wired to v15 snapshots
  (ABORT statement-atomic, FAIL keeps earlier rows — both pinned; ROLLBACK unwinds txn).
  Table-level CHECKs now also enforced on INSERT.
- 15 goldens (8 script, 5 bespoke, 2 file twins), zero deferrals, all replayed.
- dml-codegen-002 → FULL (matrix complete on INSERT + UPDATE). +3 engine-checkupd fulls.
- Scoreboard full 71→75 / partial 51 / none 103. cargo 377/377 (job-3 commit msg said 379 —
  corrected: 377). anti-cheat 24/24; 324 prior goldens md5-identical; parity 0.
