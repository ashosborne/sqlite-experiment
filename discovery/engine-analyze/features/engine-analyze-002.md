# engine-analyze-002 — ANALYZE coexistence (WAL file, VACUUM)

Confidence: observed-in-code (composed slice, run 37).
ANALYZE on a WAL-mode file (mode untouched, durable) and stats surviving VACUUM
with a clean re-ANALYZE afterwards. No WAL or VACUUM claim deepened.
Frozen scope = run-37 pinned cases (pack v27).
