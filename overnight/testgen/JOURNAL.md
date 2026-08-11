# Journal — run 4: hygiene + testgen spine (sqlite-experiment)

Charter: user paste = hygiene job + approved case matrix for error-status-api-001 and prepare-statement-api-002 ONLY.
COMMIT_AS sqlite-testgen-spine. No RECORD / goldens / conversion. Resumed from b737aefe0.

## Job 1 — index hygiene

- stop.txt: absent. Context gate: Field Guide (Test generation), TWO-FACTORIES (app line confirmed), runbook, test-generation-agent v0.1, testgen-traceability schema read. Both behaviour cards read before any case writing.
- 10 inferred wasm/jni IDs: slice features → needs-SME (behaviour_doc kept; needs-SME reason in open_questions), card headers → needs-SME banner, APP notes → confidence=inferred + needs-SME line. NOT unlocked for this Test gen.
- File-only pair (compile-options-omit-enable-002, vdbe-engine-002): card + slice confidence → inferred; assumption line added; APP notes updated. Status stays documented.
- APP_MANIFEST top-level notes rewritten: Phase B documented / surfaces candidate / completeness incomplete (stale "Phase A radar, candidates unbound" removed). estate_scan=partial retained as string. Schema VALID. COVERAGE regenerated.
- Other 173 observed-in-code cards untouched.
- BASELINE.md: "fingerprint not captured" line appended (no build present; charter forbids compiling this run).

## Job 2 — Test gen (spine batch: 2 IDs, 5 cases)

- stop.txt: absent. Matrix gate satisfied by the paste itself (PHASE A_AND_B) — recorded in each pack's SME_BRIEF/MANIFEST; no second gate awaited, no cases beyond the approved list.
- testgen/error-status-api/: C001 (errmsg+errcode after failed prepare), C002 (NULL-db static answers). Both TO_BE_RECORDED, observed-in-code gate. DEFERRED: exact wording, UTF-16, error_offset corpus, limit/status64.
- testgen/prepare-statement-api/: C001 (ROW→DONE), C002 (step after DONE, no reset), C003 (step after finalize — honest API_ARMOR caveat: unarmored build ⇒ RECORD should BLOCK rather than freeze UB). DEFERRED: reprepare/SCHEMA matrix, BUSY-on-COMMIT, UTF-16, other prepare IDs.
- Harness: one C capture-driver stub per slice under testgen/<slice>/harness/ (public C API only; OBS-line output for scrub+freeze; exit code meaningless). NOT compiled this run: sqlite3.h is a generated file absent from the tree and the charter forbids building — recorded plainly; cases remain `specified` (stubs complete), nothing marked green, nothing BLOCKED except the conditional C003 note for RECORD.
- TRACEABILITY.yaml both packs parse; rows conform to testgen-traceability schema v0.1 (id/feature_id/evidence/spec/harness/assert_mode/observables/deferred).
