# engine-btree45-002 — table cursor cells on pager pages

Confidence: observed-in-code (composed slice, run 56).
file-backed INSERT/SELECT-by-rowid/DELETE/literal-UPDATE of a single-leaf rowid table move cells on the pager leaf; committed file C-readable, integrity_check ok.
Frozen scope = run-56 pinned cases (pack v45). File-backed, DELETE journal mode, single-leaf rowid table.
