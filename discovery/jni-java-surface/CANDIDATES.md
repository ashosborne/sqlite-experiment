# CANDIDATES — jni-java-surface (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Refines run-1 jni-binding-001 (row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | capi 1:1 Java layer + callback interfaces | `ext/jni/src/org/sqlite/jni/capi/CApi.java` + ~40 callback/interface classes (AuthorizerCallback, BusyHandlerCallback, CommitHookCallback, ...) | Direct Java projection of the C API |
| 002 | wrapper1 OO Java layer | `ext/jni/src/org/sqlite/jni/wrapper1/Sqlite.java`, `SqliteException.java`, function classes (ScalarFunction, AggregateFunction, WindowFunction) | Idiomatic higher-level Java API |
| 003 | FTS5 Java bindings | `ext/jni/src/org/sqlite/jni/fts5/` (fts5_api.java, Fts5ExtensionApi.java, ...) | Java surface for FTS5 aux-function extension points |
