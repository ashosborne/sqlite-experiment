# engine-conn-003 — commit / update hooks + trace_v2

Confidence: observed-in-code (composed slice, run 36).
update_hook (op/db/table/IPK-aliased rowid, unset semantics, prior-arg return);
commit_hook (fires per committed txn incl. autocommit; non-zero aborts COMMIT
rc 19 with rollback; replacement); trace_v2 STMT text, ROW per row, CLOSE at
teardown, PROFILE counts, mask-0 unset.
Frozen scope = run-36 pinned cases (pack v26).
