# engine-btree46-001 — cursor-path first split

Confidence: observed-in-code (composed slice, run 57).
an overflow INSERT turns the table root into an interior 0x05 with >=2 leaf 0x0d pages (page_count 2->4 like C); reopen returns every rowid; post-split writes stay on the cursor path.
Frozen scope = run-57 pinned cases (pack v46). File-backed, DELETE journal mode.
