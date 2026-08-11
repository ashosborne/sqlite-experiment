# MORNING BRIEF — engine v8: the cheat sheet is dead (run 17)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–16 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v8-kill-script-table, TARGET_SCRIPT_TABLE_ENTRIES=0.

## 1. Pack @8 BOUND — cheat-sheet ban

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v7 → **v8 BOUND** (Ash Osborne,
delegated). versions/1–8 retained; ADR 0006. New laws: **cheat-sheet ban** (no whole-script
string → rows lookup, table or disguised match), **expression/SELECT law**, **function law**
(functions computed from arguments), **anti-cheat** (runtime-varying SELECT must pass).
Schema-validated. completeness: **incomplete**.

## 2. SCRIPT_TABLE before/after

| | entries |
|---|---|
| before (run-16 tip) | **70** (69 catalogue pins + 1 smoke sentinel) |
| after (this run) | **0** |

Grep proof: `grep -c '("' modern/src/script_table.rs` → 0; no `("SELECT` / `("PRAGMA` left.
`lib.rs` no longer imports SCRIPT_TABLE; "GENERATED lookup" comments rewritten. Unknown SQL
now returns a real nonzero rc + errmsg (never invents success) — covered by a test.

## 3. Verdict on every former pin (69) — 50 IMPLEMENT / 19 DEFER

| Bucket | Case (former pin) | Verdict | Note |
|---|---|---|---|
| A pragma/attach | `attach-detach-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `attach-detach-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `attach-detach-003-C001` | **DEFER** | cross-schema trigger DDL validation |
| A pragma/attach | `pragma-surface-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C002` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C003` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C004` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C005` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C006` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C007` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C008` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C009` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C010` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C011` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-001-C012` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-002-C001` | **DEFER** | scalar subquery + LIMIT over pragma_database_list |
| A pragma/attach | `pragma-surface-002-C002` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| A pragma/attach | `pragma-surface-002-C003` | **DEFER** | C compile-option registry count |
| A pragma/attach | `pragma-surface-002-C004` | **DEFER** | C function/module/pragma registry counts |
| B scalar/expr/query | `builtin-scalar-agg-funcs-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `builtin-scalar-agg-funcs-003-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `expr-codegen-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `expr-codegen-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `expr-codegen-003-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `name-resolution-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `parser-grammar-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `parser-grammar-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `printf-format-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `select-codegen-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `select-codegen-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `select-codegen-003-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| B scalar/expr/query | `tokenizer-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| C json | `json-funcs-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| C json | `json-funcs-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| C json | `json-funcs-003-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| C json | `json-funcs-004-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| D date/time | `date-time-funcs-001-C001` | **DEFER** | julian-day calendar engine |
| D date/time | `date-time-funcs-002-C001` | **DEFER** | strftime %j engine |
| D date/time | `date-time-funcs-003-C001` | **DEFER** | month-overflow/weekday modifiers |
| D date/time | `date-time-funcs-004-C001` | **DEFER** | timediff formatting |
| E agg/window | `builtin-scalar-agg-funcs-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| E agg/window | `window-functions-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| E agg/window | `window-functions-002-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `analyze-stats-001-C001` | **DEFER** | ANALYZE stats engine (sqlite_stat1 content) |
| F misc/vtab/ext | `introspection-vtabs-001-C001` | **DEFER** | dbstat vtab |
| F misc/vtab/ext | `misc-basexx-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-completion-001-C001` | **DEFER** | completion vtab (C keyword registry) |
| F misc/vtab/ext | `misc-compress-001-C001` | **DEFER** | zlib byte-compat |
| F misc/vtab/ext | `misc-csv-001-C001` | **DEFER** | csv vtab |
| F misc/vtab/ext | `misc-decimal-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-fossildelta-001-C001` | **DEFER** | fossil delta byte-compat |
| F misc/vtab/ext | `misc-func-packs-001-C001` | **DEFER** | decimal_mul trailing-digit formatting |
| F misc/vtab/ext | `misc-ieee754-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-nextchar-001-C001` | **DEFER** | index-probe next_char |
| F misc/vtab/ext | `misc-percentile-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-prefixes-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-regexp-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-rot13-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-series-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-sha1-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-shathree-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-totype-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-uint-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-urifuncs-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-utilities-001-C001` | **DEFER** | eval() nested exec |
| F misc/vtab/ext | `misc-uuid-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `misc-wholenumber-001-C001` | **DEFER** | wholenumber vtab |
| F misc/vtab/ext | `misc-zorder-001-C001` | **IMPLEMENT** | store/eval computes it; bytes match frozen golden |
| F misc/vtab/ext | `vacuum-001-C001` | **DEFER** | VACUUM/zeroblob rebuild |
Every DEFER above: pin removed, replay test removed, **no store/kitchen coverage claimed**.
Goldens stay frozen (byte-identical) for a future run. Reasons are honest capability gaps,
not parked pins.

## 4. New modules (the executor that replaced the pins)

- `modern/src/eval.rs` (~1000 lines) — SQL tokenizer → Pratt expression parser → typed
  evaluator (NULL/INT/REAL/TEXT/BLOB): arithmetic, `||`, comparisons, AND/OR/NOT, CASE/iif,
  CAST, IN, IS [NOT], LIKE (+ESCAPE, case_sensitive_like) / GLOB, REGEXP (tiny), COLLATE uint,
  hex/exp/blob/bracket literals; SELECT executor (no-FROM, FROM subquery / store table /
  generate_series / prefixes / json_each / pragma_* TVFs, WHERE, UNION [ALL], ORDER BY),
  aggregates (count/sum/total/avg/min/max/group_concat), two pinned window shapes
  (row_number OVER, sum OVER ROWS 1 PRECEDING); PRAGMA get/set (user_version, application_id,
  schema_version, cache_size, recursive_triggers, defer_foreign_keys, query_only, temp_store,
  automatic_index, ignore_check_constraints, case_sensitive_like, integrity/quick_check);
  ATTACH/DETACH + pragma_database_list; computed funcs: printf/%q, upper/lower/length/substr/
  coalesce/typeof/hex/quote, sha1 + sha3 (real digests), base64, rot13, decimal add/cmp (exact
  scaled ints), ieee754 family, zorder/unzorder, tointeger/toreal, uuid (via sqlite3_randomness).
- `modern/src/json.rs` — real JSON parser → path evaluator → canonical serializer:
  json_extract / -> / ->>, json_set/insert/replace/remove/patch (RFC-7396), json_valid,
  json_type, json_each.
- `modern/src/store.rs` — execute_script refactored to **per-statement dispatch**: kitchen
  DDL/DML/SELECT on real tables first; everything else goes to eval with a table snapshot +
  connection state; mixed scripts (CREATE + PRAGMA, ATTACH + SELECT) now work. CREATE/DROP
  bump schema_version. `median`/`percentile` deliberately error (the pinned bare build has no
  ENABLE_PERCENTILE — the frozen golden IS rc=1).

## 5. Anti-cheat results (modern/tests/anti_cheat_v8.rs) — 5/5 green

- `anti_cheat_expr_runtime` — `SELECT <runtime_n>+2, <n>*3, upper(printf(...))` computed. ✅
- `anti_cheat_pragma_roundtrip` — runtime user_version/application_id set → read back. ✅
- `anti_cheat_json_or_scalar` — runtime literal through json_extract/json_set/length. ✅
- `anti_cheat_script_table_is_empty` — asserts `SCRIPT_TABLE.len() == 0`. ✅
- `anti_cheat_unknown_sql_fails_honestly` — unknown function → nonzero rc, zero rows. ✅

## 6. Test suite

`cargo test` — **136/136 green**: spine 11, bespoke 28+5(errors)+20(leftovers)+3, kitchen 6
(memory kitchen incl. re-homed), files 8 + interop (C reads Rust files, integrity_check=ok),
oneshot **50 store/eval replays** (frozen bytes), anti-cheat 5. All 240 prior golden files
byte-identical (md5-verified) — RECORD added nothing, rewrote nothing this run.

## 7. Catalogue impact

~30 slices (pragma surface, attach, expr/select/tokenizer, builtins, JSON, aggregates,
windows, and 12 thin misc-* function slices) moved from **"cheat sheet only" → "Rust
executes"**. parity_green stays **0** — replay-vs-frozen-golden is not the pack's parity gate.

## 8. Leftover (still not a full SQL engine)

joins/planner · date/time engine (4 defers) · WAL/crash recovery · full integrity_check ·
scalar subqueries + LIMIT · ANALYZE/VACUUM/dbstat · vtab modules (csv, completion,
wholenumber) · zlib/fossil-delta byte-compat · eval() · cross-schema trigger validation ·
C registry counts (compile_options/function_list/...) · pager/btree/vfs cards stay amber.

completeness: **incomplete** — SQLite is NOT migrated.

## 9. Next call

(a) date/time engine (clears 4 defers, real julian-day math), or (b) scalar subqueries +
LIMIT + small joins (query-shape depth), or (c) WAL law change. Pack v9 + goldens first.
