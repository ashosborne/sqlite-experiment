# MORNING BRIEF — sqlite-experiment run 3

# ⚠️ PHASE B — FULL CATALOGUE DEEPEN ⚠️

Run: 2026-08-11 (resumed from `0f1fa004` on `cursor/sqlite-estate-discovery-d22c`) · committed as `sqlite-deepen-all`
Bind: `ACCEPT_ALL_CURRENT_CANDIDATES` — Ash Osborne, 2026-08-11 Europe/London (`overnight/phase-b/BIND_ALL.md`)

## Counts (no percentages)

| Metric | Count |
| --- | --- |
| Behaviours bound (accepted from candidate) | 185 (all current candidates; across 107 slices) |
| **Documented** (card with ≥1 real cite) | **185** |
| — confidence `observed-in-code` | 175 |
| — confidence `inferred` (honest file-level evidence: wasm-js-api ×3, wasm-opfs ×2, jni-java-surface ×3, jni-binding ×1, wasm-binding ×1) | 10 |
| needs-SME status rows | 0 (SME questions carried inside cards' Assumptions/unknowns instead) |
| blocked status rows | 0 |
| Slices with no card | 0 |
| Unscanned hints (unchanged, no cards invented) | 3 (misc-mmapwarm, misc-memtrace, misc-pcachetrace) |

Card batches journaled in `overnight/phase-b/JOURNAL.md` (15 batches, one commit each).

## Baseline pin

`overnight/BASELINE.md` — default Unix amalgamation (`./configure && make sqlite3` on this tree);
fingerprint via `sqlite3_compileoption_used/get` (src/main.c:5220/5253). Census cards
(compile-options 002/003, vfs-unix lock-style matrix, vdbe 199-opcode loop, misc-utilities pack,
wasm/jni umbrellas) stay one card each — matrices documented, not exploded.

## Where things live

- Cards: `discovery/<SLICE_ID>/features/<FEATURE_ID>.md` (185 files; v0.2 Phase B shape: summary,
  entrypoints with file:line, observables, as-implemented behaviour, validation, edge cases,
  dependencies, assumptions, confidence, evidence)
- Slice MANIFESTs: all `phase: B`, every feature `documented` with `behaviour_doc`
- APP_MANIFEST: all 185 behaviours `documented` + `discovery_card` pointers; schema-valid;
  surfaces intentionally remain `candidate` (charter bound behaviours only — see BIND_ALL.md)

## What this run did NOT do (explicit)

Did **not** run Test gen, RECORD/REPLAY, Architecture PACK authoring or BIND, Conversion, or
Verification. Did **not** touch `src/`, `ext/`, or `test/` (verified: zero product diffs vs base).
Did **not** invent feature IDs, seed new slices, or card the 3 residual hints. Did **not** claim
completeness anywhere.

## completeness: incomplete

Human residual gate still required. Residuals: 3 unscanned hints; METHOD_COVERAGE holes from runs
1–2 (crash/fault-injection behaviour, runtime-only behaviour, generated-code surfaces,
per-compile-option diffs pending a non-default baseline); 10 inferred-confidence cards need SME
resolution or a waiver before Test gen (Field Guide hard-stop applies to them).

## Closing ask — which documented IDs unlock Test gen first?

Operator-suggested first Test-gen batch (highest characterization value, all observed-in-code,
callable, deterministic):

1. `error-status-api-001` — the error contract every other suite asserts against
2. `prepare-statement-api-001..006` — the statement state machine (core spine)
3. `exec-convenience-api-001`, `connection-lifecycle-api-001..003`
4. `dml-codegen-002` (ON CONFLICT matrix) + `expr-codegen-002` (NULL 3-valued logic)
5. `date-time-funcs-001..004` (pin TZ) + `json-funcs-001..004` + `printf-format-001`
6. `builtin-scalar-agg-funcs-001..003`

Pick the batch (≤15 cards per Test-gen run per Field Guide) and I'll hand off per
`compose-stage-prompt`. The 10 inferred cards (wasm/jni) need SME sign-off or `waive-characterization`
before they can enter any Test-gen batch.
