# MORNING BRIEF — sqlite-experiment estate discovery

# ⚠️ PHASE A ONLY - UNBOUND CANDIDATES ⚠️

Run: 2026-08-10 (overnight loop, 15 iterations of MAX 40) · HEAD `db48b1a8` · branch `cursor/sqlite-estate-discovery-d22c`
Stop condition: **MAX_NEW_CANDIDATES cap (250) reached** at iteration 15; seed backlog simultaneously exhausted (0 unscanned_hints remaining in list — residuals live in METHOD_COVERAGE blind spots, not the hint list).

## 1. Counts (no percentages — counts only)

| Metric | Count |
| --- | --- |
| Iterations run | 15 (+ bootstrap iter 0) |
| Seeds scanned (slices created) | 60 |
| New candidate surfaces | 125 |
| New candidate behaviours | 125 |
| Total new candidates (cap) | 250 / 250 |
| Deferred recommendations (prose, in SME briefs) | 12 (get_table, legacy trace/profile, unlock-notify, pcache, mutex, tcl-binding, fts3, geopoly, intck, expert, jni-binding, wasm-binding) |
| Errors / blockers | 0 (no BLOCKED.md; 3 evidence-line corrections made and journaled in iters 7, 13, 14) |

## 2. Top candidates for first binds (operator-suggested, NOT bound)

| Behaviour id | Slice | Seam hint | Confidence | Evidence |
| --- | --- | --- | --- | --- |
| prepare-statement-api-002 | prepare-statement-api | sqlite3_step() state machine | observed-in-code | src/vdbeapi.c:980 |
| prepare-statement-api-004 | prepare-statement-api | column access + type coercions | observed-in-code | src/vdbeapi.c:1448 |
| error-status-api-001 | error-status-api | errcode/errmsg contract (foundation for all other slices' tests) | observed-in-code | src/main.c:2743 |
| connection-lifecycle-api-001 | connection-lifecycle-api | open family + URI parsing | observed-in-code | src/main.c:3380 |
| exec-convenience-api-001 | exec-convenience-api | exec callback loop | observed-in-code | src/legacy.c:30 |
| dml-codegen-002 | dml-codegen | ON CONFLICT resolution matrix | observed-in-code | src/insert.c:1901 |
| expr-codegen-002 | expr-codegen | 3-valued NULL logic (classic parity trap) | observed-in-code | src/expr.c:6147 |
| date-time-funcs-003 | date-time-funcs | modifier grammar (tz-dependent — pin TZ) | observed-in-code | src/date.c:260 |
| json-funcs-001/002 | json-funcs | extract + mutate families | observed-in-code | src/json.c:4071 |
| vtab-core-001 | vtab-core | module registration (all ext vtab slices depend on it) | observed-in-code | src/vtab.c:108 |
| session-002 | session | changeset apply/conflict (binary format = contract) | observed-in-code | ext/session/sqlite3session.c:5940 |
| fts5-001 | fts5 | fts5 vtab + MATCH | observed-in-code | ext/fts5/fts5_main.c:3891 |

Full inventory: `inventory/sqlite-experiment/APP_MANIFEST.yaml` (SoT) · glance: `inventory/sqlite-experiment/COVERAGE.md` · per-slice detail: `discovery/<SLICE_ID>/CANDIDATES.md` + `SME_BRIEF.md`.

## 3. APP_MANIFEST / COVERAGE delta

Run started from an empty inventory (bootstrap created the stub). Delta = entire current content:
0 → 125 surfaces (all `candidate`), 0 → 125 behaviours (all `candidate`), 0 → 60 scanned_seeds,
60 → 0 unscanned_hints (every hint either scanned or — none — dropped). All rows carry
`confidence=` labels in notes and evidence paths. No human-set statuses existed; none were touched.

## 4. Remaining unscanned hints + allowlist honesty metric

- `unscanned_hints` list: **empty** — all 60 seeded hints were scanned.
- Allowlist seeds scanned: 60 of 60 identified seeds (honesty metric for the *seed backlog only* —
  **not** a completeness claim; see §6 blind spots and METHOD_COVERAGE residuals 1–7).
- Skipped-with-reason inside the allowlist: `src/test1-9.c` + `src/test_*.c` (~44 TCL harness
  adapters), ext generator/fuzz tooling, `ext/wasm` build helpers (see STRUCTURAL_INDEX §6).

## 5. Recommended human priority order for bind (not auto-bound)

1. **error-status-api** — the error contract underpins every other slice's characterization.
2. **prepare-statement-api + connection-lifecycle-api + exec-convenience-api** — the core callable spine; most characterizable seams in the estate.
3. **SQL-language surfaces with downstream risk:** dml-codegen (ON CONFLICT), expr-codegen (NULL logic), date-time-funcs, json-funcs, builtin-scalar-agg-funcs.
4. **vtab-core** — unlocks every ext vtab slice.
5. **Scope decisions (accept/defer wholesale):** storage stack (btree/pager/wal/pcache/vfs) — migrated vs retained-platform; bindings (tcl/jni/wasm) — product surface vs not; fts3 — retire vs migrate.
6. **Extension slices per downstream usage list:** fts5, session, rtree, rbu, recover, misc clusters (deployment audit for fileio/eval flagged).

## 6. Suspected blind spots

- **Compile-time option matrix** (`SQLITE_OMIT_*`/`SQLITE_ENABLE_*`) — surfaces appear/disappear per build; no per-config scan done. Which build config is the migration baseline?
- **Crash/fault-injection behaviour** — upstream harness lives under `test/` + `src/test_*.c` (out of effective scope); pager/WAL crash semantics currently evidence-thin.
- **Generated-code surfaces** — pragma inventory, parser tables, opcode list are generated by `tool/` scripts (out-of-scope path); cited via source inputs only.
- **Platform variants** — os_win/os_kv/VxWorks/proxy-locking paths inventoried but not deep-scanned.
- **Runtime-only behaviour** — contention, memory-pressure, shared-cache interactions have no static evidence.
- **ext/wasm JS breadth + ext/jni Java tree** — only the C seams were evidenced.

## 7. What this run did NOT do (explicit)

Did **not** bind any candidate. Did **not** run Phase B deepen, Test generation, Test execution
(RECORD/REPLAY), Architecture PACK authoring or BIND, Conversion, or Verification. Did **not**
edit any product source under `src/` or `ext/`. Did **not** flip any MANIFEST feature to
accepted/documented. All writes confined to `discovery/**`, `inventory/**`, `overnight/**`.

## 8. Banner check

`ESTATE_SCAN_INCOMPLETE` — the method-coverage checklist is addressed for families that exist in
this estate (METHOD_COVERAGE.md), but residuals 1–7 there remain unscanned. Zero-diff re-scan ≠ complete.

## 9. Completeness

`completeness: incomplete` — human residual gate required. Of the allowlisted seeds scanned
overnight, these candidate slices were proposed; APP_MANIFEST updated; nothing bound. Residual =
METHOD_COVERAGE blind spots + unknown surfaces (compile-time and runtime-only). Never "all slices
in the estate."

## 10. Closing ask

**Bind which slice IDs today?** Suggested first batch: `error-status-api`,
`prepare-statement-api`, `connection-lifecycle-api`, `exec-convenience-api` (via `record-bind`,
then `deepen-phase-b` on accepted items only).
