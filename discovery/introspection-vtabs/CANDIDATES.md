# CANDIDATES — introspection-vtabs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Introspection vtab trio (dbstat page stats; sqlite_dbpage raw page r/w; bytecode/tables_used) | `sqlite3DbstatRegister` `src/dbstat.c:874`, `sqlite3DbpageRegister` `src/dbpage.c:473`, `sqlite3VdbeBytecodeVtabInit` `src/vdbevtab.c:436` (each compile-gated, no-op stubs at `dbstat.c:905`/`dbpage.c:504`/`vdbevtab.c:445`) | SQL-queryable internals; dbpage is write-capable (repair/forensics) |
