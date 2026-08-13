# What this pack will not run

Official SQLite tests are TCL scripts run by `./testfixture file.test`, linked to C sqlite3.
This pack is **SQL row/error checks only**. Everything below is out of scope for the Rust kitchen
(C ABI, no VDBE / btree / pager / TCL).

## Whole files skipped (almost all TCL machinery)

| file | reason |
| --- | --- |
| `aggerror.test` | tcl_udf_aggregate |
| `exists.test` | tcl_lock_fixture |
| `select2.test` | tcl_for_loop_dataload |

## Classes we will not run (even if a `.test` file exists upstream)

| class | why | examples in upstream `test/` |
| --- | --- | --- |
| Bytecode / EXPLAIN / EQP | kitchen has no VDBE; EXPLAIN is parked (`prepare-statement-api-006`) | `eqp.test`, `eqp2.test`, `do_eqp_test`, `explain_i` |
| Pager / btree | no pager/btree | `pager*.test`, `btree01.test`, `btree02.test` |
| WAL multi / WAL protocol | parked; kitchen is not WAL-multi | `wal*.test`, `e_wal*.test` |
| mmap | VFS/mmap | `mmap*.test`, `bigmmap.test` |
| FTS | extension, not kitchen SQL | `fts3*.test`, `fts4*.test` |
| rtree | extension | `rtree.test` |
| Threads / locks / busy / crash / corrupt | TCL fixture + pager | `thread*.test`, `lock*.test`, `busy*.test`, `crash*.test`, `corrupt*.test` |
| malloc / OOM / progress | allocator/hook TCL | `malloc*.test`, `progress.test` |
| VFS / URI / load_extension | dlopen + VFS | `avfs.test`, `uri.test`, `loadext.test` |
| sqlite_dbpage / dbstat / sqlite_stmt / bytecode vtab | virtual tables we are not claiming | `dbpage.test`, `stat.test`, `stmt.test` |
| PRAGMA integrity_check | pager-level | many files call `integrity_check` |
| VACUUM INTO internals | parked vacuum-into | `vacuum-into.test` |
| TCL fixture | `source tester.tcl`, `do_test` scripts, `sqlite3_create_*`, file-size tricks | `tester.tcl` itself |
| C API bind/prepare/step tests | not `sqlite3_exec` SQL | `bind.test`, `capi*.test` |

## Per-case drops inside otherwise-kept files

See `ALLOWLIST.md` histogram. Typical: `tcl_do_test_body` (script is not pure SQL),
`explain` (EXPLAIN QUERY PLAN inside SQL), `vdbe` (`PRAGMA vdbe_listing`),
`tcl_interpolated_sql` (double-quoted SQL with `$vars`), `other_db_handle` (`-db`).

## 404s (requested names that do not exist on this ref)

`aggregate.test`, `upsert.test`, `pk.test`, `json1.test`, `cte1.test`,
`union.test`, `except.test`, `intersect.test`, `glob.test`, `case.test`.

Not invented. See ALLOWLIST for stand-ins actually fetched.

