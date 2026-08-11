# Journal — run 5: Test execution RECORD (sqlite-experiment)

Charter: MODE=RECORD, TARGET=legacy, 4 CASE_IDS, C003 blocked by operator (UAF). COMMIT_AS sqlite-testexec-record. Resumed from 23b6a1ed1.

- stop.txt: absent. Context gate: Field Guide (Test execution), runbook, test-execution-agent v0.1, testexec-results schema, both testgen packs + case specs + BASELINE.md read.
- Safety first: C003 step-after-finalize capture REMOVED from the harness source (UAF even with API_ARMOR — armor checks NULL, not freed handles); testgen TRACEABILITY C003 → blocked. UB not executed, not captured, not frozen.
- Build: out-of-tree /tmp/sqlite-build (keeps repo clean of generated files) — /workspace/configure && make sqlite3 sqlite3.c, default flags, ~37s. sqlite_version 3.54.0. Fingerprint APPENDED to overnight/BASELINE.md: ENABLE_API_ARMOR=0 (expected), OMIT_AUTORESET=0 (expected), full compileoption_get dump incl. surprise defaults (FTS3/4, RTREE, DBSTAT/DBPAGE/BYTECODE vtabs, DQS=0...). Amalgamation/binaries NOT committed.
- Harnesses compiled against built sqlite3.c/sqlite3.h (public API only). Smoke = first run of error_status_harness.
- RECORD: 4 cases captured (scrub profile: none — deterministic static inputs). Goldens frozen under tests/characterization/<slice>/cases/. Immediate replay: byte-match on all 4 → REPLAY_GREEN.
- Notable: C002 step-after-DONE returned SQLITE_ROW (100) via autoreset — as the charter predicted; card sentence pins the manual contract → card-refinement note for Discovery in REPORT.md (no golden rewrite, no failure class).
- errmsg wording frozen as captured but flagged: NOT a pass/fail contract; human review at golden approval.
- TRACEABILITY updated both places (testgen: 4× REPLAY_GREEN, C003 blocked; tests/characterization: run TRACEABILITY + golden_approval PENDING_HUMAN). APP_MANIFEST: legacy_green=true ONLY on error-status-api-001 + prepare-statement-api-002 (charter + schema: after real REPLAY_GREEN); other 183 untouched. Schema VALID; COVERAGE regenerated.
