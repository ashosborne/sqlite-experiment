# engine-overflow-001 — Overflow page chains (durable large payloads)

Confidence: observed-in-code (composed slice, run 22).
TEXT/BLOB payloads exceeding one leaf persist via real overflow chains; C integrity_check=ok; exact bytes after reopen.
Frozen scope = the run-22 pinned cases; executed for real by store/dbfile (pack v12).
