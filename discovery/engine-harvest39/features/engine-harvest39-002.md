# engine-harvest39-002 — pragma_module_list lazy population

Confidence: observed-in-code (composed slice, run 49).
fresh connection lists only its own pragma vtab; sqlite3_create_module names join the list; touched pragma TVFs join lazily (C's contract, ADR 0032 hole closed; no census of untouched TVFs).
Frozen scope = run-49 pinned cases (pack v39).
