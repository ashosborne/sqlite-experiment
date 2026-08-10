# Estate discovery loop (v0.1.1) - overnight surface hunter

> **EXPERIMENTAL.** Inventory radar only. Not a default factory stage. Does not bind, deepen, test, or convert. Never claims estate completeness.

_Folded FE + Terry + CTO constraints (allowlist, characterizability, method coverage, idempotent resume, radar vs conveyor)._

Paste as the **entire** Cloud Agent prompt for a long overnight run.
Purpose: grow an honest app-level inventory of **candidate slices / surfaces** on a large legacy tree.
This is **not** Conversion. This is **not** "find everything and bind it."

Attach / ensure in-repo: `migration-factory/docs/FIELD-GUIDE.md`, `OPERATOR-RUNBOOK.md`,
`prompts/discovery-agent-v0.2.md`, `skills/operator/slice-scoping/SKILL.md`,
`schemas/app-manifest.schema.md` (+ `.json`), `schemas/discovery-manifest.schema.md`.

**Law:** Field Guide + schemas win. Never invent factory rules.

---

## Operator charter (edit before launch)

```yaml
OPERATOR: Ash Osborne
APP_ID: cargotracker          # change per estate
REPO_ROOT: .
FACTORY_ROOT: migration-factory
MAX_ITERATIONS: 12            # hard cap for overnight
MAX_NEW_SEEDS_PER_ITER: 1     # one Discovery Phase A per loop
PHASE_B: false                # MUST stay false unless morning charter changes
AUTO_BIND: false              # MUST stay false - candidates only
AUTO_ACCEPT: false
ALLOW_CONVERSION: false
STOP_WHEN_NO_NEW_SURFACES: 2  # consecutive iters with zero new surfaces/hints
INITIAL_SEEDS: []             # optional ordered seeds; prefer these first
ALLOWLIST_PATHS: []           # REQUIRED in customer rooms: dirs/modules/repos allowed to scan
OUT_OF_SCOPE_HINTS: []        # dirs/modules to never seed
MAX_FILES_TOUCHED: 5000       # soft cap; stop iteration if exceeded
MAX_RUNTIME_HINT_HOURS: 8
MAX_NEW_CANDIDATES: 40
MAX_SLICES_PHASE_A: 12
WRITE_SCOPE: factory-artefacts-only  # discovery/**, inventory/**, overnight/** only
NO_COMMITS_TO_DEFAULT_BRANCH: true
COMMIT_AS: estate-discovery-loop
```

---

## What success means

Success = a richer, still-incomplete `inventory/<APP_ID>/APP_MANIFEST.yaml` + regenerated `COVERAGE.md`,
plus `overnight/MORNING_BRIEF.md` listing proposed next human binds.

Failure modes to avoid: claiming completeness, auto-accepting features, deepening Phase B,
running Test gen / Architecture / Conversion, or minting fake `complete_bound`.

---

## Context hard gate (once)

1. Read Field Guide (App Discovery + Portfolio inventory).
2. Read Operator runbook + slice-scoping skill + Discovery Phase A prompt + APP_MANIFEST schema.
3. Identify repo identity (remote, HEAD SHA). Write `overnight/CONTEXT_GATE.md`.
4. If factory pack missing: write `overnight/BLOCKED.md` and STOP.

---

## Non-negotiables

- Write surfaces/behaviours only as `candidate` or `unknown` (or `deferred`/`rejected` **recommendations** in prose, not MANIFEST accepts).
- Never flip Discovery MANIFEST features to `accepted` / `documented`.
- Never run Phase B, Test gen, Test exec, Architecture BIND, or Conversion.
- Never set APP_MANIFEST `status: complete_bound` or `completeness` to anything that implies done.
- No completion percentages. Counts / histograms only.
- Prefer evidence citations (paths) over vibes.
- Keep `overnight/JOURNAL.md` every iteration.

## Customer-room safety (Field)

- **Allowlist:** If `ALLOWLIST_PATHS` is non-empty, do not seed or deep-scan outside it. Empty allowlist is OK only on synthetic/demo repos; in customer estates refuse to run without allowlist.
- **Write scope:** Only `discovery/**`, `inventory/**`, `overnight/**` (and factory pack updates if present). No edits to legacy application source. No pushes to customer default branches.
- **Label every finding** `confidence: observed-in-code` vs `inferred` (inferred never auto-promoted).
- **Secrets:** skip `.env`, keystores, vault paths, `*.pem` / credential files; redact tokens in journals.
- **Caps:** honour MAX_ITERATIONS, MAX_FILES_TOUCHED, MAX_RUNTIME_HINT_HOURS; stop on auth walls.
- **Abort:** if `overnight/stop.txt` appears, halt cleanly and write MORNING_BRIEF from progress so far.

---

## Idempotency / resume

- Resume from existing `APP_MANIFEST` and prior `discovery/<SLICE_ID>/` artefacts.
- Merge by `surface_id` / `locator` / `behaviour_id`. Do **not** thrash or downgrade human-set statuses (`accepted`, `documented`, `deferred`, `rejected`, `converted`, ...).
- Re-runs are additive delta jobs, not certificates of completeness.
- Prefer breadth of Phase A candidates over deep cards (Phase B is morning work).

## Separation of concerns

This prompt is **inventory radar** only. Do not smuggle in Conversion conductor behaviour (PACK BIND, Conversion, Verification). Keep those in `overnight-conductor-v0.1.md` / stage prompts.

## Loop algorithm

### Bootstrap (iter 0)
- **Cheap structural index first:** OAS/RAML, listeners, flows, schedulers, message endpoints, batch/CLI mains - each with locator + evidence path.
- Ensure `inventory/<APP_ID>/APP_MANIFEST.yaml` exists (create stub if needed).
- Run a **read-only entrypoint sweep**: HTTP/REST mappings, message listeners/consumers,
  schedulers/batch, main CLIs, obvious UI entry (note UI as adapter candidates, do not prioritize).
- Populate `unscanned_hints` with dirs/modules/entrypoint clusters not yet seeded.
- Regenerate `COVERAGE.md`.

### Each iteration (1..MAX_ITERATIONS)

1. **Pick next seed** (in order):
   - Operator `INITIAL_SEEDS` not yet tried
   - Else highest-value item from `unscanned_hints` (prefer: inbound HTTP, then messaging, then batch)
   - Else STOP with residual register
2. **Slice-scope** using `slice-scoping` skill: propose `SLICE_ID`, seed text, out-of-scope, entrypoints.
   Write `overnight/seeds/<SLICE_ID>.md`. Refuse mega-slices ("whole module"). Prefer thin, characterizable seams.
3. **Discovery Phase A only** for that slice (Discovery agent prompt).
   Outputs under `discovery/<SLICE_ID>/`: `CANDIDATES.md`, stub `MANIFEST.yaml` (all `candidate`), `SME_BRIEF.md`.
4. **Upsert APP_MANIFEST**: add/update surfaces + behaviour stubs as `candidate`/`unknown`;
   append `scanned_seeds`; shrink/adjust `unscanned_hints` honestly (do not delete a hint unless scanned or explicitly out-of-scope).
5. **Regenerate COVERAGE.md**.
6. **Journal** what was added, skipped, and why.
7. **Stop checks:**
   - Hit MAX_ITERATIONS
   - STOP_WHEN_NO_NEW_SURFACES consecutive iters with no new surface_id/behaviour_id and no new hints
   - Blocker (build/tooling) recorded in BLOCKED.md

### End of run
Write `overnight/MORNING_BRIEF.md` (one page):
1. Banner: `PHASE A ONLY - UNBOUND CANDIDATES`
2. Counts: seeds scanned / new candidate features / deferred recommendations / errors
3. Top candidates table: id, slice, endpoint/flow hint, confidence (observed vs inferred), evidence path
4. APP_MANIFEST / COVERAGE delta (what grew)
5. Remaining unscanned_hints + `% allowlist seeds scanned` if allowlist set (honesty metric, not completeness)
6. Recommended human priority order for bind (not auto-bound)
7. Suspected blind spots (config-only routes, reflection, vendor buses, prod cron)
8. Explicit: Did **not** bind, PACK, RECORD, or Convert
9. Explicit line: `completeness: incomplete` - human residual gate required
10. Closing ask: "Bind which slice IDs today?"

---

## Characterizability (Terry)

Optimize proposed slices for **later characterization**, not for "easy Conversion":
- Prefer seams with a callable boundary (HTTP, message contract, batch entry) and observable outcomes.
- Flag fire-and-forget / async chains so humans can split cards (do not smash into one mega-slice silently).
- Over-coarse slices produce unreviewable matrices later; over-fine slices produce fake TRACEABILITY theatre. Prefer mid-size, evidence-backed seams.
- Soft / inferred cards stay `inferred` - never unlock Test gen from this loop.
- `DEFER` / `SKIP_WITH_REASON` beats thin invented candidates.
- Unscannable areas become `unscanned_hints` (or skipped-with-reason rows), not fake feature cards.

## Method coverage checklist (each full pass)

Track whether this pass touched, skipped, or could not see:
- HTTP/REST (and SOAP if present)
- Message listeners / queues / topics
- Schedulers / batch / CLI mains
- Gateway/RAML/OpenAPI specs if present
- UI-only adapters (note; deprioritize when a service boundary exists)

Emit `overnight/METHOD_COVERAGE.md` + set APP_MANIFEST note `estate_scan: partial` until checklist is addressed. Zero-diff re-scan ≠ complete. Banner `ESTATE_SCAN_INCOMPLETE` in MORNING_BRIEF if checklist unmet.

Log **what was not searched** (auth walls, binaries, missing repos, skipped allowlist holes) as first-class output in MORNING_BRIEF.

## Anti-patterns (refuse)

- Re-seeding the same package without new entrypoints
- Accepting candidates because "looks core"
- Declaring the estate fully sliced / "% complete" / "all slices found"
- Unlocking Test gen or matrix from overnight candidates
- Silent deletion of human accepted/deferred/rejected rows
- Treating COVERAGE growth as migration progress (inventory progress only)
- Collapsing async chains into one slice without noting seams (flag in SME_BRIEF; still Phase A only)
- Treating UI pages as primary slices when a service/REST boundary exists

---

## Completeness claims

**Never claim "we found all slices."** Safe claim only:

> Of the allowlisted seeds scanned overnight, these candidate slices were proposed; APP_MANIFEST updated; nothing bound.
>
> Reliable claim shape: all *seeded and scanned* surfaces have candidate rows; residual = unscanned_hints + unknown surfaces. Never "all slices in the estate."

Estate discovery covers **scanned seeds**, not the enterprise. Unlisted repos, generated code, partner APIs, runtime-only flows, feature flags, and twin copy-paste paths remain residual until a human says otherwise.

## Morning human

1. Read MORNING_BRIEF + COVERAGE.
2. Pick slices to bind via `record-bind`.
3. Run Phase B deepen only on accepted.
4. Re-run this loop later with updated OUT_OF_SCOPE / new dumps if residual remains.

