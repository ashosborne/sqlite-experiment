# MORNING BRIEF — engine v18: UDF registration + sqlite3_value/result (run 28)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–27 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v18-udf-value. MAX_NEW_CASES 40 (used 24).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @18 BOUND — UDF-registration / value-result / no-dlopen laws

versions/18.yaml + ADR 0016, schema-validated. WAL forbidden; SCRIPT_TABLE stays 0.

## 2. APIs implemented (real cross-language callbacks)

- `sqlite3_create_function` / `_v2` — per-connection registry keyed by (name, nArg);
  xFunc (scalar), xStep+xFinal (aggregate), xDestroy, user_data (pApp). All-NULL
  callbacks delete; re-register overwrites (xDestroy of the replaced entry fires);
  `sqlite3_close` runs xDestroy for the rest. DETERMINISTIC flag accepted (honest no-op).
- The eval engine invokes the **real C callback** for a registered name: builds
  `Sqlite3Value` args, calls `xFunc(ctx, argc, argv)`, reads the result/error from
  `Sqlite3Context`. Aggregates run one context per group (xStep per row → xFinal),
  reusing the GROUP BY machinery; `sqlite3_aggregate_context` gives a lazily-allocated
  zeroed per-group buffer.
- `sqlite3_value_*`: type / numeric_type (numeric affinity on numeric-looking text) /
  int / int64 / double / text (raw bytes, NUL-appended) / blob / bytes (= rendered
  text length for numerics). `sqlite3_result_*`: null / int / int64 / double / text /
  blob / error (non-UTF-8 result_text carried as a blob so `hex(echo(X'DEAD'))='DEAD'`).
  `sqlite3_user_data` / `context_db_handle` / `get_autocommit`.

## 3. Case table: 24 frozen / 0 deferred

engine-udf-001 (10 scalar: register+call, arity error, overwrite, delete-by-NULL,
result-type matrix, result_error, user_data, WHERE+nested-with-builtin, v2 xDestroy
replace→1/close→2, DETERMINISTIC) · engine-value-001 (8: 5-class fingerprint,
empty-vs-abc, numeric_type of text, int64/column-int truncation, blob hex round-trip,
bound-param→UDF, zeroblob(0), variadic nArg=-1 real argc) · engine-udf-002 (6
aggregate: mysum, empty→NULL, NULL-skip, GROUP BY, overwrite-with-scalar, three-row).
Two-run deterministic, delegated HUMAN_ACCEPTED, all replay byte-identical (the Rust
twin registers the same extern "C" callbacks).

## 4. Card flips

- **engine-udf-001 (scalar create_function) → full**
- **engine-udf-002 (aggregate xStep/xFinal) → full**
- **engine-value-001 (value/result marshalling) → full**
- **loadext-api-001 stays partial** (note tightened): shared-library dlopen
  `sqlite3_load_extension` is NOT implemented; in-process create_function is a
  different surface and is now done. No greenwash.

## 5. Anti-cheat + cargo

engine-udf anti-cheats: runtime scalar (`twice(<runtime>)`=2n, unregistered errors),
runtime value-type fingerprint, runtime aggregate over inserted rows — all green,
SCRIPT_TABLE asserted 0. `cargo test` **428/428**; all 390 prior goldens md5-identical.

## 6. Scoreboard before → after

| State | run 27 | **run 28** |
|---|---|---|
| full | 79 | **82** |
| partial | 50 | 50 (loadext note tightened) |
| none | 103 | 103 |
| behaviours | 232 | 235 |

## 7. Explicit honesty line

**SQLite is NOT migrated.** 82 of 235 behaviours done in modern; all parity
UNVERIFIED; parity_green 0; nothing verified. No dlopen, no window UDFs, no UTF-16
create_function16, no api_routines thunk table, no planner, no WAL.

## 8. Next call

(a) UTF-16 text/prepare APIs (prepare-statement-api-001's sole remaining gap +
column UTF-16), (b) index-expression conflict targets to finish upsert-001, or
(c) collation registration (sqlite3_create_collation) as a sibling of the UDF surface.
Pack v19 + goldens first.
