# Migration factory Field Guide (living doc)

How humans run Cursor-powered modernisation factories.  
Audience: customer operators, Slice SMEs, domain engineers, platform/SRE, and enablement staff ramping onto this approach.

**Read first:** [TWO-FACTORIES.md](./TWO-FACTORIES.md) — we run **two product lines** (application modernisation and OS modernisation) on one harness, plus a thin [LINKAGE.md](./LINKAGE.md) bridge.

### Context hard gate (operators)

Agent prompts and the Architecture skill **hard-refuse** to run without this Field Guide (and their stage-linked schemas/docs) in context. When you launch an agent: **attach/add** `migration-factory/docs/FIELD-GUIDE.md` plus that stage’s schemas (and TWO-FACTORIES / Architecture skill / PACK schema where linked). Do not expect agents to invent factory rules from memory.

**New to AI / new to the factory:** start with [MIGRATION-FOR-NEW-USERS.md](./MIGRATION-FOR-NEW-USERS.md). **Operators:** use [OPERATOR-RUNBOOK.md](./OPERATOR-RUNBOOK.md) (gate → skill routing table) and the [operator skill pack](../skills/operator/README.md). For whole-app “what’s left,” see **Portfolio inventory** below and `inventory/<APP_ID>/APP_MANIFEST.yaml`. **Experimental:** overnight estate candidate hunting via `prompts/estate-discovery-loop-v0.1.md` (inventory radar; not a default stage).

---

## What we are building

A **federated factory core**: shared controls, rules, skills, evidence discipline, and MANIFEST-as-SoT patterns. Domain and platform teams still own their units of work.

Two factories:

| Factory | Unit of work | Primary evidence | Primary SME |
| --- | --- | --- | --- |
| **Application modernisation** | Slice (related features) | Source code | Domain / product |
| **OS modernisation** | Host group / runtime footprint | CrowdStrike (etc.) host dumps | Platform / SRE |

Do **not** merge them into one Discovery agent.

---

## Core idea: buy the harness, build the workflows

Cursor provides the coding-agent harness (IDE, Cloud Agents, Skills, Hooks, MCP).  
We encode the operating playbooks as prompts, skills, schemas, and runbooks on top.

---

## Application modernisation factory

Pipeline (human gates between stages):

1. **Discovery** → as-is behaviour catalogue for a slice  
2. **Test generation** → characterization/contract **specs + harness stubs** that pin that behaviour  
3. **Test execution** → baseline on legacy  
4. **Architecture (skill)** → emits BOUND PACK.yaml Conversion must obey  
5. **Conversion** → bounded feature PRs following PACK  
6. **Verification** → parity evidence + sign-off pack  

### App Discovery (v0.2) — how to run it

**When:** legacy application (or large module); need honest as-is inventory before characterization tests or code migration.

**Slice:** migrate-able unit, e.g. "account opening", "NCD quote". On a microservice it may be the whole service. On a monolith it is a cluster of related features.

**Natural language alone is a seed, not the contract.** A human must **bind** candidates before deepening.

#### Inputs

| Input | Example |
| --- | --- |
| `SLICE_ID` | `account-opening` |
| `SLICE_SEED` | `account opening` |
| `SEED_ENTRYPOINTS` (optional) | `/api/accounts/open`, `AccountOpeningController` |
| `OUT_OF_SCOPE_HINTS` | `funding after open`, `card issuance` |

#### Run shape

**Phase A (map) — Plan Mode / interactive**  
1. Paste App Discovery prompt v0.2 + inputs  
2. Agent produces `CANDIDATES.md` + stub `MANIFEST.yaml` + `SME_BRIEF.md`  
3. Bind: accept / reject / defer (10–20 min)

**Phase B (deepen) — strong reasoning model; Cloud Agent OK**  
1. Behaviour cards only for **accepted** items  
2. Prefer a PR under `discovery/<SLICE_ID>/`  
3. SME signs → unlocks Test generation  

#### Do / don't

| Do | Don't |
| --- | --- |
| Scope to a slice | "Document the entire codebase" |
| Demand code citations | Accept undocumented claims |
| Use MANIFEST.yaml as SoT | Let Excel be the write-path |
| Keep behaviour as-is | Invent user-story narratives |
| Explicit `needs-SME` | Hide unknowns to look complete |
| One Cloud Agent per slice | One eternal estate crawler |

#### Done enough (App Discovery)

- [ ] `MANIFEST.yaml` ok  
- [ ] Behaviour cards for every accepted feature  
- [ ] `SME_BRIEF.md` signed  
- [ ] Deferred + blocked listed  
- [ ] CSV (if any) regenerated from MANIFEST  

**Handoff:** Test gen consumes accepted, documented behaviour cards only.

Prompt: `prompts/discovery-agent-v0.2.md`  
Schema: `schemas/discovery-manifest.schema.md`

---


### App Test generation (v0.1) — how to run it

**When:** Discovery pack for the slice is bound and documented; you need characterization/contract specs before running legacy.

**Job:** Discovery behaviour → **case specs + harness stubs + traceability** (step 1 only). **Locked 2026-08-04:** legacy run, golden recording, and “legacy green” are owned by the **Test execution** agent — not Test gen.

**Critical rule:** Happy/error/boundary only when Discovery evidence shows that branch. No invented expects. No spreadsheet-as-SoT.

#### Inputs

| Input | Example |
| --- | --- |
| `SLICE_ID` | `account-opening` |
| Discovery pack | `discovery/account-opening/` (MANIFEST + cards, human-bound) |
| `CARD_IDS[]` (optional batch) | `account-opening-001` … |
| Test stack hints | xUnit / pytest / … |

**Hard stop if:** unbound slice, `inferred`/`needs-SME` without waiver, no callable boundary (mark BLOCKED).

#### Run shape

**Phase A — case matrix (Plan Mode)**  
Propose cases from evidence only → `SME_BRIEF.md` → human approve/drop/defer.

**Phase B — specs + stubs (Cloud Agent OK)**  
Write `scenarios/…`, harness stubs, `TRACEABILITY.yaml`. Assert mode default `TO_BE_RECORDED`. PR under `testgen/<SLICE_ID>/`.

#### Do / don't

| Do | Don't |
| --- | --- |
| Trace every case to a behaviour card + citation | Invent ISTQB matrix / expects |
| DEFER speculative perf/security/E2E | Mandate those buckets on every feature |
| Batch ≤5–15 cards | “Every feature in the estate” |
| Hand off “ready to record” | Claim suite complete and green |
| CSV export from TRACEABILITY | Write Excel as master |

#### Done enough (Test gen)

- [ ] In-batch documented cards have ≥1 characterization case (or BLOCKED/DEFER explicit)
- [ ] TRACEABILITY complete; no silent gaps
- [ ] Harness stubs or BLOCKED reasons
- [ ] README checklist: ready for Test execution

**Handoff:** Test execution records goldens on running legacy; does not rediscover behaviour.

Prompt: `prompts/test-generation-agent-v0.1.md`  
Schema: `schemas/testgen-traceability.schema.md`

---

### App Test execution (v0.1) — how to run it

**Owns:** **RECORD** (default) / **REPLAY** against **legacy** — exercise → scrub → freeze goldens → replay green.  
**Does not own:** scenario invention (Test gen); production fixes (Conversion); **modern COMPARE** (Verification).

**Locked:** do not open-ask “legacy vs modern.” Orchestrator sets `MODE` + legacy `TARGET`.

#### Modes

| Mode | Target | Success |
| --- | --- | --- |
| RECORD (default) | Legacy | Goldens written + replay green |
| REPLAY | Legacy | Still green; no silent golden overwrite |
| COMPARE | Modern vs goldens | Verification stage |

#### Inputs

| Input | Example |
| --- | --- |
| `SLICE_ID` | `account-opening` |
| `MODE` | `RECORD` |
| Test gen pack | approved specs/stubs + TRACEABILITY |
| `CASE_IDS[]` | optional batch |

#### Do / don't

| Do | Don't |
| --- | --- |
| First run: actual → scrubbed golden | Invent “expected” before RECORD |
| Classify fails as harness/flake/discovery-gap/behavioural-delta; BLOCKED = can’t-run state | Mini-Jira severity theatre as primary output |
| Human-approve goldens before Conversion | Overwrite goldens on REPLAY without approve |
| results.json + REPORT.md SoT | Spreadsheet write-path |
| Seed/stub fixtures; same-seed → same-golden | Live prod deps |
| Treat legacy goldens as read-only for modern | Overwrite legacy goldens from modern runs |

#### Done enough (RECORD)

- [ ] In-batch cases `REPLAY_GREEN` or explicit BLOCKED list  
- [ ] Goldens + TRACEABILITY in git  
- [ ] Human golden-approval checklist  

**Handoff out:** frozen baseline unlocks Conversion; same goldens later used by Verification on modern.

Prompt: `prompts/test-execution-agent-v0.1.md`  
Schema: `schemas/testexec-results.schema.md`

---

### Architecture (skill, not a standing agent) — how to run it

**Job:** Produce/validate a machine-readable **target PACK** for the migration/slice. Different migrations can have different targets without new agents.

**Shape:** Skill owns schema + authoring/validation recipe. Optional Cloud Agent may *author* a messy DRAFT. Human architect sets `status: BOUND`. **Conversion only consumes BOUND packs** — it never “does architecture.”

**Order:** After characterization `REPLAY_GREEN` for in-scope cards → DRAFT pack → human BIND → Conversion. DRAFT before green is allowed; BIND is not.

#### Pack location

```text
architecture/<MIGRATION_ID>/
  PACK.yaml
  ADR/
  slice-overlays/<SLICE_ID>.yaml
```

#### Must be binding enough (required themes)

1. Target stack + patterns (+ forbidden / do-not-port)  
2. Contract authority (OAS etc.; sourced vs inferred; parity = characterization)  
3. Layout / naming map  
4. Edit surface (allow vs deny paths)  
5. Non-goals / deferrals  
6. Quality gates / done checks  
7. Known risks  
8. Meta: `pack_id`, `version`, `status`, `bound_by` / `bound_at`, change policy  

BOUND packs are **immutable**. Changes → new `versions/vN/` snapshot + SUPERSEDE + re-bind (never overwrite in place).

**BIND authority:** human architect (not chat approve). Prefer CODEOWNERS on `architecture/**` so not every committer can flip BOUND.

**Overlays:** if `slice-overlays/<SLICE_ID>.yaml` exists, **both** parent and overlay must be BOUND. Overlay wins only on allow-listed keys; missing keys inherit parent.

**CI:** validate PACK.yaml against `schemas/architecture-pack.schema.json`.

#### Do / don't

| Do | Don't |
| --- | --- |
| Per-migration packs (+ slice overlays) | One standing Architecture agent personality |
| Templates as example DRAFTs | Hardcode one vendor stack into the Skill |
| Human BIND in git (+ CODEOWNERS on `architecture/**`) | “Approved in chat” / agent self-bind |
| Retain `versions/vN/` on SUPERSEDE | Overwrite PACK history in place |
| Conversion hard-fail if not BOUND | Let Conversion freestyle / improve architecture |

Skill: `skills/architecture-pack/SKILL.md`  
Schema: `schemas/architecture-pack.schema.md` + `architecture-pack.schema.json`

---

### App Conversion (v0.1) — how to run it

**Job:** Iterate **documented** Discovery features and implement them on the target stack.  
**Law:** BOUND Architecture PACK. **Conversion executes the PACK; it does not design.** Conversion only consumes **BOUND** packs.

**Spine:** Characterize legacy (REPLAY_GREEN) → BIND pack → convert feature-by-feature → Verification compares modern to **same** goldens.  
**Defaults:** one PR per behaviour; legacy REPLAY safety OK (not modern COMPARE).

#### Hard stops (refuse)
- No BOUND effective pack (parent + overlay both BOUND if overlay exists)
- Edits outside `edit_surface` (do not touch `legacy/**` unless allow-listed)
- Net-new features / drive-by refactors / inventing stack not in PACK
- Changing characterization goldens to make modern green
- Expanding contracts unless pack allows
- Ambiguous `mapping_rules` → `ARCHITECTURE_GAP` (no guessing)

#### Inputs / batch default
- `FEATURE_IDS[]` default = all Discovery-bound documented features in the slice with `REPLAY_GREEN` (still slice-batched)

#### Plan mid-gate
Always emit a read-only plan artifact; human gate ≠ always block.
- **Mandatory:** first Conversion on this `pack_id@version`; any feature touches `known_risks`; `REQUIRE_PLAN_GATE=true`; plan proposes atomic multi-feature cluster
- **Optional/skip:** single-feature batch; pack already proven on sibling feature; no `known_risks`

#### Continue / abort policy
- `ARCHITECTURE_GAP` / unbound pack → **abort** batch
- `SCOPE_VIOLATION` on one feature → `BLOCKED` that feature; continue others only if no dependency; else `DEPENDENCY_BLOCKED`
- `CONVERSION_DEFECT` / `HARNESS_FAIL` / `FLAKE` → `BLOCKED` feature (retry once optional); never silently `DONE`
- Never next-feature past unresolved pack-level gap

#### Feature loop
1. Validate pack + REPLAY_GREEN batch  
2. For each documented feature: map via `mapping_rules`, implement inside edit surface, honour `known_risks`  
3. **One PR per behaviour** (default; atomic cluster only if pack treats as one): cite `pack_id@version` + ticket; checklist from pack quality gates; **Verification not run** checked by default  
4. Legacy REPLAY = best-effort; if harness unavailable set `LEGACY_REPLAY=SKIPPED` (do not hard-fail Conversion solely for that); do **not** run modern COMPARE  
5. Hook review skills: migration-correctness, security-triage, optional pack-compliance  

#### Failure notes
- `BLOCKED` = status on feature batch summary, not a failure class
- Keep `PRECONDITION_FAIL` (aka upstream precondition; often `DISCOVERY_GAP`)
- Keep `CONTRACT_RISK` for Conversion; reserve `BEHAVIOURAL_DELTA` for Testexec
- Batch summary required: `{feature_id, status: DONE|BLOCKED, code?, pr_url?}`

#### Done enough
- [ ] In-batch features converted or explicit `BLOCKED` (with taxonomy code)  
- [ ] Goldens unchanged  
- [ ] Pack citation on every PR  
- [ ] Emit `PARITY=UNVERIFIED` handoff; Verification runs COMPARE vs same goldens  
- [ ] Conversion never claims parity / COMPARE green — only Verification may set `PARITY=GREEN|FAIL`  

Prompt: `prompts/conversion-agent-v0.1.md`

---

### App Verification (v0.1) — how to run it

**Job:** Prove modern matches frozen characterization pins, and explain it in plain English with evidence.  
**One-liner:** Verification judges modern against frozen goldens; it never renegotiates the past.

**Not:** calling Test execution in RECORD on modern (would overwrite goldens). Reuse the same harness/cases/scrub/seeds with `MODE=COMPARE`, `TARGET=modern`, `GOLDENS=read-only`, `FORBID_RECORD=true`.

#### Three outputs
1. Modern COMPARE vs same legacy goldens → `PARITY=GREEN|FAIL` (or `BLOCKED` state)
2. Plain-English narrative (`HOW_IT_WORKS` / what we did) — cold-readable, cites `pack_id@version` + Conversion PRs; never substitutes for COMPARE
3. Human evidence pack (`REPORT.md` + `results.json` + diffs)

#### Entry / refuse
- Require Conversion handoff `PARITY=UNVERIFIED` + goldens + matching suite hashes
- Refuse golden edits, scope-narrowing to drop fails, quiet GREEN with failures, aspirational extras as parity

#### Artefacts
```text
verification/<SLICE_ID>/<RUN_ID>/
  HANDOFF.yaml
  PARITY.yaml
  narrative/
  evidence/
```

#### Done enough
- [ ] COMPARE completed for in-scope cases  
- [ ] `PARITY` set only by this stage  
- [ ] Narrative + evidence linked  
- [ ] FAIL leaves diffs + taxonomy; no golden rebase  

Prompt: `prompts/verification-agent-v0.1.md`

---

### Portfolio inventory (APP_MANIFEST) — how to run it

**Job:** Track **what’s known / what’s left** across the whole app while Discovery stays slice-scoped. Incomplete by default.

**SoT:** `inventory/<APP_ID>/APP_MANIFEST.yaml` (schema: `schemas/app-manifest.schema.json`).  
Slice `discovery/<slice>/MANIFEST.yaml` stays the deep bound truth. APP_MANIFEST is the **wide index** (surfaces + behaviours + pointers + rolled-up status). It does **not** duplicate TRACEABILITY case rows.

**Human glance surface:** generate `inventory/<APP_ID>/COVERAGE.md` from the APP_MANIFEST (histogram/counts, unscanned hints, weakest-case rollups).  
**Never hand-edit COVERAGE.md** — regenerate after inventory updates. Operators and SMEs read COVERAGE; agents write APP_MANIFEST.

**Anti-greenwash:**
- No completion %. Counts only (e.g. verified / converted / documented / known behaviours / unscanned surfaces).
- `legacy_green` only after Test execution RECORD → REPLAY_GREEN.
- `parity_green` / behaviour `status: verified` only after Verification COMPARE `PARITY=GREEN`.
- Under `WAIVED_PATHFINDER`: stop at `converted` + `parity: WAIVED`. Provisional modern tests never set green flags.

**Who writes:** Discovery adds surfaces/behaviours; Conversion/Verification **bump status + pointers** only; they do not invent inventory.  
**App complete?** Human residual gate only (`complete_bound` / `bound_incomplete_ok`). Agents never declare the residual empty.

**Operator SoT:** create and maintain `inventory/<APP_ID>/APP_MANIFEST.yaml` for each app (see `schemas/app-manifest.schema.md`). Generate `COVERAGE.md` from it; never hand-edit coverage.

Schema (human): `schemas/app-manifest.schema.md`  
Operator routing: [OPERATOR-RUNBOOK.md](./OPERATOR-RUNBOOK.md)

### Experimental: estate discovery loop (inventory radar)

**Status: EXPERIMENTAL.** Optional overnight Cloud Agent prompt. **Not** a required pipeline stage.

**Job:** Grow an honest app-level inventory of *candidate* slices/surfaces by looping slice-scoping + Discovery **Phase A only**. Updates `APP_MANIFEST` (`candidate`/`unknown`) and regenerates `COVERAGE.md`.

**Does not:** auto-bind, Phase B deepen, Test gen, RECORD, PACK BIND, Conversion, Verification, or claim "all slices found."

**Prompt:** `prompts/estate-discovery-loop-v0.1.md`

**Customer-room defaults:** allowlisted paths, factory-artefact write scope only, morning brief banner `PHASE A ONLY - UNBOUND CANDIDATES`.

**Completeness:** still a human residual gate. This loop is inventory progress, not migration progress.

Keep separate from `prompts/overnight-conductor-v0.1.md` (slice conveyor). Radar vs conveyor.


---

## OS modernisation factory

Pipeline (sketch; agents after Discovery still TBD):

1. **OS Discovery** → as-is runtime inventory for a host group (CrowdStrike-first)  
2. **Platform bind** → keep / replace / retire / defer / noise  
3. **Target architecture** → human-defined image / runtime standards  
4. **Conversion / uplift** → host or workload moves (owned by platform engineering)  
5. **Verification** → inventory parity + sign-off  

CrowdStrike (or equivalent EDR) dumps are the **default Discovery ingest**. LSEG can already export “everything running on a box.” Prefer those offline packs over live RTR from Cursor unless access is explicitly scoped.

Default dump families: process list, services, listeners, packages, OS version, scheduled tasks.  
**Not default:** full memory dumps (`memdump`) — forensics, not factory inventory.

### OS Discovery (v0.1) — how to run it

**When:** EOL OS pressure, unknown process trees, need host-group inventory before uplift.

#### Inputs

| Input | Example |
| --- | --- |
| `HOST_GROUP_ID` | `payments-prod-win2012` |
| `DUMP_BUNDLE` | path to CrowdStrike export pack |
| `CAPTURE_AS_OF` | dump timestamp / ticket id |
| `SEED_HOSTS` | optional hostname list |
| `KNOWN_NOISE` | AV/EDR process patterns |

#### Run shape

**Phase A** — parse dumps → candidates → `SME_BRIEF.md` → **stop for platform bind**  
**Phase B** — runtime cards for keep/replace/retire only  

#### Do / don't

| Do | Don't |
| --- | --- |
| Treat dumps as as-of snapshots | Assume dumps are live truth forever |
| Record parser coverage / missing fields | Invent columns the export lacked |
| Cluster same binary across hosts | One card per PID forever |
| Emit weak linkage stubs only | Claim process = feature without evidence |
| One Cloud Agent per host group | Merge into App Discovery MANIFEST |

Prompt: `prompts/os-discovery-agent-v0.1.md`  
Schema: `schemas/os-discovery-manifest.schema.md`  
Detail: [TWO-FACTORIES.md](./TWO-FACTORIES.md)

---

## Linkage (thin bridge)

After both factories have **bound** catalogues for an overlapping pathfinder, run Linkage:

- Confirmed / candidate links: feature ↔ runtime / host  
- Orphans both ways  
- Joint SME gate before sequencing Conversion waves  

See [LINKAGE.md](./LINKAGE.md). Linkage does not re-discover code or hosts.

---

## Pathfinder sequencing

1. Pick one business **slice** and one **host group** that (likely) serves it  
2. Run App Discovery and OS Discovery (parallel is fine)  
3. Bind both  
4. Run Linkage  
5. Decide which factory’s Conversion line owns the next PR / change wave  

---

## Models and surfaces (current recommendation)

| Step | Surface | Why |
| --- | --- | --- |
| Phase A taxonomy (app or OS) | Plan Mode on desktop | Human iteration on boundaries |
| Phase B deepen | Frontier / strong reasoning; Cloud Agent for large trees | Citations + durable PR output |
| Bulk parse / file walks | Faster model OK | Save frontier for ambiguity |

---

## Glossary

| Term | Meaning |
| --- | --- |
| Slice | Bounded set of related app features to migrate together |
| Host group | Bounded set of hosts / runtime footprint for OS Discovery |
| Behaviour card | As-is description of what code does, with evidence |
| Runtime card | As-is description of what runs on hosts, with dump evidence |
| MANIFEST | Machine-readable catalogue (source of truth) |
| Bind | Human accept/reject (app) or keep/replace/retire (OS) of candidates |
| Characterization | Tests that pin current behaviour, not desired correctness |
| Factory core | Shared skills, rules, agents, controls |
| Linkage | Thin bridge between app and OS catalogues |
| Pathfinder | First slice + host group that configures the lines |
| Characterization case | Spec that pins observed behaviour; expects usually TO_BE_RECORDED until execution |
| TRACEABILITY | behaviour_id → case_id → files map (Test gen SoT companion) |
| Test execution | Stage that runs harness on legacy and freezes goldens |
| Architecture PACK | Machine-readable to-be contract (`PACK.yaml`); Conversion hard-requires BOUND |
| Architecture skill | Recipe to produce/validate PACK; not a standing agent |
| Conversion | Implements documented features under BOUND PACK; does not design |
| Verification | Modern COMPARE vs same goldens; sets PARITY; narrative + evidence |
| CrowdStrike dump pack | Offline host inventory export used as OS Discovery input |
| APP_MANIFEST | App-level portfolio inventory (wide index); incomplete until human residual gate |
| COVERAGE.md | Generated human glance of APP_MANIFEST counts/status — never hand-edit |
| Operator runbook | One-page gate → skill routing table for humans running the factory |
| WAIVED_PATHFINDER | Explicit waiver when legacy RECORD cannot run; Conversion may proceed; no PARITY claim |
| Estate discovery loop | **EXPERIMENTAL** overnight inventory radar (Phase A candidates only); never auto-bind or claim completeness |
| Overnight conductor | **EXPERIMENTAL** slice conveyor prompt (provisional overnight charter); separate from estate discovery |

---

## Change log

| Date | Change |
| --- | --- |
| 2026-08-05 | Experimental: estate-discovery-loop overnight prompt (inventory radar; Phase A only; no auto-bind). Documented separately from overnight conductor. |
| 2026-08-04 | Scrubbed pathfinder seed: operators point at inventory/<APP_ID>/APP_MANIFEST.yaml only (no committed customer seed). |
| 2026-08-04 | Added MIGRATION-FOR-NEW-USERS.md (junior / new-to-AI walkthrough of each stage). |
| 2026-08-04 | Operator runbook + skill pack (gate routing); APP_MANIFEST portfolio inventory + COVERAGE.md (anti-greenwash; human residual gate). |
| 2026-08-04 | Operator day-1 skills expanded: waive-characterization, matrix-gate-reply, deepen-phase-b, conversion-pr, verify-parity. |
| 2026-08-03 | Initial guide. App Discovery v0.2; OS deferred. |
| 2026-08-03 | Split into two factories. OS Discovery v0.1 (CrowdStrike-first). Linkage bridge. |
| 2026-08-04 | App Test generation v0.1: characterization specs + harness stubs; goldens owned by Test execution. |
| 2026-08-04 | Stage split locked: Test gen = specs/stubs only; Test execution owns legacy green + record goldens. |
| 2026-08-04 | App Test execution v0.1: legacy RECORD/REPLAY; COMPARE deferred to Verification; BLOCKED taxonomy. |
| 2026-08-04 | Test execution: same-seed→same-golden; never overwrite legacy goldens from modern. |
| 2026-08-04 | Test execution enum: BLOCKED = state; failure classes HARNESS_FAIL\|FLAKE\|DISCOVERY_GAP\|BEHAVIOURAL_DELTA. |
| 2026-08-04 | Architecture as skill + PACK.yaml (BOUND immutable); no standing Architecture agent. |
| 2026-08-04 | Architecture nits: JSON Schema, overlay dual-BIND, inherit/override rules, CODEOWNERS, no agent self-bind, versions/ retention. |
| 2026-08-04 | Conversion agent v0.1: feature loop under BOUND PACK; goldens immutable; handoff to Verification. |
| 2026-08-04 | Conversion v0.1: one-PR-per-behaviour default; legacy REPLAY safety only; review skills stacked with Bugbot. |
| 2026-08-04 | Conversion: plan mid-gate mandatory/optional; Continue/abort policy; BLOCKED=status; DEPENDENCY_BLOCKED; PARITY=UNVERIFIED handoff; LEGACY_REPLAY=SKIPPED. |
| 2026-08-04 | Context hard gate: all agent prompts + Architecture skill refuse without Field Guide (+ stage-linked docs) in context. |
| 2026-08-04 | Field Guide customer-facing scrub: removed internal attribution language. |

---

## Open decisions

- Estate discovery loop promotion criteria (when to graduate from EXPERIMENTAL)  
- Whether overnight conductor stays EXPERIMENTAL alongside it  
- APP_MANIFEST ↔ Discovery writer contract (when Discovery must upsert surfaces)  
- COVERAGE.md generator skill / make target  
- Exact FEATURE_ID / RUNTIME_ID schemes and dedupe rules  
- Example BOUND packs / templates under architecture/_templates/  
- Concrete CrowdStrike export recipe LSEG will standardise (column set)  
- OS Conversion agent scope (image bake vs lift-and-shift vs replatform) — later  
- Who owns confirmed linkage records (platform vs domain vs joint CAB)  
- Whether contract tests (OAS) sit beside characterization or inside the same agent  
