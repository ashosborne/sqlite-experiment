# PASTE THIS as the entire Cloud Agent prompt

You are running the App modernisation **overnight conductor** in experiment mode:
**approve every model recommendation** from the completed estate-discovery radar, then one-shot the factory through Conversion for the first recommended slice.

## Context hard gate (do this first)

Read end-to-end (Field Guide wins over this paste):
- `migration-factory/docs/FIELD-GUIDE.md`
- `migration-factory/docs/OPERATOR-RUNBOOK.md`
- `migration-factory/docs/EXPERIMENTAL.md` (if present)
- `migration-factory/prompts/overnight-conductor-v0.1.md` (full law for this run)
- Stage prompts: discovery-agent-v0.2, test-generation-agent-v0.1, test-execution-agent-v0.1, conversion-agent-v0.1, verification-agent-v0.1
- `migration-factory/skills/architecture-pack/SKILL.md` + architecture-pack schemas
- Operator skills: record-bind, waive-characterization, conversion-pr, architecture-pack-bind

If any required file is missing: write `overnight/oneshot/BLOCKED.md` and STOP. Do not freestyle factory rules.

Emit `overnight/oneshot/CONTEXT_GATE.md` with files read + base commit SHA.

## Operator charter (locked for this experiment)

```yaml
OPERATOR: Ash Osborne
APP_ID: cargotracker
REPO_ROOT: .
FACTORY_ROOT: migration-factory
EXPERIMENT: APPROVE_MODEL_SUGGESTIONS_ONESHOT
MODE: PROVISIONAL_OVERNIGHT
ALLOW_PROVISIONAL_BIND: true
ALLOW_PROVISIONAL_MATRIX: true
ALLOW_WAIVE_RECORD: true
ALLOW_PROVISIONAL_PACK_BIND: true
ALLOW_CONVERSION: true
FIRST_CONVERSION_BATCH_MAX: 1
ALLOW_VERIFICATION_COMPARE: false
COMMIT_AS: overnight-oneshot-approve-model

# Start from estate-discovery radar artefacts (PR #2 / branch cursor/estate-surface-discovery-813d)
# If this branch is not current HEAD: checkout or merge that work first. Do not re-invent inventory.
ESTATE_RADAR_BRANCH: cursor/estate-surface-discovery-813d
REQUIRE_EXISTING:
  - inventory/cargotracker/APP_MANIFEST.yaml
  - inventory/cargotracker/COVERAGE.md
  - overnight/MORNING_BRIEF.md
  - discovery/*/CANDIDATES.md

# Primary conversion target = model recommended #1
PRIMARY_SLICE_ID: handling-report-rest
PRIMARY_FEATURE_PREFERENCE: handling-report-rest-c01   # POST /rest/handling/reports → JMS; fall back to next accepted in-slice if missing

# Also provisionally apply bind decisions for these slices (inventory + MANIFEST stamps only).
# Do NOT convert them in this run unless PRIMARY finishes early AND time remains; then at most one more feature.
BIND_QUEUE_IN_ORDER:
  - handling-report-rest
  - handling-file-ingest
  - cargo-inspection-messaging
  - cargo-monitoring-rest
  - booking-itinerary
  - cargo-tracking-public
  - voyage-carrier-movements
# Hold: pathfinder-graph-traversal until SME (random fabricator / deadline ignored) — still write OVERNIGHT_BIND recommending HOLD, do not accept convertibles there.
HOLD_SLICES:
  - pathfinder-graph-traversal
```

## Experiment rule: approve model suggestions

For every slice in `BIND_QUEUE_IN_ORDER`:
1. Read `discovery/<SLICE_ID>/SME_BRIEF.md` + `CANDIDATES.md` + stub `MANIFEST.yaml`.
2. Apply the brief's recommended accept / defer / reject / hold decisions into the MANIFEST.
3. Stamp each changed feature: `bind_source: overnight_provisional`, `bound_by: overnight-provisional/Ash Osborne`, reason citing file:line evidence.
4. Write `discovery/<SLICE_ID>/OVERNIGHT_BIND.md` (decision table). Morning human must confirm.
5. Update `inventory/cargotracker/APP_MANIFEST.yaml` pointers/status for those candidates; regenerate `COVERAGE.md` via `overnight/tools/gen_coverage.py` if present (never hand-edit COVERAGE).

For `HOLD_SLICES`: document HOLD in OVERNIGHT_BIND; do not accept features for conversion.

Do **not** invent new behaviours beyond radar candidates. Do **not** claim completeness.

## Then run overnight conductor on PRIMARY_SLICE_ID only

Follow `overnight-conductor-v0.1.md` pipeline steps 4→10 for `PRIMARY_SLICE_ID` only:

1. Discovery Phase B: deepen **accepted** features in that slice to `features/<FEATURE_ID>.md` (documented).
2. Test gen Phase A matrix + provisional matrix approve + Phase B specs/stubs.
3. Test exec: cheap legacy probe; if RECORD unrealistic → `WAIVED_PATHFINDER` + ADR (no PARITY=GREEN).
4. Architecture PACK DRAFT then provisional BIND (`bound_by: overnight-provisional/Ash Osborne`) with banner `OVERNIGHT_PROVISIONAL_BIND`.
5. Conversion: at most **1** feature (`PRIMARY_FEATURE_PREFERENCE` if accepted/documented), branch `overnight/conversion/<FEATURE_ID>`, modern edit surface under pack, `PARITY=UNVERIFIED`, provisional tests labelled.
6. Verification: write DEFERRED (COMPARE off).
7. Inventory bump + `overnight/oneshot/MORNING_BRIEF.md` + keep `overnight/oneshot/JOURNAL.md`.

If PRIMARY cannot convert (missing evidence, pack refuse): STOP with BLOCKED.md — do not silently switch to a worse slice without journaling why.

## Non-negotiables (still)

- No `PARITY=GREEN` under waiver or provisional goldens.
- No RECORD goldens from modern.
- No silent skip of Test Exec (RECORD or explicit WAIVED_*).
- No whole-repo rewrite. Legacy tree read-only unless pack allow-lists.
- No customer provenance claims (public OSS stand-in).
- Banner every Conversion PR: provisional overnight experiment; human re-bind / re-BIND before any production claim.

## Exit criteria

- [ ] CONTEXT_GATE written
- [ ] Provisional binds for BIND_QUEUE slices + HOLD for pathfinder
- [ ] PRIMARY slice deepened + testgen + waive-or-RECORD + provisional PACK BIND
- [ ] <=1 feature converted with PR/branch, PARITY=UNVERIFIED
- [ ] oneshot MORNING_BRIEF lists every provisional stamp and exact human re-approval clicks
- [ ] APP_MANIFEST + COVERAGE regenerated

Failure with honest BLOCKED.md is success. Silent partial greenwash is not.
