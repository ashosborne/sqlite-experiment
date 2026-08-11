# Journal — run 3: bind-all + Phase B full-catalogue deepen (sqlite-experiment)

Charter: user paste = human bind (ACCEPT_ALL_CURRENT_CANDIDATES) + Phase B deepen for all accepted.
COMMIT_AS sqlite-deepen-all. No test gen / RECORD / PACK / Conversion. Writes: discovery/, inventory/, overnight/ only.
Resumed from 0f1fa004 on cursor/sqlite-estate-discovery-d22c.

---

## Setup + baseline + bind

- stop.txt: absent. Read record-bind + deepen-phase-b skills (context gate: Field Guide, runbook, discovery v0.2, both schemas re-read).
- Tooling added under overnight/bin/: bind_all.py (bind recorder), phase_b_upsert.py (Phase B status sync — run-1 radar upsert NOT used here; it force-writes candidate), gen_cards.py (v0.2 card writer; structural fields from slice MANIFESTs — no invented locators; resume-safe).
- overnight/BASELINE.md written: default Unix amalgamation pin + compileoption fingerprint; census cards stay one card each; completeness incomplete.
- BIND: 185 features candidate→accepted across 107 slice MANIFESTs; APP_MANIFEST behaviours candidate→accepted (185). Surfaces intentionally left candidate (charter binds behaviours; decision recorded in BIND_ALL.md). 3 unscanned hints untouched. Schema VALID.
- Bind record: overnight/phase-b/BIND_ALL.md — "Ash Osborne, 2026-08-11 Europe/London, ACCEPT_ALL_CURRENT_CANDIDATES."

## Batch 1 — core API spine (15 cards)

- stop.txt: absent.
- connection-lifecycle-api 001-004, prepare-statement-api 001-006, exec-convenience-api 001-002, backup-api 001-003 → documented, all observed-in-code (symbol+line evidence).
- Coercion matrix (prepare-statement-api-004) kept as one card pinning the seam; matrix rows deferred to Test-gen granularity.
- Upsert: PASS (documented=15). COVERAGE regenerated.

## Batch 2 — remaining public API seams (15 cards)

- blob-io(2), serialize-memdb(2), loadext(2), unlock-notify(1), auth(2), attach(3), error-status(3) → documented, observed-in-code.
- unlock-notify card documents the gated contract explicitly (ENABLE_UNLOCK_NOTIFY + shared-cache NOT in the default baseline build).
- Upsert: PASS (documented=30). COVERAGE regenerated.

## Batch 3 — SQL function surfaces (14 cards)

- builtin-scalar-agg-funcs(3), date-time-funcs(4), json-funcs(4), printf-format(3) → documented, observed-in-code.
- Registry/family cards stay clustered per charter (no per-function explosion); parity traps carried into cards (ASCII-only LIKE folding, sum overflow, %q/%Q, localtime TZ pinning, JSON5-in/canonical-out).
- Upsert: PASS (documented=44). COVERAGE regenerated.

## Batch 4 — SQL-language surfaces (14 cards)

- pragma-surface(2), window-functions(2), upsert(2), triggers(2), foreign-keys(3), ddl-schema(3) → documented, observed-in-code.
- Pragma census stays one card (charter); trigger/FK cascade behaviour carded with the run-1 async-chain flags preserved.
- Upsert: PASS (documented=58). COVERAGE regenerated.

## Batch 5 — maintenance + compiler front-end (13 cards)

- analyze-stats(2), vacuum(2), tokenizer(2), parser-grammar(2), name-resolution(2), expr-codegen(3) → documented, observed-in-code.
- Dialect gates carded explicitly (DQS fallback, keyword fallback, legacy_alter interactions); IN-with-NULL parity trap pinned on expr-codegen-002.
- Upsert: PASS (documented=71). COVERAGE regenerated.

## Batch 6 — SELECT/DML/planner/engine (11 cards)

- select-codegen(3), dml-codegen(2), where-optimizer(2), vdbe-engine(2), btree(2) → documented, observed-in-code.
- vdbe-engine-001 stays the 199-opcode census card (charter); bare-column-in-aggregate and REPLACE-trigger quirks pinned; dml constraint-order observability carded.
- Upsert: PASS (documented=82). COVERAGE regenerated.

## Batch 7 — storage stack + platform primitives (13 cards)

- pager(2), wal(2), pcache(2), vfs-os-abstraction(3), malloc(2), mutex(1), util-primitives(1) → documented, observed-in-code.
- vfs-os-abstraction-003 card explicitly delegates deep contracts to run-2 vfs-win/vfs-kv cards (umbrella kept for continuity, extra line evidence added).
- Crash-safety blind spot restated on pager-001 (harness out of scope — honest limitation, not silently ignored).
- Upsert: PASS (documented=95). COVERAGE regenerated.

## Batch 8 — vtab core, adapters, FTS (10 cards)

- vtab-core(2), introspection-vtabs(1), shell-cli(2), tcl-binding(1), fts5(3), fts3(1) → documented, observed-in-code.
- Compile-gated trio card notes none of the three gates are in the default baseline; shell dot-command surface stays dispatcher-pinned per cluster-at-bind note.
- Upsert: PASS (documented=105). COVERAGE regenerated.

## Batch 9 — extension subsystems (12 cards)

- rtree(2), geopoly(1), session(3), rbu(2), recover(2), intck(1), expert(1) → documented, observed-in-code.
- Session gate (ENABLE_SESSION+PREUPDATE_HOOK) documented vs baseline; float32 R-tree rounding and changeset-format contracts pinned.
- Upsert: PASS (documented=117). COVERAGE regenerated.

## Batch 10 — bindings + umbrella clusters (8 cards)

- icu(1), qrf(1) → documented, observed-in-code.
- jni-binding(1), wasm-binding(1) → documented with confidence **inferred** (file/macro-level evidence; internals not traced — honest per charter, not fake observed-in-code).
- misc-vfs-shims / misc-func-packs / misc-vtab-packs umbrella cards delegate to their thin run-2 cards; misc-vfs-shims card explicitly EXCLUDES the 3 unscanned members (mmapwarm/memtrace/pcachetrace stay hints — no cards invented).
- misc-utilities stays a census card (never unbundled); fileio/eval security powers flagged.
- Upsert: PASS (documented=125). COVERAGE regenerated.

## Batch 11 — run-2 core: compile options + platform VFS (10 cards)

- compile-options-omit-enable(3) — 002/003 stay census cards per charter (no 128-flag explosion); 001 is the baseline oracle.
- vfs-win(2), vfs-kv(2), vfs-unix-variants(3 — lock-style matrix stays census) → documented, observed-in-code.
- Upsert: PASS (documented=135). COVERAGE regenerated.

## Batch 12 — run-2 core: init/config + wasm/jni layers (11 cards)

- global-init-config(3) → documented, observed-in-code (db_config toggle inventory carded — the highest-value SME follow-up).
- wasm-js-api(3), wasm-opfs(2), jni-java-surface(3) → documented with confidence **inferred** (JS/Java layers evidenced at file/class level, not traced/executed — honest labelling per charter).
- Upsert: PASS (documented=146). COVERAGE regenerated.

## Batch 13 — thin misc vtab cards A (13 cards)

- misc-series/csv/zipfile-sqlar/unionvtab/qpvtab/completion/closure/amatch/fuzzer/prefixes/wholenumber/stmt/templatevtab → documented, observed-in-code.
- Upsert: PASS (documented=159). COVERAGE regenerated.

## Batch 14 — thin misc cards B (13 cards)

- misc-vtablog/vtshim/btreeinfo/zorder/basexx/sha1/shathree/decimal/ieee754/percentile/totype/uint/regexp → documented, observed-in-code.
- Type asymmetry (sha1 hex TEXT vs sha3 BLOB) and strict-conversion contracts pinned; regexp non-PCRE dialect carded.
- Upsert: PASS (documented=172). COVERAGE regenerated.
