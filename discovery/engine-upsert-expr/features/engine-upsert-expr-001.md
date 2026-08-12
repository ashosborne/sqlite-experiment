# engine-upsert-expr-001 — expression UNIQUE conflict targets

Confidence: observed-in-code (composed slice, run 31).
ON CONFLICT(lower(c)) / (a+b) / (lower(a), b) resolve to matching expression UNIQUE
indexes and drive DO NOTHING / DO UPDATE (excluded.*); mismatched targets reproduce
C's "ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint";
conflicts on non-targeted constraints abort rc 19 with the qualified message;
catch-all, OR IGNORE/REPLACE, case/whitespace folding and durable reopen pinned.
Frozen scope = run-31 pinned cases (pack v21).
