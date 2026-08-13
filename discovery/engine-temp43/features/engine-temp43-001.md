# engine-temp43-001 — TEMP schema isolation + triggers

Confidence: observed-in-code (composed slice, run 53).
CREATE TEMP/TEMPORARY TABLE under schema temp, temp-first unqualified resolution, main./temp. qualified, sqlite_temp_master, unqualified-drop precedence, TEMP triggers fire.
Frozen scope = run-53 pinned cases (pack v43).
