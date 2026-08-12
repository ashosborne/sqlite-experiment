# engine-vacuum-001 — VACUUM rebuild basics

Confidence: observed-in-code (composed slice, run 34).
Empty-db no-op; rows/tables/indexes survive; "cannot VACUUM from within a
transaction" (txn continues); page_count shrinks after churn; WITHOUT ROWID
survives; INTEGER PRIMARY KEY keys preserved; implicit rowids renumbered
1,3,5 -> 1,2,3 by the real rebuild.
Frozen scope = run-34 pinned cases (pack v24).
