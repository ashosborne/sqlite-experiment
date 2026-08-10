# Operator runbook (App modernisation) — routing table

One-page map for humans running the factory. **Not** a Discovery tutorial — stage how-tos live in the [Field Guide](./FIELD-GUIDE.md).

**Context hard gate:** before any agent or operator skill runs, attach `docs/FIELD-GUIDE.md` + that stage’s schemas/prompts. Agents refuse without them.

**EXPERIMENTAL estate discovery:** optional overnight inventory radar (`prompts/estate-discovery-loop-v0.1.md`). Phase A candidates only; never auto-bind; never claim completeness. Separate from the overnight Conversion conductor.

**Portfolio inventory:** SoT is `inventory/<APP_ID>/APP_MANIFEST.yaml` (schema: `schemas/app-manifest.schema.json`). **Generate** `inventory/<APP_ID>/COVERAGE.md` from the manifest. Operators **read** it; **never hand-edit** `COVERAGE.md`. See Field Guide **Portfolio inventory** + `schemas/app-manifest.schema.md`.

---

## Where artefacts land

| Stage / artefact | Path |
| --- | --- |
| Slice inputs (seeds) | Operator brief / ticket; echoed in Discovery prompt |
| Discovery Phase A | `discovery/<SLICE_ID>/CANDIDATES.md`, `MANIFEST.yaml` (stub), `SME_BRIEF.md` |
| Discovery Phase B | `discovery/<SLICE_ID>/features/<FEATURE_ID>.md` (+ bound MANIFEST) |
| Test gen | `testgen/<SLICE_ID>/` — scenarios, stubs, `TRACEABILITY.yaml`, `SME_BRIEF.md` |
| Test exec | `tests/characterization/<SLICE_ID>/…`, goldens `*.approved.*`, `results.json`, `REPORT.md` |
| Waiver (no legacy run) | `testexec/<SLICE_ID>/WAIVED_*.md` + ADR under `architecture/` or `discovery/` (see skill) |
| Architecture | `architecture/<MIGRATION_ID>/PACK.yaml`, `ADR/`, `slice-overlays/<SLICE_ID>.yaml` |
| Conversion | Feature PRs citing `pack_id@version`; batch summary |
| Verification | `verification/<SLICE_ID>/<RUN_ID>/` — `PARITY.yaml`, narrative, evidence |
| Portfolio inventory | `inventory/<APP_ID>/APP_MANIFEST.yaml` (SoT); generated `inventory/<APP_ID>/COVERAGE.md` (**read-only**) |

---

## Routing table

| Gate / stuck moment | Skill | Human says (one line) | Next artefact |
| --- | --- | --- | --- |
| Starting a pathfinder; fuzzy “whole module” | `slice-scoping` | “Scope slice `SLICE_ID` with these seeds / out-of-scope.” | Bound slice inputs ready for Discovery Phase A |
| Discovery Phase A done; candidates waiting | `record-bind` | “Bind: accept / reject / defer per candidate.” | Updated `MANIFEST.yaml` + bind record; unlock deepen |
| Bind done; need Phase B deepen prompt | `deepen-phase-b` | “Deepen accepted only for `SLICE_ID`.” | Paste-ready Phase B prompt → behaviour cards PR |
| Test gen Phase A matrix waiting | `matrix-gate-reply` | “Approve / drop / defer these case rows.” | Paste-ready matrix reply → Phase B specs/stubs |
| Any mid-gate (golden approve, Conversion plan, SME sign-off) | `compose-gate-reply` | “Approve / waive / block with reason.” | Durable gate reply (chat or PR comment) |
| Unsure which stage prompt to paste next | `compose-stage-prompt` | “Compose next-stage run for `STAGE` + `SLICE_ID`.” | Paste-ready agent prompt + required attachments |
| No legacy runtime/license; cannot RECORD | `waive-characterization` | “Waive Test Exec under `WAIVED_PATHFINDER`.” | `WAIVED_*` + ADR; Conversion may proceed; **no PARITY claim** |
| Need to-be target before Conversion | `architecture-pack-author` | “Author DRAFT PACK for `MIGRATION_ID`.” | `PACK.yaml` `status: DRAFT` (+ ADR) |
| DRAFT pack ready; architect will BIND | `architecture-pack-bind` | “BIND checklist for pack@version (human only).” | Git commit: `status: BOUND` + `bound_by` / `bound_at` |
| Conversion feature about to PR | `conversion-pr` | “Compose PR body for feature(s).” | PR description: pack@version, edit surface, risks |
| Modern built; need parity proof | `verify-parity` | “Run Verification COMPARE for `SLICE_ID`.” | `PARITY.yaml` + evidence pack (not chat greening) |
| **EXPERIMENTAL:** overnight estate candidate hunt | `prompts/estate-discovery-loop-v0.1.md` | "Run estate discovery under allowlist; Phase A only." | APP_MANIFEST + COVERAGE delta; MORNING_BRIEF; nothing bound |
| Unsure what’s left across the app | _(read)_ `COVERAGE.md` | “Show coverage for `APP_ID`.” | Glance only — residual completeness is a **human** gate on APP_MANIFEST |
| Want Jira cards from behaviour cards | _(future)_ `jira-cards-mcp` | — | See `skills/operator/jira-cards-mcp/FUTURE.md` — do not build MCP yet |

---

## Refuse / don’t-proceed rows

| Situation | Do **not** | Instead |
| --- | --- | --- |
| No Field Guide / schemas in context | Launch agent anyway | Attach Field Guide + stage schemas; hard-refuse is correct |
| Discovery candidates unbound | Deepen Phase B / invent accepts | `record-bind` first |
| Cards `inferred` / `needs-SME` without waiver | Test gen Phase B or RECORD | SME resolve, waive explicitly, or drop from batch |
| No `REPLAY_GREEN` and no `WAIVED_*` | BIND Architecture pack; start Conversion | Run Test Exec **or** `waive-characterization` |
| Chat “looks good” on architecture | Treat as BOUND | `architecture-pack-bind` → git `status: BOUND` (+ CODEOWNERS) |
| Claim overnight found all slices / auto-bind from estate loop | Treat COVERAGE growth as migration done | Morning bind only; residual stays human |
| Hand-edit `COVERAGE.md` | “Fix” coverage in markdown | Update `APP_MANIFEST.yaml` (or re-run inventory writers) and **regenerate** COVERAGE |
| Waiver path used | Claim `PARITY=GREEN` or skip Verification forever | Convert under waiver; Verification still required later for parity claims |
| Conversion without BOUND pack | “Just implement Spring-style…” | `architecture-pack-author` → human BIND |
| RECORD on modern / rewrite goldens from modern | “Make tests pass on new stack” | Verification `MODE=COMPARE`, goldens read-only |
| Agent self-binds MANIFEST / PACK | Accept agent-flipped `BOUND` | Human bind skills only |

---

## Stage spine (reminder)

```text
Discovery → Test Gen → Test Exec ─┬→ Architecture → Conversion → Verification
                                  │
                                  └→ WAIVED_PATHFINDER (skill: waive-characterization)
                                       → Architecture → Conversion
                                       → Verification still owns any future PARITY claim
```

Exit criteria and stage tutorials: [FIELD-GUIDE.md](./FIELD-GUIDE.md).  
Operator skill pack: [`../skills/operator/README.md`](../skills/operator/README.md).

---

## Change log

| Date | Change |
| --- | --- |
| 2026-08-05 | Experimental estate-discovery-loop documented in routing table (inventory radar). |
| 2026-08-04 | Scrubbed pathfinder seed reference; SoT is inventory/<APP_ID>/APP_MANIFEST.yaml only. |
| 2026-08-04 | Initial operator routing table + skill pack links; WAIVED_PATHFINDER fork; APP_MANIFEST/COVERAGE.md routing (read-only coverage). |
