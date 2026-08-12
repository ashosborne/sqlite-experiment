# engine-vtab31-003 — SELECT through the module cursor

Confidence: observed-in-code (composed slice, run 41).
full scan via xOpen/xBestIndex(0-constraint)/xFilter/xEof/xColumn/xNext/xClose; WHERE/aggregates/ORDER BY/JOIN/subselects over the scan; HIDDEN-column WHERE via xColumn.
Frozen scope = run-41 pinned cases (pack v31).
