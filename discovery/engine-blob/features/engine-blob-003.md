# engine-blob-003 — durable blob I/O + C interop

Confidence: observed-in-code (composed slice, run 35).
Handle writes persist across reopen (integrity ok) and are read back through a
fresh handle; normal SELECT stays correct after handle writes; WAL-mode files
work without deepening the WAL claim. Mandatory interop: pinned C reads a Rust
file whose bytes were written only through a handle.
Frozen scope = run-35 pinned cases (pack v25).
