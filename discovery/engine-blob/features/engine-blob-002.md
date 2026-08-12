# engine-blob-002 — incremental read/write, bounds, expiry

Confidence: observed-in-code (composed slice, run 35).
Full and offset-slice reads; bounds errors ("SQL logic error") leave the buffer
untouched; write at offset visible to SQL with the length unchanged; read-only
write rc 8; write-past-end refused; zeroblob preallocate + interior write;
expiry on UPDATE/DELETE of the row (rc 4 "query aborted", bytes -> 0);
zero-length blob edges.
Frozen scope = run-35 pinned cases (pack v25).
