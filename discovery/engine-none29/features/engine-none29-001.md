# engine-none29-001 — qualified-name-in-trigger DML rejection

Confidence: observed-in-code (composed slice, run 39).
Non-TEMP trigger bodies reject qualified INSERT/UPDATE/DELETE targets with C's exact message (trigger not created); TEMP triggers are exempt; qualified names in a trigger SELECT are allowed; multi-statement bodies fail as a whole.
Frozen scope = run-39 pinned cases (pack v29).
