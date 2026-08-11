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

---

# Run 8 — stamp six + tidy (sqlite-stamp-six)

- stop.txt: absent. Jobs in charter order:
- Job 1: golden_approval extended to 8 stamped_case_ids in both prepare-statement TRACEABILITY files (run-6 002 stamp preserved via history note); pending_golden_approval cleared with note; C003 verified BLOCKED/golden_path null.
- Job 2: actuals collision fixed by splitting logs/record_raw.txt into six full-id files (no harness re-run); colliders deleted; results.json actual_path corrected ×6; byte-check each actual == its golden.
- Job 3: prep-002 card Summary + observables line de-MISUSE'd (pin wording); APP_MANIFEST note same tidy, conversion-block note kept; COVERAGE regenerated.
- Job 4: legacy_green → exactly 5 (error-status-001 + prep 001/002/003/005); parity_green all false; counts updated; schema VALID.
- All 10 goldens md5-verified byte-identical. No RECORD/COMPARE/PACK/Conversion. Run-7 brief preserved as MORNING_BRIEF-2026-08-11-run7.md.

---

# Run 10 — testgen + RECORD open/exec (sqlite-record-open-exec)

- stop.txt: absent. Cards + testgen/testexec prompts re-read; run-8/9 briefs preserved.
- Job 1: testgen packs for connection-lifecycle-api (2 cases) + exec-convenience-api (3 cases), spine pattern; full-case-id OBS tags; URI/open16/flag-MISUSE + multi-statement scope parked in DEFERRED per DO_NOT_RECORD; harness frees pzErrMsg.
- Job 2: run-5 pin reused (re-verified 3.54.0/armor0/autoreset0, zero drift); 5 cases RECORDED → goldens frozen → immediate replay byte-matched → REPLAY_GREEN. exec abort path = 4 (SQLITE_ABORT, card shape). golden_approval PENDING_HUMAN (no stamp). legacy_green untouched (5); parity 0. Stamped ten md5-identical; modern/ + pack zero diffs. results.json actual_path unique per full case id.
