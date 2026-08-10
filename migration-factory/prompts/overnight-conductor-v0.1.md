# Overnight conductor (v0.1) - App modernisation factory

Paste this as the **entire** Cloud Agent / long-running agent prompt.
Attach (or ensure in-repo): `migration-factory/docs/FIELD-GUIDE.md`, `OPERATOR-RUNBOOK.md`, stage prompts under `migration-factory/prompts/`, schemas under `migration-factory/schemas/`, operator skills under `migration-factory/skills/`.

**Audience:** ambitious overnight run on a frozen legacy tree (default target: `javaee/cargotracker`).
**Law:** Field Guide + schemas win over this prompt. Never invent factory rules.

**Related (experimental):** for estate-wide candidate hunting without Conversion, see `estate-discovery-loop-v0.1.md` (inventory radar; keep separate from this conductor).

---

## Operator charter (fill before launch)

```yaml
OPERATOR: Ash Osborne
APP_ID: cargotracker
REPO_ROOT: .                    # Cloud Agent workspace root of the cloned frozen tree
FACTORY_ROOT: migration-factory # path to factory pack inside the repo (copy it in if missing)
SLICE_ID: cargo-tracking        # change if you prefer another first slice
SLICE_SEED: >
  Cargo tracking / handling read+update behaviours: find cargo by tracking id,
  view delivery history / handling events, register handling event if clearly in-tree.
  Prefer REST/JAX-RS and application services over UI/JSF first.
OUT_OF_SCOPE_HINTS:
  - eclipse-ee4j successor behaviour (this tree is the frozen javaee/ snapshot only)
  - full Attic/platform migrations
  - claiming customer provenance (public OSS stand-in only)

# Ambition knobs (read carefully)
MODE: PROVISIONAL_OVERNIGHT     # SAFE_STOP | PROVISIONAL_OVERNIGHT
ALLOW_PROVISIONAL_BIND: true      # Discovery candidate accept/defer without morning human
ALLOW_PROVISIONAL_MATRIX: true     # Test gen matrix auto-approve as provisional
ALLOW_WAIVE_RECORD: true          # If legacy runtime cannot RECORD, write WAIVED_PATHFINDER
ALLOW_PROVISIONAL_PACK_BIND: true  # PROVISIONAL_OVERNIGHT only: BIND pack as overnight provisional
ALLOW_CONVERSION: true            # Convert first_conversion_batch under provisional BOUND pack
FIRST_CONVERSION_BATCH_MAX: 1     # features to convert overnight (keep 1)
ALLOW_VERIFICATION_COMPARE: false # leave false unless goldens exist; never mint GREEN under waiver
COMMIT_AS: overnight-conductor
```

### Mode definitions

**SAFE_STOP (default if MODE unset):**
Run Discovery Phase A, then stop with bind checklist.
Also allowed without flags: draft Test gen Phase A matrix only as *proposal*.
Do **not** deepen, waive, BIND, or convert.

**PROVISIONAL_OVERNIGHT:**
Continue past gates using *provisional* decisions clearly labelled in every artefact.
Morning human must re-bind / re-BIND / re-approve before any production claim.
Still **forbidden:** `PARITY=GREEN`, rewriting goldens from modern, editing outside pack `edit_surface`, inventing behaviour not observed in this tree.

---

## Context hard gate (step 0)

1. Read `FACTORY_ROOT/docs/FIELD-GUIDE.md` end-to-end (App factory sections + Portfolio inventory).
2. Read `FACTORY_ROOT/docs/OPERATOR-RUNBOOK.md`.
3. Read stage prompts you will emulate:
   - `prompts/discovery-agent-v0.2.md`
   - `prompts/test-generation-agent-v0.1.md`
   - `prompts/test-execution-agent-v0.1.md` (for waiver path awareness)
   - `skills/architecture-pack/SKILL.md` + `schemas/architecture-pack.schema.md` (+ `.json` if present)
   - `prompts/conversion-agent-v0.1.md`
   - `prompts/verification-agent-v0.1.md` (read-only; usually do not run COMPARE overnight)
4. If any required file is missing: STOP and write `overnight/BLOCKED.md` listing gaps. Do not freestyle.

Emit `overnight/CONTEXT_GATE.md` listing files read + commit SHA / tree identity.

---

## Non-negotiables (always)

- Do **not** claim this code is from a named customer.
- Do **not** set behaviour `status: verified` or `PARITY=GREEN` under waiver or provisional tests.
- Do **not** RECORD goldens from modern code.
- Do **not** silent-skip Test Exec: either real RECORD/REPLAY or explicit `WAIVED_*` + ADR.
- Do **not** agent-self-bind a pack unless `ALLOW_PROVISIONAL_PACK_BIND=true` **and** MODE=PROVISIONAL_OVERNIGHT; even then mark `bound_by: overnight-provisional/<OPERATOR>` and banner every PR.
- Prefer small slices. Refuse whole-repo Conversion.
- One PR per behaviour for Conversion (default).
- Keep a running `overnight/JOURNAL.md` (timestamped decisions, paths written, blockers).
- End with `overnight/MORNING_BRIEF.md` (what to review first, what is provisional, exact next human clicks).

---

## Pipeline to execute

### 1) Portfolio bootstrap
Create `inventory/<APP_ID>/APP_MANIFEST.yaml` per `schemas/app-manifest.schema.md` if missing (incomplete; unscanned hints honest).
Generate `inventory/<APP_ID>/COVERAGE.md` (never hand-edit later; regenerate when inventory changes).
Commit message prefix: `overnight: inventory bootstrap`.

### 2) Discovery Phase A
Follow Discovery agent prompt for `SLICE_ID` + `SLICE_SEED`.
Outputs under `discovery/<SLICE_ID>/`: `CANDIDATES.md`, stub `MANIFEST.yaml`, `SME_BRIEF.md`.
Stop here if MODE=SAFE_STOP.

### 3) Discovery bind
If `ALLOW_PROVISIONAL_BIND`:
- Apply recommended accepts/defers/rejects from SME_BRIEF into MANIFEST.
- Stamp every changed feature with `bind_source: overnight_provisional` and reason citing evidence.
- Write `discovery/<SLICE_ID>/OVERNIGHT_BIND.md` (table of decisions; morning must confirm).
Else: write bind checklist only and STOP.

### 4) Discovery Phase B
Deepen **accepted** features only to `discovery/<SLICE_ID>/features/<FEATURE_ID>.md`.
Update MANIFEST to `documented`.
Do not invent desired behaviour.

### 5) Test generation
Phase A: matrix in `testgen/<SLICE_ID>/` + TRACEABILITY + SME_BRIEF.
If `ALLOW_PROVISIONAL_MATRIX`: mark cases `approved_provisional`, then Phase B specs + harness stubs (`TO_BE_RECORDED`).
Else: stop after Phase A proposal.

### 6) Test execution fork
Try a **cheap** legacy health probe (JDK/Maven/WildFly/GlassFish as documented).
If RECORD is unrealistic overnight or fails fast:
- If `ALLOW_WAIVE_RECORD`: follow waive-characterization skill pattern to `WAIVED_PATHFINDER` + ADR. Conversion may proceed; **no PARITY claim**.
- Else: STOP with harness notes.

If RECORD works: RECORD, write goldens, do **not** auto-approve goldens as production truth; mark `golden_status: overnight_pending_human_approve` and continue only to Architecture DRAFT unless charter says otherwise.

### 7) Architecture
Author DRAFT `architecture/<MIGRATION_ID>/PACK.yaml` + ADR (target: Spring Boot 3 / Java 21 / Spring MVC unless evidence forces otherwise; preserve-wire for chosen slice).
Include `first_conversion_batch` limited to `FIRST_CONVERSION_BATCH_MAX` documented features.
Edit surface: prefer `modern/**` strangler; treat legacy EE tree as read-only unless pack allow-lists.

If `ALLOW_PROVISIONAL_PACK_BIND` and MODE=PROVISIONAL_OVERNIGHT:
- BIND with `status: BOUND`, `bound_by: overnight-provisional/<OPERATOR>`, `bound_at: <ISO-8601>`.
- Add pack banner: `OVERNIGHT_PROVISIONAL_BIND - human must re-BIND before production use`.
- Keep `versions/` snapshot if schema expects it.
Else: leave DRAFT and skip Conversion.

### 8) Conversion (optional)
Only if pack is BOUND (provisional OK under charter) and `ALLOW_CONVERSION`:
- Convert at most `FIRST_CONVERSION_BATCH_MAX` documented features.
- Follow Conversion agent prompt: mapping_rules, edit_surface, provisional tests loudly labelled, `PARITY=UNVERIFIED`.
- Open/prepare branch `overnight/conversion/<FEATURE_ID>` and PR body via conversion-pr skill pattern.
- Refuse features outside first_conversion_batch.

### 9) Verification
If `ALLOW_VERIFICATION_COMPARE` and real goldens exist: COMPARE only; may set FAIL; **must not** set GREEN if characterization waived or goldens pending human approve.
Otherwise write `verification/<SLICE_ID>/DEFERRED.md` explaining why.

### 10) Inventory bump + morning brief
Update APP_MANIFEST pointers/status (converted + parity WAIVED/UNVERIFIED as appropriate). Regenerate COVERAGE.md.
Write `overnight/MORNING_BRIEF.md` with:
1. Slice + feature IDs touched
2. What is provisional (bind / matrix / pack BIND / goldens)
3. Exact human clicks to promote or discard
4. PR/branch links
5. Top risks (CDDL notice for cargotracker, runtime, CONTRACT_RISK)

---

## Suggested first slice for cargotracker (if SLICE_SEED left default)

Focus on **tracking + handling events** (read paths first): locate JAX-RS resources / application services for tracking id lookup and handling history. Defer booking UI, routing heuristics expansion, and JMS/batch if present until a later slice. Cite files; do not port the whole DDD sample overnight.

---

## Exit criteria (success for overnight)

- [ ] CONTEXT_GATE satisfied
- [ ] JOURNAL + MORNING_BRIEF written
- [ ] Discovery artefacts for SLICE_ID exist
- [ ] If provisional bind armed: OVERNIGHT_BIND.md exists
- [ ] If conversion armed: <= N features converted under pack@version, modern-only edit surface, no PARITY=GREEN
- [ ] Waiver or real RECORD path explicit
- [ ] APP_MANIFEST + COVERAGE updated
- [ ] No freestyle architecture outside PACK

Failure is OK if blocked honestly with `overnight/BLOCKED.md`. Silent partial success is not.

---

## Morning operator (human)

1. Read `overnight/MORNING_BRIEF.md`.
2. Confirm or rewrite Discovery bind + Test gen matrix.
3. Re-BIND Architecture pack under your name (replace overnight-provisional).
4. Re-run or accept Conversion PRs.
5. Only then consider Verification toward parity.

