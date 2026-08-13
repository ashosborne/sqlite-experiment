# engine-harvest39-003 — per-row blob expiry + TEXT-cell writes

Confidence: observed-in-code (composed slice, run 49).
UPDATE/DELETE of a DIFFERENT row leaves the handle readable and writable; the handle's own row expires it (rc 4, bytes 0); TEXT-cell writes patch bytes in place and the cell stays TEXT.
Frozen scope = run-49 pinned cases (pack v39).
