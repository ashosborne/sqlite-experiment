# tcl-binding-001 — TCL package init and sqlite3 command dispatch

Slice: `tcl-binding` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Package registration creates 'sqlite3' command; each db handle becomes a TCL command with ~40 subcommands.

## Entrypoints (citations)

- `Sqlite3_Init()` (other) — `src/tclsqlite.c:4469`, `src/tclsqlite.c:4483`, `src/tclsqlite.c:4503`

## Inputs / outputs / observables

- TCL: sqlite3 db file → db command with ~40 subcommands (eval/onecolumn/exists/transaction/function/collate/busy/trace/backup/restore/nullvalue...); TCL error propagation

## Behaviour (as implemented)

- Sqlite3_Init (src/tclsqlite.c:4469) registers the package + 'sqlite3' command; each connection becomes a TCL command object (DbObjCmd dispatch, same file); db eval binds TCL variables as parameters ($var), returns rows as lists or runs scripts per row; transaction subcommand wraps script in BEGIN/COMMIT with error rollback

## Validation rules found in code

- TCL-level argument validation with usage messages

## Edge cases found in code

- Nested transaction subcommand uses savepoints; db function creates UDFs backed by TCL scripts (marshalling costs)

## Dependencies

- connection-lifecycle-api

## Assumptions / unknowns

- Product-vs-test-only question from run 1 stands — bound by ACCEPT_ALL policy
- Is the TCL binding a product surface downstream, or test-only?

## Evidence

- `src/tclsqlite.c:4469`
- `src/tclsqlite.c:4483`
- `src/tclsqlite.c:4503`
