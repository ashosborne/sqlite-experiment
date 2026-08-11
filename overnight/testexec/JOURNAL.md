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

---

# Run 7 — RECORD six (sqlite-testexec-record-six)

- stop.txt: absent. Hygiene first: TRACE titles (002-C002 autoreset wording, 002-C003 "BLOCKED — UAF, do not RECORD", error-status C002 drop "MISUSE-safe"), CASE-003.md rewritten without armor advice, 003-C002/001-C002/005-C001 specs aligned with harness flow. Harness control flow untouched.
- Build: run-5 build reused; fingerprint re-verified live (3.54.0 / armor 0 / autoreset 0); zero src/ext diffs since run-5 base; BASELINE dump NOT rewritten. Harness compiled fresh.
- RECORD: six cases captured (scrub: none); goldens frozen; immediate replay byte-matched all six → REPLAY_GREEN. Stamped four verified byte-identical (md5). C003 not touched.
- Traceability: testgen six → REPLAY_GREEN (assert_mode RECORDED); tests/characterization six rows appended with run id; pending_golden_approval=PENDING_HUMAN recorded beside the run-6 stamp (which covers only the 002 pair). legacy_green unchanged (exactly 2). No PACK, no COMPARE, no Conversion.
