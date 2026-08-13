# engine-harvest39-001 — per-statement STMT/PROFILE trace events

Confidence: observed-in-code (composed slice, run 49).
multi-statement exec fires one STMT + one PROFILE per prepared statement with the SQL C reports (terminator kept); prepared statements fire from step (sqlite3_sql on the PROFILE handle).
Frozen scope = run-49 pinned cases (pack v39).
