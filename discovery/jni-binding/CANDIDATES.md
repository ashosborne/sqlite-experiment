# CANDIDATES — jni-binding (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | JNI adapter layer | `ext/jni/src/c/sqlite3-jni.c` (JNIEXPORT bridge macros, e.g. `:179`, `:5443`; bridges generated via macro families) + generated header `sqlite3-jni.h` | Full Java binding of the C API incl. callback marshalling |
