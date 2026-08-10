# Seed — date-time-funcs
SLICE_ID: date-time-funcs
SLICE_SEED: "date/time SQL functions (date, time, datetime, julianday, unixepoch, strftime, timediff)"
SEED_ENTRYPOINTS: sqlite3RegisterDateTimeFunctions()
OUT_OF_SCOPE: other builtin functions (builtin-scalar-agg-funcs)
Rationale: self-contained file with rich modifier grammar; classic characterization target.
