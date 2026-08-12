# ADR 0016 — engine v18: UDF registration + value/result (pack v18)

Date: 2026-08-12 · Status: BOUND (supersedes pack v17; v1–v17 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 28)

## Context
Built-ins and misc functions ran for real, but there was no way for an application to
register its own SQL functions — the sqlite3_create_function / sqlite3_value_* /
sqlite3_result_* surface was entirely missing (distinct from loadext's dlopen path).

## Decision — a real cross-language UDF path
A per-connection registry maps (name_lower, nArg) → callbacks
(xFunc | xStep+xFinal, xDestroy, user_data). sqlite3_create_function[_v2] populate it;
all-NULL callbacks delete (running xDestroy); re-registering the same name/arity
overwrites (xDestroy of the replaced entry fires); sqlite3_close runs xDestroy for the
rest. The eval engine, when it meets an unknown function name that IS registered,
builds `Sqlite3Value` boxes for the args, calls the C `xFunc(ctx, argc, argv)`, and
reads the result out of `Sqlite3Context` (or its error). Aggregates reuse the GROUP BY
machinery: one context per group, xStep per row, then xFinal;
sqlite3_aggregate_context hands back a lazily-allocated zeroed per-group buffer.

Value accessors (type / numeric_type / int / int64 / double / text / blob / bytes)
mirror C — including numeric affinity on numeric-looking text and value_bytes = the
rendered text length for numerics; value_text returns raw bytes so blobs round-trip.
Result writers (null / int / int64 / double / text / blob / error) set the context;
non-UTF-8 result_text bytes are carried as a blob so `hex(echo(X'DEAD'))='DEAD'`.
user_data, context_db_handle, get_autocommit exported. The `*` pseudo-column of
count(*) is not mis-evaluated: args are only evaluated once a name is known to be a UDF.

## Consequences
24 cases frozen (two-run gate, delegated stamp), all replay byte-identical; cargo
428/428; anti-cheat: runtime scalar, runtime value-type fingerprint, runtime
aggregate. New cards engine-udf-001 (scalar), engine-udf-002 (aggregate),
engine-value-001 (marshalling) full. loadext-api-001 stays partial — dlopen NOT
implemented (note tightened: in-process create_function is a different surface).
No window UDFs, no create_function16/UTF-16, no api_routines thunk. NOT migrated.
