# MORNING BRIEF — sqlite-experiment run 4: hygiene + Test gen (spine batch)

Run: 2026-08-11 · resumed from `b737aefe0` on `cursor/sqlite-estate-discovery-d22c` · committed as `sqlite-testgen-spine`
Stage: **Test generation only** (specs + stubs). Matrix gate = the operator paste itself.

## 1. Hygiene results (Job 1)

| Fix | Count |
| --- | --- |
| Behaviour notes rewritten to `confidence=inferred` in APP_MANIFEST | 12 (10 wasm/jni + 2 file-only) |
| Slice features → `needs-SME` (cards banner'd, reason in open_questions) | 10 (wasm-js-api ×3, wasm-opfs ×2, jni-java-surface ×3, jni-binding, wasm-binding) |
| File-only cards downgraded to `confidence=inferred` (status stays documented) | 2 (compile-options-omit-enable-002, vdbe-engine-002) |
| Observed-in-code cards touched beyond the two above | 0 (173 untouched) |
| APP_MANIFEST top-level notes | Fixed: "Phase B cards: behaviours documented; surfaces still candidate; completeness incomplete" (stale Phase-A sentence removed; `estate_scan=partial` kept; no invented keys) |
| COVERAGE.md | Regenerated from APP_MANIFEST; schema VALID |
| BASELINE fingerprint | **Not captured** — no build exists in this workspace and the charter forbids compiling; "fingerprint not captured" line appended to `overnight/BASELINE.md`. Test execution captures it at first build. |

## 2. Cases written (Job 2) — assert mode TO_BE_RECORDED, no green claims

| Case ID | Spec | Harness |
| --- | --- | --- |
| error-status-api-001-C001 (errmsg+errcode after failed prepare of invalid SQL) | testgen/error-status-api/scenarios/error-status-api-001/CASE-001.md | testgen/error-status-api/harness/error_status_harness.c |
| error-status-api-001-C002 (errcode/errmsg on NULL db — guarded static answers) | .../CASE-002.md | same harness |
| prepare-statement-api-002-C001 (single-row SELECT: step→ROW, step→DONE) | testgen/prepare-statement-api/scenarios/prepare-statement-api-002/CASE-001.md | testgen/prepare-statement-api/harness/step_state_machine_harness.c |
| prepare-statement-api-002-C002 (step after DONE without reset) | .../CASE-002.md | same harness |
| prepare-statement-api-002-C003 (step a finalized statement) | .../CASE-003.md | same harness |

TRACEABILITY.yaml in both packs (schema v0.1); scenario MANIFESTs record the paste as the matrix gate.
**No other feature IDs specified.** The 10 needs-SME rows stay locked out of Test gen pending SME/waiver.

## 3. Deferred / blocked

- **Deferred (operator-directed, in each pack's DEFERRED.md):** exact error-message wording as contract; UTF-16 twins; error_offset corpus; limit/status64 (other IDs); auto-reprepare/SQLITE_SCHEMA matrix; BUSY-on-COMMIT; prepare-statement-api-001/003–006.
- **Conditional block recorded for RECORD:** prepare-statement-api-002-C003 (step-after-finalize) is only safely capturable on a build with `SQLITE_ENABLE_API_ARMOR`; on an unarmored baseline Test execution should mark it `BLOCKED (unsafe capture on unarmored build)` rather than freeze UB. Documented in the case spec + harness comment.
- **Harness compile status:** stubs NOT compiled this run — `sqlite3.h` is a generated file absent from the source tree and the charter forbids building. Cases are `specified` (stubs complete); no fake green run.

## 4. What this run did NOT do (explicit)

Did **not** RECORD or freeze goldens, did **not** run Test execution, did **not** convert, PACK, or
seed Discovery. Did **not** touch `src/`, `ext/`, `test/`. Writes confined to `discovery/**`
(hygiene only), `inventory/**`, `overnight/**`, `testgen/**`.

## 5. completeness: incomplete

Estate residuals unchanged (3 unscanned hints; METHOD_COVERAGE holes; 10 needs-SME cards; per-flag
compile-option diffs pending baseline fingerprint).

## 6. Closing ask

**Run Test execution RECORD on these 5 cases?** It needs: (a) the pinned baseline build
(`./configure && make sqlite3.c` per BASELINE.md) which also captures the missing compileoption
fingerprint, (b) MODE=RECORD, TARGET=legacy on the two harness drivers, (c) the C003 armor check
before capture. Say the word and the stage handoff is ready.
