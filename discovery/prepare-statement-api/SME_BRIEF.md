# SME brief — prepare-statement-api (Phase A)

**Found:** 6 candidates covering the full statement state machine. Highest-value characterization seam in the estate: pure C API, deterministic return codes, no I/O beyond the db file.

**Boundaries:** compilation internals (tokenize/parse/codegen) intentionally excluded — they are downstream seeds; this slice pins the *contract*, not the compiler.

**Recommended binds:** accept all six; 002+004 first (step contract and column coercion matrix carry the most behavioural risk).
**Flag:** auto-reprepare (005) is an async-ish hidden retry path — keep it a separate card, do not fold into 001.
**Open questions:** UTF-16 variants — does the migration target need them, or defer?

STOPPED for human bind. No Phase B performed.
