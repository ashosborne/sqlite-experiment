# SQLite SQL-only allowlist

- **Git ref:** `sqlite/sqlite@master` (`2665df90e7bbdcffa59102204090f7c9ede04cc4`)
- **VERSION file:** `3.54.0` (3.54-era; 3.54.0 is trunk / unreleased as of 2026-08-13)
- **Fetched from:** `https://raw.githubusercontent.com/sqlite/sqlite/master/test/<file>.test`
- **Cases kept:** 3375
- **Cases dropped (reason histogram):** 891
- **Files skipped whole:** 3
- **Kinds:** {'execsql': 2840, 'catchsql': 535}
- **Roles:** {'test': 3327, 'setup': 48}

## Sequential execution

SQLite `testfixture` shares one connection per `.test` file. Cases in `cases.jsonl`
are ordered (`seq`) per `source_file`. The Rust harness **must** open `:memory:`
once per file and run cases in `seq` order. Do not shuffle. Do not reset between
cases in the same file. Reset (new `:memory:`) between files.

Top-level `execsql { ... }` with no expected result is emitted as `role=setup`
(`kind=execsql`, empty `expected`). Run it; do not treat empty expected as a row check
failure — setup SQL is allowed to return no rows.

## Files kept

| file | kept | dropped | seen do_test/execsql | notes |
| --- | ---: | ---: | ---: | --- |
| `alter.test` | 58 | 62 | 112 | ratio=0.513 |
| `analyze.test` | 39 | 5 | 44 | ratio=0.886 |
| `between.test` | 4 | 13 | 15 | ratio=0.267 |
| `cast.test` | 127 | 3 | 129 | ratio=0.977 |
| `check.test` | 99 | 8 | 106 | ratio=0.934 |
| `coalesce.test` | 8 | 1 | 9 | ratio=0.889 |
| `collate1.test` | 58 | 2 | 58 | ratio=1.0 |
| `count.test` | 15 | 13 | 26 | ratio=0.577 |
| `delete.test` | 44 | 18 | 61 | ratio=0.71 |
| `distinct.test` | 20 | 10 | 23 | ratio=0.87 |
| `expr.test` | 43 | 6 | 37 | ratio=1.0 |
| `func.test` | 232 | 46 | 265 | ratio=0.875 |
| `having.test` | 9 | 6 | 9 | ratio=1.0 |
| `in.test` | 114 | 9 | 123 | ratio=0.927 |
| `index.test` | 90 | 16 | 104 | ratio=0.865 |
| `insert.test` | 69 | 7 | 76 | ratio=0.908 |
| `intpkey.test` | 81 | 24 | 104 | ratio=0.779 |
| `join.test` | 150 | 32 | 181 | ratio=0.829 |
| `join2.test` | 47 | 0 | 47 | ratio=1.0 |
| `json101.test` | 251 | 3 | 251 | ratio=1.0 |
| `json102.test` | 199 | 4 | 201 | ratio=0.99 |
| `like.test` | 40 | 124 | 160 | ratio=0.248 |
| `limit.test` | 118 | 7 | 122 | ratio=0.952 |
| `null.test` | 41 | 0 | 41 | ratio=1.0 |
| `orderby1.test` | 37 | 25 | 62 | ratio=0.597 |
| `select1.test` | 132 | 68 | 192 | ratio=0.663 |
| `select3.test` | 50 | 3 | 51 | ratio=0.98 |
| `select5.test` | 36 | 1 | 34 | ratio=1.0 |
| `subquery.test` | 83 | 10 | 90 | ratio=0.892 |
| `trigger1.test` | 82 | 12 | 89 | ratio=0.872 |
| `trigger2.test` | 52 | 4 | 36 | ratio=0.981 |
| `unionall.test` | 45 | 0 | 45 | ratio=1.0 |
| `unique.test` | 35 | 0 | 35 | ratio=1.0 |
| `update.test` | 133 | 6 | 138 | ratio=0.957 |
| `upsert1.test` | 32 | 3 | 35 | ratio=0.914 |
| `values.test` | 68 | 3 | 69 | ratio=0.986 |
| `view.test` | 93 | 29 | 122 | ratio=0.762 |
| `where.test` | 61 | 211 | 269 | ratio=0.227 |
| `where2.test` | 46 | 64 | 106 | ratio=0.43 |
| `window1.test` | 285 | 11 | 287 | ratio=0.993 |
| `with1.test` | 97 | 10 | 103 | ratio=0.942 |
| `without_rowid1.test` | 52 | 9 | 58 | ratio=0.897 |

## Drop reasons (all files, including skipped)

| reason | count |
| --- | ---: |
| `tcl_do_test_body` | 724 |
| `tcl_loop_or_proc` | 87 |
| `regex_expected` | 47 |
| `pragma_integrity_check` | 18 |
| `file_skipped_tcl_machinery` | 3 |
| `tcl_interpolated_setup` | 3 |
| `vdbe` | 3 |
| `rtree` | 2 |
| `tcl_interpolated_sql` | 2 |
| `fts` | 1 |
| `corrupt` | 1 |

## Files requested but 404 on master

These names are **not** in `sqlite/sqlite` `test/` on this ref (confirmed HTTP 404):

| requested | why missing | stand-in fetched? |
| --- | --- | --- |
| `aggregate.test` | no such file; aggregates live in `count.test`, `minmax*.test`, `aggnested.test`, `distinctagg.test` | extra: `count.test` |
| `upsert.test` | numbered `upsert1.test` … `upsert5.test` | extra: `upsert1.test` |
| `pk.test` | no such file; integer PK is `intpkey.test` / `rowid.test` | extra: `intpkey.test` |
| `json1.test` | JSON tests are `json101.test` … `json109.test` (plus `test/json/`) | extra: `json101.test` |
| `cte1.test` | CTEs are `with1.test` … `with6.test` | allowlist already has `with1.test` |
| `union.test` | compound UNION ALL is `unionall.test`; UNION/EXCEPT/INTERSECT also in `select5.test` | extra: `unionall.test`, `select5.test` |
| `except.test` | no standalone file | extra: `select5.test` |
| `intersect.test` | no standalone file | extra: `select5.test` |
| `glob.test` | GLOB lives in `like.test` / `like2.test` / `like3.test` | allowlist `like.test` |
| `case.test` | CASE lives in `expr.test` | allowlist `expr.test` |

## Extra files (up to 10, from fossil `test/` listing)

Listed `https://www.sqlite.org/src/dir?name=test&ci=trunk` (1194 `*.test` files).
Picked SQL-looking names, **not** explain/pager/wal/malloc/fts/rtree/thread/corrupt/lock/vfs:

- `having.test`
- `exists.test`
- `count.test` (stand-in for missing `aggregate.test`)
- `upsert1.test` (stand-in for missing `upsert.test`)
- `unionall.test` (stand-in for missing `union.test`)
- `json101.test` (stand-in for missing `json1.test`)
- `check.test`
- `select5.test` (compound SELECT; UNION/EXCEPT/INTERSECT)
- `func.test`
- `intpkey.test` (stand-in for missing `pk.test`)

Did **not** invent filenames. Directory listing saved as `github-test-listing.txt`.

## Extractor limits (honest)

- Brace-quoted SQL only. Double-quoted SQL with `$tcl` interpolation is dropped.
- `do_test` bodies that mix Tcl (`set`, `for`, `catch` except the catchsql idiom) are dropped.
- `execsql2` (name/value pairs) is not extracted — different expected shape.
- `ifcapable` / `if` bodies are harvested without evaluating the capability; some cases
  need compile options the kitchen may not have. Record those as known-fail, do not skip silently.
- Mid-file dropped cases mean later cases in that file may see different state than testfixture.
  Prefer first-slice files with high keep ratio (`values.test`, `json101.test`, `json102.test`,
  `window1.test`, `with1.test`, `upsert1.test`, `unionall.test`, `without_rowid1.test`).

