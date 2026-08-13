# sqlite-sql-suite — SQL-only regression harness (run 54)

Confidence: observed-in-code (test infrastructure, not a behavioural pin).

A vendored SQL-only extract of the upstream SQLite TCL suite
(`do_execsql_test` / `do_catchsql_test` / SQL-only `do_test { execsql {…} }`)
replayed through the kitchen's existing `sqlite3_exec` C ABI. It builds **no**
behavioural pins, fills **no** `SCRIPT_TABLE` entries, and flips **no**
APP_MANIFEST card. It is NOT a migration claim.

- **Pack:** `tests/sqlite-sql-suite/cases.jsonl` (3375 cases),
  `ALLOWLIST.md`, `SKIP.md` — extracted from
  `sqlite/sqlite@2665df90e7bbdcffa59102204090f7c9ede04cc4`, VERSION 3.54.0.
- **Harness:** `modern/tests/sqlite_sql_suite.rs`. Opens `:memory:` per source
  file, runs each case's SQL through `sqlite3_exec`, flattens result cells the
  way Tcl's `$db eval` + `[list {*}$result]` does (NULL → `{}`), and compares to
  the extracted `expected`. `catchsql` compares rc always and errmsg when it
  matches C's wording.
- **Known-fail:** `tests/sqlite-sql-suite/known-fail.txt` — one id per line with
  a reason. Listed cases still run; a listed case that now passes prints XPASS.
  A mismatch NOT listed is a harness failure (the test fails).

## Run-54 first slice

Files: `values.test`, `unique.test`, `coalesce.test`, `upsert1.test`,
`null.test` (184 cases). Result: **103 pass / 0 unexpected fail / 81 known-fail
/ 0 xpass**. The 81 known-fails are honest kitchen gaps, not silent skips —
categories: parser (unsupported statement forms), upsert/ON CONFLICT semantics,
missing constraint validation, NULL/row-text divergence, catchsql errmsg
wording. See `known-fail.txt` for the per-id reason.

Scope is the first slice by default; override with
`SQLITE_SQL_SUITE_SLICE=a.test,b.test`.
