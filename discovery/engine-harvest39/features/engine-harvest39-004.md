# engine-harvest39-004 — sqlite3_randomness + test_control PRNG

Confidence: observed-in-code (composed slice, run 49).
N>0 fills exactly N bytes (tail untouched), draws differ, N=0 writes nothing; PRNG SAVE/RESTORE replays the stream and same-seed reseeds replay (within-run predicates; unseeded bytes never goldens).
Frozen scope = run-49 pinned cases (pack v39).
