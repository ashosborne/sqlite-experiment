# engine-collation-002 — column COLLATE / per-connection persistence honesty

Confidence: observed-in-code (composed slice, run 30).
CREATE TABLE column COLLATE clauses drive WHERE and ORDER BY; registration is
per-connection so a file reopen without re-register reproduces C's
"no such collation sequence" error; collation names fold case-insensitively
(incl. quoted); RTRIM/NOCASE interaction pinned.
Frozen scope = run-30 pinned cases (pack v20). Residual: declared-collation
resolution is by unambiguous column name; collation-aware UNIQUE/INDEX keys not pinned.
