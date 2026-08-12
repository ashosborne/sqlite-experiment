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

## Run 27 — 2026-08-12 — engine v17: index lookups (pack v17)

- Pack v16→v17 BOUND (+versions/17, ADR 0015): index-lookup / explicit-shape / multi-leaf laws.
- eval: real index-probe path (BTreeMap over durable entries; probe counter); IndexDef
  (multi-col/expr/partial); index_key_for evaluates exprs + partial predicate; unique-index
  conflicts on INSERT. dbfile: 0x02 interior + 0x0a leaf multi-leaf index b-trees. lib: honest
  EQP SEARCH USING INDEX; store::index_for/index_probe_count.
- 20 goldens (12 script + 8 file/EQP bespoke); interop: C integrity_check=ok + C planner uses
  index on Rust multi-leaf files. cargo 401/401; 366 prior goldens md5-identical.
- ddl-schema-002 → FULL (all three v12 residuals closed). upsert-001 tightened (index-expression
  conflict targets remain — kept partial). Scoreboard full 75→79 / partial 50 / none 103.

## Run 28 — 2026-08-12 — engine v18: UDF registration + value/result (pack v18)

- Pack v17→v18 BOUND (+versions/18, ADR 0016): UDF-registration / value-result / no-dlopen laws.
- lib.rs: per-conn UDF registry (thread_local (name,nArg)->FnEntry); create_function[_v2],
  value/result accessors, user_data/aggregate_context/context_db_handle; xDestroy on
  replace/close (fires even when pApp NULL); value_text raw bytes + result_text utf8-detect
  (blob round-trip). eval.rs: ctx.db, udf scalar arm (guarded on udf_name_exists to avoid
  count(*) '*' eval), udf aggregate via eval_agg + expr_has_udf_agg; zeroblob + quote(blob).
- 24 goldens (engine-udf 16, engine-value 8; harness /tmp/udf_harness.c gcc); Rust twin
  engine_udf.rs registers matching extern "C" callbacks. anti-cheat runtime scalar/value/agg.
- Flips: engine-udf-001/002 + engine-value-001 full; loadext-api-001 kept partial (dlopen absent).
- Scoreboard full 79→82 / partial 50 / none 103. cargo 428/428; 390 prior goldens md5-identical.
## Run 29 — 2026-08-12 — engine v19: UTF-16 prepare + column16 (pack v19)

- Pack v18→v19 BOUND (+versions/19, ADR 0017): UTF-16 PREPARE LAW + UTF-16 COLUMN LAW.
- lib.rs: prepare16/_v2/_v3 decode real UTF-16LE buffers (nByte in bytes, <0=to-NUL,
  surrogate pairs) through the shared prepare core; UTF-16 pzTail = consumed prefix
  mapped to code units into the CALLER buffer. column_text16/bytes16/name16/decltype16
  (+ UTF-8 column_decltype) from engine values; bind_text16. store.rs: stmt_decltypes
  parses CREATE TABLE declared types (expressions → NULL).
- Real UTF-16 input exposed two engine bugs, fixed: find_kw_top byte-boundary panic on
  multi-byte SQL; SELECTT mis-dispatch (keywords now word-bounded, incl. stmt_prepare_check).
- 18 goldens (engine-utf16-001 x10 prepare16 family, engine-utf16-002 x8 column16;
  harness /tmp/utf16_harness.c with real unsigned-short buffers). BOM/endian: native-LE
  no-BOM pinned; BOM behaviour not claimed. Rust twin engine_utf16.rs + anti-cheat
  runtime prepare16 / text16 round trip. 410 prior goldens md5-identical.
- Flips: prepare-statement-api-001 partial→full (sole gap closed); engine-utf16-001/002
  new full; util-primitives-001 codec note updated, stays partial (hash/PRNG absent).
- Scoreboard full 82→85 / partial 49 / none 103 (237 known). cargo 449/449.

## Run 30 — 2026-08-12 — engine v20: create_collation + registry COLLATE (pack v20)

- Pack v19→v20 BOUND (+versions/20, ADR 0018): COLLATION-REGISTRATION + REGISTRY-DRIVEN
  COMPARE laws.
- lib.rs: COLL_REG per-connection registry (name→pArg/xCompare/xDestroy; v18 UDF shape);
  create_collation→_v2; NULL-delete / replace / close fire xDestroy (pinned 0/1/2);
  eTextRep 1/2/3/4/8 ok, 0/99→MISUSE; collation_needed factory on lookup miss;
  coll_user_cmp invokes the real C callback with UTF-8 bytes.
- eval.rs: coll_order helper — builtins (binary/nocase/rtrim + rot13/uint/decimal) then
  registry; =/range/ORDER BY rewired; unknown → "no such collation sequence" at prepare;
  ORDER BY resolves names up front; quoted collation names.
- store.rs: Col.coll parsed from CREATE TABLE COLLATE (persists via create_sql, survives
  reopen); build_coll_snapshot → Ctx.col_colls; declared collation drives bare-column
  compares and ORDER BY (residual: by unambiguous column name).
- 17 goldens (engine-collation-001 x10, -002 x5, -003 x2; harness /tmp/coll_harness.c).
  rot13/uint NOT re-homed (stay built-ins; goldens untouched). Rust twin
  engine_collation.rs + anti-cheat runtime reverse collation / invocation counter.
  428 prior goldens md5-identical.
- Flips: engine-collation-001/002/003 new full; expr-codegen-001 stays partial;
  misc-rot13/uint stay full. Scoreboard full 85→88 / partial 49 / none 103 (240 known).
  cargo 469/469.

## Run 31 — 2026-08-12 — engine v21: upsert expression conflict targets (pack v21)

- Pack v20→v21 BOUND (+versions/21, ADR 0019): EXPRESSION-TARGET + EVAL-CONSISTENCY laws.
- store.rs: Stmt::Insert.target parses ON CONFLICT (<expr-list>) [WHERE <pred>] (was
  silently DISCARDED pre-run — every target acted catch-all); resolve_upsert_target
  normalizes case/whitespace and matches UNIQUE IndexDefs (expression/multi-col/partial,
  WHERE must structurally equal) then PK/UNIQUE cols then uniq_sets; mismatch -> pinned
  "ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint" (rc 1);
  targeted_conflict routes through index_key_for (v17); non-targeted conflicts abort
  rc 19 with qualified message (other_conflict_msg).
- 16 goldens (engine-upsert-expr-001 x10, -002 x4 regressions, -003 x2 partial targets;
  harness /tmp/upx_harness.c). Batch C partial-WHERE targets frozen (stable pins), not
  skipped. Rust twin engine_upsert_expr.rs + anti-cheat runtime upsert / mismatch.
  445 prior goldens md5-identical.
- Flips: upsert-001 partial→full (sole named gap closed); engine-upsert-expr-001/002/003
  new full; upsert-002 untouched full. Scoreboard full 88→92 / partial 48 / none 103
  (243 known). cargo 488/488.

## Run 32 — 2026-08-12 — engine v22: first WAL slice (pack v22, ALLOW_WAL lifted)

- Pack v21→v22 BOUND (+versions/22, ADR 0020): WAL-FORMAT / JOURNAL-MODE / CHECKPOINT-MIN
  laws supersede the historic blanket WAL ban. Single-process scope documented.
- dbfile.rs: real WAL format — header magic 0x377f0682 (LE-word cksums), v3007000,
  fixed salts; frame headers with cumulative checksums; write_wal (full committed image
  as one txn), read_wal_overlay (checksum-validated recovery scan), apply_wal_overlay,
  set_journal_versions.
- store.rs: Conn.journal + pending_ckpt; open_file honours header versions=2 + recovers
  -wal; save_file wal branch = C clean close (checkpoint + delete sidecars); wal_sync
  post-exec hook (commit flush / pending checkpoint / wal->delete switch); wal_frame_count.
- eval.rs: journal_mode get/set (wal/delete files, memory :memory:); wal_checkpoint
  pragma family returns counts row, defers file work to wal_sync.
- lib.rs: wal_sync hooks in exec + both step paths; prepared statements now surface
  pragma rows + column names (C parity for PRAGMA journal_mode=WAL via prepare/step).
- 16 goldens (engine-wal-001 x10, engine-wal-002 x6; harness /tmp/wal_harness.c).
  Mandatory: rust_write_c_read_wal (C recovers wal-only data, integrity ok),
  anti_cheat_wal_checkpoint_passive (wal-blind backfill proof), runtime reopen.
  461 prior goldens md5-identical.
- Flips (under-claimed): wal-001 none→partial, wal-002 none→partial (residuals precise);
  engine-wal-001/002 new composed full. Scoreboard full 92→94 / partial 50 / none 101
  (245 known). cargo 508/508.

## Run 33 — 2026-08-13 — engine v23: thin-gap harvest (pack v23)

- Pack v22→v23 BOUND (+versions/23, ADR 0021): THIN-GAP HARVEST LAW (+FpDecode parity).
- 24 goldens / 33 case-ids in 9 batches (engine-harvest23-001..009; harness /tmp/hv_harness.c).
- lib.rs: multi-entry auto-ext registry + reset; malloc accounting in sized_alloc;
  snprintf + str_append; sqlite3_limit id matrix + VARIABLE_NUMBER prepare enforcement;
  errstr + extended_for (2067/1299/275/787); auth_check_action + statement-class gate in
  exec; complete() BEGIN/CASE/END nesting scan.
- eval.rs: first/last/nth_value + ntile + percent_rank + cume_dist; named WINDOW clause;
  RowsFull frame. store.rs: Col.ref_deferred + defer_foreign_keys; COMMIT-time
  fk_violation_exists (txn stays open on failure); qualified UNIQUE/CHECK messages.
- fpdec.rs NEW: faithful port of sqlite3FpDecode/Fp2Convert10/Fp10Convert2 + %!.17g
  assembly — REAL rendering now matches C byte-for-byte incl. double-rounding artifacts
  (1/3 → 0.33333333333333332). All prior REAL pins replay through it.
- Flips: loadext-api-002, malloc-subsystem-001, window-functions-001,
  error-status-api-001/002, foreign-keys-001 → full. Tightened: printf-002/003 (va_list
  platform residual), auth-callback-api-001, tokenizer-002. +9 composed batch cards.
- Scoreboard full 94→109 / partial 44 / none 101 (254 known). cargo 519/519.
  477 prior goldens md5-identical. WAL claims untouched.

## Run 34 — 2026-08-13 — engine v24: VACUUM + VACUUM INTO (pack v24)

- Pack v23→v24 BOUND (+versions/24, ADR 0022): VACUUM LAW (real rebuild; INTO writes a
  fresh C-readable file; canned answers = SCOPE_VIOLATION).
- 18 goldens (engine-vacuum-001 x8 basics, -002 x4 durable/interop, -003 x6 INTO;
  harness /tmp/vac_harness.c). Key C truths pinned: implicit rowids renumber 1,3,5->1,2,3;
  IPK/WITHOUT ROWID keep keys; "cannot VACUUM from within a transaction" (txn survives);
  page_count shrink; INTO exists/txn/rc-14 errors; :memory: export; WAL mode preserved.
- store.rs: Stmt::Vacuum{into} + exec arm (renumber, freelist reset, immediate file+wal
  rewrite, INTO write with pinned errors); freelist page model (refresh_pages after
  INSERT/DELETE/UPDATE; page_hwm grow-only); image_of(&Store) refactor; DELETE arbitrary
  WHERE via eval_standalone (whx fallback); INSERT constant-expression VALUES
  (zeroblob) + one-paren VALUES fix; kitchen rowid projection/sort with IPK aliasing;
  sqlite_master type/name projections + multi-key ORDER BY (kitchen-only for master).
  eval.rs: page_count pragma (freelist model); Conn.page_cur/page_hwm. rc-14 mapping.
- Legacy vacuum-001-C001 golden (deferred since run 11) now replays for real
  (engine_vacuum.rs twin). Anti-cheat: pid-seeded renumbering + runtime INTO target
  read by C. Interop: C reads Rust file after in-place VACUUM (integrity ok).
- Flips (under-claimed): vacuum-001 none→partial (page_size/auto_vacuum apply +
  attached forms residual), vacuum-002 none→partial (URI targets residual);
  engine-vacuum-001/002/003 new composed full. Stretch skipped per charter.
- Scoreboard full 109→112 / partial 46 / none 99 (257 known). cargo 541/541.
  510 prior goldens md5-identical.

## Run 35 — 2026-08-13 — engine v25: incremental blob I/O (pack v25)

- Pack v24→v25 BOUND (+versions/25, ADR 0023): BLOB I/O LAW (live handles on real
  cell payload; canned bytes = SCOPE_VIOLATION).
- 22 goldens (engine-blob-001 x8 lifecycle, -002 x10 I/O+bounds+expiry, -003 x4
  durable/interop; harness /tmp/blob_harness.c). Pinned C truths: validation order +
  exact errmsgs; bounds rc 1 "SQL logic error" with untouched buffers; RO write rc 8;
  expiry rc 4 "query aborted" + bytes->0; zero-length edges; text cells readable;
  errmsg "not an error" after success.
- lib.rs: Sqlite3Blob handle (db, table, ci, rowid, readonly, connection-write marker);
  sqlite3_blob_open/close/reopen/bytes/read/write; expiry via wal_marker comparison
  (residual: connection-write granular vs C per-row — documented). store.rs:
  blob_target (pinned validation order, IPK aliasing, indexed-write refusal),
  blob_len/read_bytes/write_bytes (writes never resize, never bump change counters —
  a handle must not expire itself); UPDATE SET constant-expression fallback (zeroblob).
- Anti-cheat: pid-seeded payload + runtime offset + expiry proof. Interop: C CLI reads
  a Rust file whose bytes were written only through a handle (integrity ok).
- Flips (under-claimed): blob-io-api-001 none→partial, blob-io-api-002 none→partial;
  engine-blob-001/002/003 new composed full. Stretch skipped per charter.
- Scoreboard full 112→115 / partial 48 / none 97 (260 known). cargo 565/565.
  528 prior goldens md5-identical.

## Run 36 — 2026-08-13 — engine v26: connection lifecycle (pack v26)

- Pack v25→v26 BOUND (+versions/26, ADR 0024): CONNECTION LIFECYCLE LAW.
- 22 goldens (engine-conn-001 x6 close/close_v2, -002 x6 busy, -003 x10 hooks/trace;
  harness /tmp/conn_harness.c). C truths: close rc 5 exact errmsg (blob handles count;
  reset does not unblock); close_v2 zombie keeps stmts usable; open-txn close rolls
  back; busy handler counts 1/3 + "database is locked"; timeout<->handler exclusivity;
  update_hook [op main tbl rowid]; commit_hook 3-fire count + abort rc 19; trace
  STMT text / ROW 3 / CLOSE 1 / PROFILE 2 / mask-0 unset.
- lib.rs: STMTS/BLOBS/ZOMBIES registries; conn_teardown extraction (TRACE_CLOSE);
  busy_handler/busy_timeout + busy_should_retry; commit/update/trace registrations
  with prior-arg returns; trace fires in exec (STMT/PROFILE) + step (ROW).
- store.rs: in-process FILE_LOCKS (BEGIN IMMEDIATE holds; write statements consult
  busy loop; COMMIT/ROLLBACK/close release); FILE_VERSIONS + commit-time delete-mode
  flush + maybe_refresh_from_file sibling reload; commit_flush_pending flag (COMMIT
  marker moved in an earlier exec); commit_hook consult at Commit arm + snapshot-based
  autocommit abort; update_hook fires at insert/replace/update/delete with IPK-aliased
  rowids; rc 5 mapping.
- Anti-cheat: close-BUSY/finalize cycle, pid-seeded update_hook log, runtime
  commit_hook abort. 550 prior goldens md5-identical.
- Flips (under-claimed): connection-lifecycle-api-002/003/004 none→partial (residuals
  precise: backup coupling, single-process lock model, trace granularity);
  engine-conn-001/002/003 new composed full. Stretch skipped.
- Scoreboard full 115→118 / partial 51 / none 94 (263 known). cargo 589/589.

## Run 37 — 2026-08-13 — engine v27: ANALYZE -> sqlite_stat1 (pack v27)

- Pack v26→v27 BOUND (+versions/27, ADR 0025): ANALYZE LAW; planner load explicitly
  NOT claimed (zero plan pins); STAT4 off on the pinned build.
- 17 goldens (engine-analyze-001 x15, -002 x2; harness /tmp/an_harness.c). C truths:
  empty table -> stat1 exists, no rows; NULL-idx row only for index-less tables; one
  row per index otherwise; multi-column prefixes "6 3 2"; ANALYZE <index> touches only
  that row; DROP maintenance; WITHOUT ROWID "w|w|2 1"; the near-1.0 rounding quirk
  "11 1" (formula from src/analyze.c statGet, ported exactly).
- store.rs: Stmt::Analyze parse/exec; sqlite_stat1 as an ORDINARY catalog table
  (durable via dbfile, VACUUM-safe, C-readable both directions); stat1_text over
  index_key_for tuples; stat1_ival ceil + quirk; pk_cols for WITHOUT ROWID pseudo-
  index; stat1_delete/insert maintenance wired into DROP INDEX / DROP TABLE arms.
- Also: serialized the run-30 collation_needed twins (shared capture globals across
  test threads — latent flake, 8 consecutive clean runs after).
- Anti-cheat: pid-seeded table + runtime row count -> computed stat integers.
  Interop: C reads Rust stats; modern reads C's. 572 prior goldens md5-identical.
- Flips (under-claimed): analyze-stats-001 none→partial (STAT4/optimize/attached/
  annotations residual); analyze-stats-002 KEPT none ("planner cost model not
  claimed"); engine-analyze-001/002 new composed full.
- Scoreboard full 118→120 / partial 52 / none 93 (265 known). cargo 609/609.

## Run 38 — 2026-08-13 — engine v28: mega harvest (pack v28)

- Pack v27→v28 BOUND (+versions/28, ADR 0026): MEGA-HARVEST LAW; extension functions only
  where the pinned baseline provides them.
- 33 goldens (engine-harvest28-001..008 minus dropped 005; harness /tmp/hv28_harness.c).
- TWO-LEVEL-PIN CATCH: force-linking ext .c files hid baseline membership. Prior misc-*
  goldens decided it: compress/next_char/wholenumber/completion bundled (rc=0) → honest;
  percentile/median absent (misc-percentile-001 rc=1) → implemented then REVERTED + batch F
  dropped (would break the prior bare-build golden).
- lib.rs: sqlite3_get_table/free_table (char** marshal, NULL ptrs, 0x0 empty/non-query,
  errmsg-out); sqlite3_status64 (MEMORY_USED + bad-op MISUSE) / db_status (LOOKASIDE/
  SCHEMA_USED + bad-op ERROR); authorizer READ helpers; string/comment-aware complete().
- store.rs: apply_read_auth prepass (READ per referenced column in select-list order,
  IGNORE→null the snapshot; suppressed during the prepare probe); CreateVtab; schema_footprint;
  ident() quoted-identifier stripping; quoted FROM lookup.
- eval.rs: BETWEEN operator; blob!=text in vnum_eq; SELECT * expansion; compress/uncompress
  (RLE), next_char scalar; completion/pragma_function_list/pragma_pragma_list TVFs;
  wholenumber bounded generator (WN_BOUND from max int literal).
- Flips: get_table + auth-002 + nextchar → full; status-003/compress/wholenumber/completion/
  parser-grammar-002 none→partial; tokenizer-002 + pragma-surface-002 tightened;
  misc-percentile-001 kept none; +7 composed engine-harvest28 full cards.
- Also fixed a latent flake in run-30 collation twins earlier; this run touched none.
- Scoreboard full 120→130 / partial 52→57 / none 93→85 (272 known). cargo 644/644.
  589 prior goldens md5-identical.

## Run 39 — 2026-08-13 — engine v29: none-batch, baseline-honest (pack v29)

- Pack v28→v29 BOUND (+versions/29, ADR 0027): NONE-BATCH LAW; mandatory bare-amalgamation
  presence checks before implementing any extension surface.
- Presence check (no-ext compile of sqlite3.c): delta_create / eval / dbstat / sqlite_dbpage
  / bytecode / sqlite_stmt ALL ABSENT → misc-fossildelta/utilities/introspection/stmt kept
  none (prior rc=0 goldens were force-linked; run-38 percentile lesson generalized).
  Lookaside slab + variadic db_config not modellable → malloc-subsystem-002 kept none.
- Implemented attach-detach-003 core rule: qualified table names in non-TEMP trigger DML
  rejected with C's exact message (trigger not created); TEMP exempt; qualified SELECT
  allowed; multi-stmt body fails whole. store.rs: TriggerReject/TriggerNoop stmts + parse
  scan; split_statements keeps TEMP/TEMPORARY TRIGGER whole.
- 6 goldens (engine-none29-001; C004 dropped — INSERT-SELECT trigger body unsupported).
  anti_cheat over runtime table names. 622 prior goldens md5-identical.
- Flips: attach-detach-003 none→partial; engine-none29-001 new composed full;
  fossildelta/utilities/introspection/malloc-002 kept none (documented).
- Scoreboard full 130→131 / partial 57→58 / none 85→84 (273 known). cargo 652/652.

## Run 40 — 2026-08-13 — engine v30: attached-schema ownership (pack v30)

- Pack v29→v30 BOUND (+versions/30, ADR 0028): ATTACHED-SCHEMA LAW. ATTACH/DETACH core,
  bare-amalgamation pins.
- store.rs: ident() accepts schema.table (main/temp->bare); eval_snapshot with qualified+
  unqualified aliases (main wins collision); image_of excludes dotted keys; Stmt::Attach/
  Detach + exec arms (dup/reserved/missing errors, teardown of schema.* objects);
  ATTACHED_PATHS + load_attached_image/attached_image + save_attached (durable file aux);
  attached-schema VIEW cross-schema reference rejection. lib.rs: save_attached wired into
  close. eval.rs: pragma_database_list carries seq; ORDER BY unknown/non-projected column
  skipped instead of defaulting to col 0.
- 14 goldens (engine-attach30-001 x8, -002 x4, -003 x2; harness /tmp/attach_harness.c).
  Dropped C003-of-003 (trigger firing on attached table) + a sqlite_master WHERE 0 probe.
  anti_cheat over runtime schema/table + DETACH-gone. 628 prior goldens md5-identical.
- Flips: attach-detach-001/002/003 deepened (still partial, residuals shrank);
  engine-attach30-001/002/003 new composed full.
- Scoreboard full 131->134 / partial 58 / none 84 (276 known). cargo 668/668.

## Run 41 — 2026-08-13 — engine v31: vtab core (pack v31)

- Pack v30→v31 BOUND (+versions/31, ADR 0029): VTAB-CORE LAW. create_module/declare_vtab
  are core C API on the bare amalgamation; pins use tiny in-process test modules
  (intseries with HIDDEN lim, pairtab) — no ext/misc force-linking.
- lib.rs: C-ABI Sqlite3Module/Sqlite3Vtab/Sqlite3VtabCursor/Sqlite3IndexInfo;
  sqlite3_create_module/_v2 per-connection registry (replace + close run the _v2
  destructor); sqlite3_declare_vtab (MISUSE outside a constructor; parses names/types/
  HIDDEN); vtab_create_instance drives xCreate with the C argv convention (pzErr
  surfaced, no schema entry on failure); vtab_scan drives xOpen/xBestIndex(0-constraint)/
  xFilter/xEof/xColumn/xNext/xClose; vtab_drop_instance -> xDestroy; vtab_close wired
  into conn_teardown.
- store.rs: CreateVtab carries raw args + sql; exec arm: registered module -> real path,
  wholenumber -> legacy harvest28 generator (NOT re-homed; see ADR 0029), else
  "no such module: X"; DROP TABLE vtab arm; sqlite_master rootpage-0/sql projections for
  vtab entries. eval.rs: source_rows vtab cursor hook (rows carry HIDDEN cols; SELECT *
  expands to the visible declared shape); pragma_table_info reports vtab name/type.
- 25 goldens (engine-vtab31-001 x10, -002 x8, -003 x7; harness /tmp/vtab_harness.c,
  two-run deterministic). 3 anti-cheat tests (runtime module/table names + payload,
  reshape on recreate, runtime unknown-module exact error). 642 prior goldens untouched.
- Flips: vtab-core-001 none->partial, vtab-core-002 none->partial;
  engine-vtab31-001/002/003 new composed full; misc-vtab-packs-001 stays none (notes).
- Scoreboard full 134->137 / partial 58->60 / none 84->82 (279 known). cargo 696/696.

## Run 42 — 2026-08-13 — engine v32: compile-option diagnostics (pack v32)

- Pack v31→v32 BOUND (+versions/32, ADR 0030): COMPILE-OPTIONS LAW. Presence checks:
  diagnostics API present on the pinned bare build (38-entry fingerprint, COMPILER=
  gcc-13.3.0 row frozen as a pin decision); generate_series ABSENT from the bare
  amalgamation (planned SQL census-count pin dropped); ENABLE_UNLOCK_NOTIFY=0 so
  unlock-notify-api-001 stays none with zero cases.
- lib.rs: static 38-entry COMPILE_OPTS table; sqlite3_compileoption_used (case-
  insensitive, optional SQLITE_ prefix, '=' boundary rule, unknown/empty->0) and
  sqlite3_compileoption_get (C order, NULL past either end) + pub helpers.
  eval.rs: sqlite_compileoption_used / sqlite_compileoption_get SQL twins.
- 19 goldens (engine-compile32-001 x11 diagnostics, -002 x4 OMIT census, -003 x4
  ENABLE census; harness /tmp/copt_harness.c, two-run deterministic). 2 anti-cheat
  tests (runtime fake option -> 0 via C+SQL; C/SQL enumeration round-trip, len 38,
  every entry used()=1). 667 prior goldens untouched.
- Flips: compile-options-omit-enable-001 none->full (pinned seam), -002/-003
  none->partial (census only; 77/51-guard per-feature census NOT claimed);
  unlock-notify-api-001 stays none (presence note); engine-compile32-001/002/003
  new composed full.
- Scoreboard full 137->141 / partial 60->62 / none 82->79 (282 known). cargo 717/717.

