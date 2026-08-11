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

---

# Run 6 — stamp + card patches + next spine batch (sqlite-stamp-and-next-spine)

## Job 1 — stamp + truth

- stop.txt: absent. Golden hashes captured before any change; verified BYTE-IDENTICAL after (md5 diff clean).
- golden_approval: HUMAN_ACCEPTED (Ash Osborne, 2026-08-11 Europe/London) written to all 4 TRACEABILITY files; 4 stamped cases stay REPLAY_GREEN; C003 stays BLOCKED (UAF), golden_path null, no sqlite3_step(NULL) under this ID.
- C001 observable classes tagged: contract = the integers (prepare.rc=1, errcode=1, extended=1); errmsg.text = wording_deferred (shape-only for future COMPARE; recorded line kept in golden). C002 'out of memory' = contract string (sqlite3ErrStr/SQLITE_NOMEM — Terry).
- Card patches (edit-in-place, old sentence quoted inside each patch):
  - error-status-api-001: NULL-db validation line → "errcode=7 (SQLITE_NOMEM) + errmsg 'out of memory' (static guarded answers). This is not MISUSE."
  - prepare-statement-api-002: DONE sentence → autoreset truth (third step returns ROW(100) on this pin; manual rule = OMIT_AUTORESET=on contract only); finalized-statement line → "NOT observed and not safely observable (UAF even with armor); C003 permanently BLOCKED" (no observed-MISUSE claim left).
  - Discovery notes appended to both cards pointing at run 2026-08-11T1205Z-legacy-record + BASELINE fingerprint.
- APP_MANIFEST: prepare-statement-api-002 notes += not-conversion-ready-while-C003-BLOCKED; pending-approval notes flipped to HUMAN_ACCEPTED; legacy_green still exactly the 2 stamped IDs; parity_green all false; schema VALID; COVERAGE regenerated. No other card touched. No PACK authored.
