# engine-conn-001 — close / close_v2 handle tracking

Confidence: observed-in-code (composed slice, run 36).
Clean close OK; NULL no-ops; unfinalized statements and open blob handles block
close (rc 5, exact C errmsg; reset does not unblock); close_v2 zombies with the
statement still usable and tears down at the last finalize; closing with an open
transaction rolls back (file reopen pinned).
Frozen scope = run-36 pinned cases (pack v26).
