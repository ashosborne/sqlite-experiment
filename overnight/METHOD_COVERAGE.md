# Method coverage — estate discovery run (sqlite-experiment)

Checklist per the stock loop, adapted honestly to a C library estate. "Touched" means at least one
Phase A pass produced evidence-cited candidates for that method family this run.

| Method family (stock checklist) | Status this pass | Notes / evidence |
| --- | --- | --- |
| HTTP/REST (and SOAP) | **Not present in estate (observed)** | No HTTP listener, server socket, or route table anywhere under `src/`/`ext/`. Closest analogue = public C API (`src/sqlite.h.in`), covered by 11 API-boundary slices. |
| Message listeners / queues / topics | **Not present in estate (observed)** | No message-bus consumers. Closest analogues: unlock-notify callback (`unlock-notify-api`), commit/update hooks (`connection-lifecycle-api-004`), session change-tracking (`session`). All touched. |
| Schedulers / batch / CLI mains | **Touched** | CLI mains inventoried in STRUCTURAL_INDEX §3: sqlite3 shell (slice `shell-cli`), changeset/changesetfuzz, expert CLI, rbu CLI, dbdump standalone. No cron/scheduler constructs exist (observed). |
| Gateway / RAML / OpenAPI specs | **Touched (analogue)** | No OAS. Contract authority = `src/sqlite.h.in` (public API doc-comments) — cited across slices as the spec analogue. |
| UI-only adapters | **Noted, deprioritized** | `ext/wasm` fiddle app (browser demo UI). Recorded as adapter candidate inside `wasm-binding`; not prioritized per loop rules. No other UI exists. |

## Estate-specific families this loop added (for honesty)

| Family | Status | Slices |
| --- | --- | --- |
| Public C API boundaries | Touched | connection-lifecycle, prepare-statement, exec-convenience, backup, blob-io, serialize-memdb, loadext, unlock-notify, auth-callback, error-status, vtab-core |
| SQL-language surfaces | Touched | builtin-scalar-agg-funcs, date-time-funcs, json-funcs, printf-format, pragma-surface, window-functions, upsert, triggers, foreign-keys, ddl-schema, analyze-stats, vacuum, attach-detach |
| Compiler pipeline seams | Touched | tokenizer, parser-grammar, name-resolution, expr-codegen, select-codegen, dml-codegen, where-optimizer |
| Engine/storage seams | Touched | vdbe-engine, btree, pager, wal, pcache, vfs-os-abstraction, malloc-subsystem, mutex-subsystem, util-primitives |
| Language bindings | Touched | tcl-binding, jni-binding, wasm-binding |
| Extension subsystems | Touched | fts5, fts3, rtree, geopoly, session, rbu, recover, intck, expert, icu, qrf + 4 ext/misc clusters |

## What was NOT searched / could not be seen (first-class residuals)

1. **Compile-time option matrix** — `SQLITE_OMIT_*` / `SQLITE_ENABLE_*` gate whole surfaces on/off.
   This pass read the default-visible code; no per-configuration scan was performed.
2. **Fault-injection / crash-recovery behaviour** — the upstream harness for this lives under `test/`
   and `src/test_*.c`, both outside the effective scan scope (charter marks `test/` out of scope;
   `src/test_*.c` are harness adapters, skipped-with-reason). Pager/WAL crash semantics are therefore
   candidates without characterization evidence paths yet.
3. **Generated-code surfaces** — `parse.c`, `opcodes.*`, `keywordhash.h`, `pragma.h`, amalgamation
   `sqlite3.c` are not in the tree; behaviour that only manifests in generated output (e.g. full
   pragma inventory from `tool/mkpragmatab.tcl`, out-of-scope path) was cited via source inputs only.
4. **Platform-specific paths** — `os_win.c` and `os_kv.c` were inventoried but not deep-scanned;
   VxWorks/proxy-locking variants inside `os_unix.c` untouched.
5. **ext/wasm JS layer breadth** — 40 files; only the API-glue seam was evidenced. Worker/promise
   APIs, OPFS sync-access-handle pool variants not individually carded.
6. **ext/jni Java tree** — Java-side classes not scanned (C bridge only).
7. **Runtime-only behaviour** — anything visible only under load (contention, cache pressure,
   memory-pressure callbacks) has no static evidence path.
8. **Out-of-scope trees by charter** — `art/ autoconf/ autosetup/ doc/ tool/ mptest/ test/ .fossil-settings/` — never seeded.

**Conclusion:** checklist addressed for every family that exists in this estate; absent families
recorded as observed-absent rather than silently skipped. `estate_scan=partial` note retained in
APP_MANIFEST — zero-diff re-scan would still not equal completeness (residuals 1–7 above).

---

# Resume run 2 addendum — residual hunt (2026-08-10)

This run walked run-1 residuals 1, 4, 5, 6 and the global-init-config open question, plus unbundled
the three ext/misc umbrella clusters into 42 planned thin slices (39 scanned before the 120-delta cap).

## Residuals walked this run

| Run-1 residual | Now | Slices |
| --- | --- | --- |
| 1. Compile-time option matrix | **Carded** (diagnostics API + OMIT/ENABLE censuses; per-option carding deferred until baseline build pinned) | compile-options-omit-enable |
| 4. Platform variants | **Carded** (win32 incl. shm/mmap, kvvfs incl. pluggable K/V methods, unix locking-style matrix/proxy/VxWorks) | vfs-win, vfs-kv, vfs-unix-variants |
| 5. ext/wasm JS breadth | **Carded** (capi projection, oo1, worker1/promiser, OPFS async-proxy + sahpool) | wasm-js-api, wasm-opfs |
| 6. ext/jni Java tree | **Carded** (capi Java layer, wrapper1, fts5 bindings) | jni-java-surface |
| (open question) global init/config | **Carded** | global-init-config |

## Still-open residuals (unchanged from run 1, restated honestly)

2. **Crash/fault-injection behaviour** — harness remains under `test/`/`src/test_*.c` (out of scope by charter; not an oracle). Pager/WAL crash semantics still evidence-thin.
3. **Generated-code surfaces** — unchanged; cited via source inputs only.
7. **Runtime-only behaviour** (contention, memory pressure) — no static seam; per charter NOT minted as cards.
8. **Out-of-scope trees** — unchanged.

## New residuals from this run

- 3 unscanned thin vfs-shim hints left when the 120-delta cap tripped: `misc-mmapwarm`, `misc-memtrace`, `misc-pcachetrace` (in APP_MANIFEST `unscanned_hints`).
- Per-compile-option surface diffs: carding one slice per OMIT/ENABLE flag only makes sense after a human pins the baseline build config (else 128 speculative cards).
- ext/wasm JS breadth below the API layer (utility namespaces, tester infrastructure) and ext/jni annotation/test packages — noted, deliberately not carded (test/annotation support, not product seams).

**Conclusion:** `estate_scan=partial` remains in APP_MANIFEST notes. Empty or short hint lists after
this run still do not mean complete — residuals 2/3/7 + the baseline-config decision are the honest gap.
