# engine-wal-001 — WAL mode, sidecar files, durability, C interop

Confidence: observed-in-code (composed slice, run 32).
journal_mode=WAL sticks on file DBs (header versions=2; reopen reports wal);
sidecars appear on first write, not at the pragma; clean close checkpoints and
removes -wal/-shm; wal->delete switch; :memory: refuses WAL; multi-commit,
rollback, multi-page and same-process second-connection visibility; empty-DB
mode persistence with integrity_check ok. C CLI recovers a Rust db+wal whose
data lives only in the -wal.
Frozen scope = run-32 pinned cases (pack v22). Single-process only.
