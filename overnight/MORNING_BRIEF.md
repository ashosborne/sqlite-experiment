# MORNING BRIEF — sqlite-experiment estate discovery (RESUME RUN 2: residual hunt)

# ⚠️ PHASE A ONLY - UNBOUND CANDIDATES ⚠️

Run: 2026-08-10 resume (12 iterations of MAX 24) · resumed from `5f8992664` · branch `cursor/sqlite-estate-discovery-d22c`
Previous brief preserved: `overnight/MORNING_BRIEF-2026-08-10.md` (run 1: 60 slices / 250 candidates).
Stop condition: **MAX_NEW_CANDIDATES delta cap (120/120) reached** at resume iteration 12. Three thin
vfs-shim hints remain in the residual register — backlog NOT exhausted this time, and either way ≠ estate mapped.

## 1. Counts (this-run delta · counts only, no percentages)

| Metric | This run | Estate total |
| --- | --- | --- |
| Iterations run | 12 (+ one-time re-seed) | 15 + 12 |
| Seeds scanned (new slices) | 47 | 107 |
| New candidate surfaces | 60 | 185 |
| New candidate behaviours | 60 | 185 |
| New candidates (cap) | 120 / 120 | 370 |
| Unscanned hints remaining | 3 (misc-mmapwarm, misc-memtrace, misc-pcachetrace) | 3 |
| Deferred recommendations (prose) | 8 (vfs-unix-variants ×3 platform-gated, templatevtab, rot13, wholenumber-vs-series, jni-java pending consumers, wasm pending browser scope) | — |
| Errors / blockers | 0 (one YAML authoring error caught by parse check + fixed pre-commit; 2 evidence-line corrections journaled) | 0 |

Run-1 rows: **untouched** — all 250 remain `candidate`; no status changes, no deletions, umbrella rows preserved.

## 2. Top new candidates (operator-suggested, NOT bound)

| Behaviour id | Slice | Seam hint | Confidence | Evidence |
| --- | --- | --- | --- | --- |
| global-init-config-003 | global-init-config | db_config toggle inventory — defaults silently shape every other slice | observed-in-code | src/main.c:967 |
| compile-options-omit-enable-001 | compile-options-omit-enable | compileoption diagnostics — the oracle for pinning the baseline build | observed-in-code | src/main.c:5220 |
| global-init-config-002 | global-init-config | sqlite3_config op matrix | observed-in-code | src/main.c:443 |
| wasm-opfs-002 | wasm-opfs | sahpool OPFS VFS (deployment-choice fork) | observed-in-code | ext/wasm/api/sqlite3-vfs-opfs-sahpool.c-pp.js |
| wasm-js-api-002 | wasm-js-api | oo1 DB/Stmt JS API | observed-in-code | ext/wasm/api/sqlite3-api-oo1.c-pp.js |
| vfs-kv-002 | vfs-kv | pluggable sqlite3_kvvfs_methods persistence seam | observed-in-code | src/os_kv.c:330 |
| misc-cksumvfs-001 | misc-cksumvfs | per-page checksums — file-format constraint if ever deployed | observed-in-code | ext/misc/cksumvfs.c:833 |
| misc-regexp-001 | misc-regexp | REGEXP dialect (own NFA, not PCRE) — dialect-parity trap | observed-in-code | ext/misc/regexp.c:901 |
| misc-csv-001 | misc-csv | CSV vtab — filesystem access from SQL (security) | observed-in-code | ext/misc/csv.c:964 |
| jni-java-surface-001 | jni-java-surface | capi Java layer (~40 callback interfaces) | observed-in-code | ext/jni/src/org/sqlite/jni/capi/CApi.java |

Full delta: 47 new slices under `discovery/` (compile-options, 3 platform-VFS slices, global-init-config, 2 wasm slices, jni-java-surface, and 39 thin unbundled ext/misc slices refining the run-1 umbrellas).

## 3. APP_MANIFEST / COVERAGE delta

250 → 370 candidate rows (125→185 surfaces, 125→185 behaviours); scanned_seeds 60 → 107;
unscanned_hints 0 → 11 (re-seed) → expanded to 42 thin hints while walking → 3 remaining.
All new rows `candidate` with `confidence=observed-in-code` and evidence paths. No human-set
statuses existed; none were touched; no umbrella rows modified (thin slices refine, not replace).

## 4. Remaining unscanned hints + honesty metric

- 3 hints remain (cap-stopped mid-unbundle): `misc-mmapwarm`, `misc-memtrace`, `misc-pcachetrace` — all debug/ops trace helpers, low product risk.
- Seeds scanned this run: 47 of 50 identified (11 charter seeds, of which 3 unbundle meta-hints expanded to 42 thin seeds; 39 of those walked). Honesty metric for the seed backlog only — **not** completeness (see §6).

## 5. Recommended human priority order for bind (not auto-bound)

1. **compile-options-omit-enable-001 + global-init-config (all three)** — pin the baseline build + config defaults before characterizing anything else; every other slice's behaviour depends on it.
2. **Scope decisions that unlock/park whole groups:** browser (wasm-js-api / wasm-opfs / vfs-kv), Java (jni-java-surface), platforms (vfs-win / vfs-unix-variants).
3. **Deployed-extension audit for the 39 thin misc slices** — bind only what downstream actually loads; the SME briefs carry per-slice defer recommendations.
4. **Flagged parity traps first among accepted misc slices:** misc-regexp (dialect), misc-cksumvfs (file format), misc-csv/misc-zipfile-sqlar (fs access + zlib).

## 6. Suspected blind spots (unchanged core + new)

- Crash/fault-injection behaviour (harness out of scope) — pager/WAL crash semantics still evidence-thin.
- Generated-code surfaces (tool/-produced); runtime-only contention/memory-pressure behaviour (no static seam — not carded, per charter).
- Per-OMIT/ENABLE surface diffs: awaiting baseline-build pinning; 128 flags not speculatively carded.
- ext/wasm sub-API breadth and ext/jni annotation/test packages — noted, not carded (support code).
- 3 unscanned trace-helper hints (§4).

## 7. What this run did NOT do (explicit)

Did **not** bind, deepen (Phase B), generate tests, RECORD, PACK, BIND, convert, or verify.
Did **not** edit product source under `src/` or `ext/`. Did **not** modify, downgrade, or delete any
run-1 row or artefact (run-1 brief preserved as `MORNING_BRIEF-2026-08-10.md`). Did **not** re-scan
any of run-1's 60 seeds. All writes confined to `discovery/**`, `inventory/**`, `overnight/**`.

## 8. Banner check

`ESTATE_SCAN_INCOMPLETE` — method-coverage checklist addressed for existing families (see
METHOD_COVERAGE.md resume addendum); residuals 2/3/7 + baseline-config decision + 3 unscanned hints remain.

## 9. Completeness

`completeness: incomplete` — human residual gate required. Of the residual seeds scanned in this
resume, these candidate slices were proposed; APP_MANIFEST updated; nothing bound. Never "all slices
in the estate."

## 10. Closing ask

**Bind which slice IDs today?** Suggested first batch: `compile-options-omit-enable`,
`global-init-config` (baseline pinning), plus your scope calls on `wasm-*` / `jni-java-surface` /
`vfs-win` — those decisions accept or park ~20 slices at once.
