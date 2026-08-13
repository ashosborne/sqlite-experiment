# engine-pager44-002 — rollback restores / commit persists

Confidence: observed-in-code (composed slice, run 55).
ROLLBACK replays the journal to restore UPDATE/DELETE/multi-row pre-images; COMMIT persists all pages and a fresh open re-reads them.
Frozen scope = run-55 pinned cases (pack v44). File-backed, DELETE journal mode.
