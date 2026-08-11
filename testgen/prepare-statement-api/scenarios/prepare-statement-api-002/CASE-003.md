# prepare-statement-api-002-C003 — BLOCKED — UAF, do not RECORD

Feature: `prepare-statement-api-002` · Kind: characterization · Status: **BLOCKED (permanent)**
Evidence gate: blocked · Citations: card Validation section (patched run 6: "NOT observed and not
safely observable"), `src/vdbeapi.c:980`

## Why this case is blocked

Stepping a statement handle after `sqlite3_finalize()` is **use-after-free**. This holds even when
`SQLITE_ENABLE_API_ARMOR` is compiled in — armor guards NULL pointers, not freed memory. There is
no safe way to capture this observable; undefined behaviour is never executed, recorded, or frozen.

- The harness contains no code path that touches a handle after finalize (call removed in run 5).
- No golden exists; `golden_path: null` in TRACEABILITY.
- Any future pin of the *defined* guarded path (`sqlite3_step(NULL)`) would be a **new case ID
  (C004)** requiring its own operator approval — it must not be recorded under this ID.
