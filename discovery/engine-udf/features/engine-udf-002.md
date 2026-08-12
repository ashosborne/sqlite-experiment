# engine-udf-002 — Application-registered SQL functions

Confidence: observed-in-code (composed slice, run 28).
sqlite3_create_function[_v2] scalar (xFunc) + aggregate (xStep/xFinal) invoked from SQL; overwrite/delete/xDestroy; user_data.
Frozen scope = run-28 pinned cases; real C-callback invocation (pack v18).
