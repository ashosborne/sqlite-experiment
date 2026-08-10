# PASTE THIS as the entire Cloud Agent prompt

You are running a **provisional feature conveyor** on cargotracker: loop the remaining accepted features, auto-approve model recommendations at every factory gate, deepen → testgen → RECORD-or-waive → extend PACK → convert → next feature.

## Start state (do not reinvent)

1. Checkout / continue from branch **`overnight/oneshot-cargotracker`** (merge `overnight/conversion/handling-report-rest-c01` if `modern/**` for c01 is not already on HEAD).
2. Read end-to-end: `migration-factory/docs/FIELD-GUIDE.md`, `OPERATOR-RUNBOOK.md`, `prompts/overnight-conductor-v0.1.md`, stage prompts, architecture-pack skill + schemas, operator skills (record-bind, waive-characterization, conversion-pr, architecture-pack-bind, deepen-phase-b, matrix-gate-reply).
3. Read `overnight/oneshot/MORNING_BRIEF.md` and existing `architecture/cargotracker-spring-strangler/PACK.yaml`.
4. Emit `overnight/conveyor/CONTEXT_GATE.md` (files read + HEAD SHA). If factory pack missing: `overnight/conveyor/BLOCKED.md` and STOP.

## Operator charter

```yaml
OPERATOR: Ash Osborne
APP_ID: cargotracker
FACTORY_ROOT: migration-factory
EXPERIMENT: FEATURE_CONVEYOR_APPROVE_MODEL
MODE: PROVISIONAL_OVERNIGHT
ALLOW_PROVISIONAL_BIND: true          # already stamped; only adjust if evidence forces
ALLOW_PROVISIONAL_MATRIX: true
ALLOW_WAIVE_RECORD: true             # prefer real RECORD when Payara/JDK8 works (as oneshot did)
ALLOW_PROVISIONAL_PACK_BIND: true     # extend + re-stamp provisional BOUND under overnight-provisional
ALLOW_CONVERSION: true
FIRST_CONVERSION_BATCH_MAX: 1        # hard rule: ONE feature converted per loop iteration
ALLOW_VERIFICATION_COMPARE: false    # never mint PARITY=GREEN
COMMIT_AS: overnight-conveyor
MAX_FEATURES: 20                     # safety cap for this run
MAX_MINUTES: 480                     # stop cleanly when approaching; write MORNING_BRIEF
BASE_BRANCH: overnight/oneshot-cargotracker
SKIP_ALREADY_CONVERTED:
  - handling-report-rest-c01
# c02 on handling-report-rest was covered by the same endpoint in oneshot — treat as DONE unless
# modern tests do not cover its documented contract; then convert only the missing cases.
SKIP_OR_VERIFY:
  - handling-report-rest-c02
HOLD_FOREVER_THIS_RUN:
  - pathfinder-graph-traversal/*     # HOLD until SME
  - cargo-inspection-messaging-c04   # deferred
  - cargo-inspection-messaging-c05   # needs-SME
  - booking-itinerary-c05            # deferred
  - voyage-carrier-movements-c02     # deferred
```

## Conversion queue (accepted only, model priority order)

Process **in this order**. Skip any id already `converted` in APP_MANIFEST or listed in SKIP_*.

1. handling-file-ingest-c01
2. handling-file-ingest-c02
3. handling-file-ingest-c03
4. handling-file-ingest-c04
5. cargo-inspection-messaging-c01
6. cargo-inspection-messaging-c02
7. cargo-inspection-messaging-c03
8. cargo-monitoring-rest-c01
9. booking-itinerary-c01
10. booking-itinerary-c02
11. booking-itinerary-c03
12. booking-itinerary-c04
13. cargo-tracking-public-c01
14. cargo-tracking-public-c02
15. voyage-carrier-movements-c01

Do **not** invent new feature ids. Do **not** bind pathfinder or deferred/needs-SME rows.

## Loop body (repeat until queue empty, MAX_FEATURES, or MAX_MINUTES)

For each `FEATURE_ID` with slice `SLICE_ID`:

### A) Journal start
Append `overnight/conveyor/JOURNAL.md`: feature id, slice, HEAD SHA, timestamp.

### B) Deepen (Discovery Phase B) if needed
If `discovery/<SLICE_ID>/features/<FEATURE_ID>.md` missing or status not `documented`:
- Deepen **this feature only** from observed code (no desired-behaviour invention).
- Update MANIFEST feature → `documented` with `bind_source: overnight_provisional` preserved.

### C) Test generation
- Ensure `testgen/<SLICE_ID>/` exists; add/extend TRACEABILITY + scenarios for this feature.
- Auto-approve cases as `approved_provisional`.
- Phase B: specs + harness stubs (or reuse oneshot harness patterns). Prefer thin status-line contracts like the handling-report-rest RECORD.

### D) Test execution
- Prefer **real RECORD** on legacy (Payara/JDK8) when the environment allows (oneshot proved it works).
- Mark goldens `golden_status: overnight_pending_human_approve`.
- If RECORD fails fast or is unrealistic for this seam: `WAIVED_PATHFINDER` + ADR; Conversion may continue; **no PARITY=GREEN**.

### E) Architecture pack extend
- Update `architecture/cargotracker-spring-strangler/PACK.yaml`: add this feature to `first_conversion_batch` / mapping_rules / edit_surface as needed (version bump if schema requires).
- Keep provisional BOUND stamp (`bound_by: overnight-provisional/Ash Osborne`) + `OVERNIGHT_PROVISIONAL_BIND` banner.
- Do not widen edit_surface beyond what this feature needs.

### F) Conversion (exactly one feature)
- Branch: `overnight/conversion/<FEATURE_ID>` from current conveyor HEAD (or commit on a long-lived `overnight/conveyor` branch and tag each feature commit clearly — prefer **one branch per feature** + open/prepare PR body).
- Implement under pack `edit_surface` (default `modern/**`); legacy read-only.
- Provisional tests labelled; `PARITY=UNVERIFIED`.
- Refuse any second feature in the same iteration.
- Write `overnight/conveyor/features/<FEATURE_ID>/CONVERSION_PR_BODY.md`.

### G) Inventory + commit
- Update `inventory/cargotracker/APP_MANIFEST.yaml` (feature → converted / UNVERIFIED).
- Regenerate `COVERAGE.md` via `overnight/tools/gen_coverage.py` (never hand-edit).
- Commit with prefix `conveyor: <FEATURE_ID>`.
- Append JOURNAL outcome (RECORD/WAIVED, pack version, branch SHA, risks).

### H) Continue
Next feature in the queue. If blocked on one feature: write `overnight/conveyor/features/<FEATURE_ID>/BLOCKED.md`, mark it skipped in JOURNAL, **continue** to the next queue item (do not halt the whole conveyor unless CONTEXT_GATE / factory files missing).

## Non-negotiables

- No `PARITY=GREEN`, no `verified`, no goldens minted from modern.
- No silent skip of Test Exec (RECORD or explicit WAIVED_*).
- One feature conversion per iteration.
- No whole-repo rewrite. No customer provenance claims.
- Banner every Conversion PR: provisional conveyor experiment; human re-bind / re-BIND / golden approve before production claims.
- Field Guide + schemas win over this paste.

## End of run

Write `overnight/conveyor/MORNING_BRIEF.md`:
1. Features converted / skipped / blocked (table)
2. Every provisional stamp still needing human clicks
3. Branch + PR links (or `pull/new/...` URLs)
4. Pack version + CONTRACT_RISK open questions
5. What remains in the queue
6. `completeness: incomplete` (always)

## Exit criteria

- [ ] CONTEXT_GATE written
- [ ] At least attempted every queue item until cap/time (or honest BLOCKED per feature)
- [ ] Each converted feature has branch/PR body + PARITY=UNVERIFIED
- [ ] APP_MANIFEST + COVERAGE regenerated
- [ ] MORNING_BRIEF + JOURNAL complete
- [ ] No PARITY=GREEN

Silent partial greenwash is failure. Honest BLOCKED rows are success.
