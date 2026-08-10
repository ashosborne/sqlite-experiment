# Operator skill pack

Skills that help **humans** run the App modernisation factory without depending on ad-hoc chat coaching.  
Stage agent prompts stay under `prompts/`; Architecture PACK authoring/validation recipe also lives at `skills/architecture-pack/` (factory skill). These operator skills **compose gates, binds, waivers, and paste-ready prompts**.

**Always attach:** `docs/FIELD-GUIDE.md` (+ stage schemas). See [OPERATOR-RUNBOOK.md](../../docs/OPERATOR-RUNBOOK.md) for the routing table.

## Day-1 skills

| Skill id | Use when |
| --- | --- |
| [`slice-scoping`](./slice-scoping/SKILL.md) | Define `SLICE_ID`, seeds, entrypoints, out-of-scope |
| [`record-bind`](./record-bind/SKILL.md) | Capture Discovery (or OS) bind decisions into MANIFEST |
| [`deepen-phase-b`](./deepen-phase-b/SKILL.md) | After bind: compose Discovery Phase B deepen prompt |
| [`matrix-gate-reply`](./matrix-gate-reply/SKILL.md) | Paste-ready Test gen matrix approve/drop/defer replies |
| [`compose-gate-reply`](./compose-gate-reply/SKILL.md) | Generic human gate replies (golden approve, plan gate, SME sign-off) |
| [`compose-stage-prompt`](./compose-stage-prompt/SKILL.md) | Compose next-stage agent run prompts + attachment list |
| [`waive-characterization`](./waive-characterization/SKILL.md) | `WAIVED_*` when Test Exec cannot run (e.g. no legacy runtime / license / harness) |
| [`architecture-pack-author`](./architecture-pack-author/SKILL.md) | Compose PACK authoring prompt / DRAFT recipe handoff |
| [`architecture-pack-bind`](./architecture-pack-bind/SKILL.md) | Human BIND checklist (nits + BIND); never agent self-bind |
| [`conversion-pr`](./conversion-pr/SKILL.md) | Conversion PR body: pack@version, edit surface, residual risks |
| [`verify-parity`](./verify-parity/SKILL.md) | Route parity to Verification agent — do not invent COMPARE in chat |

## Future

| Id | Notes |
| --- | --- |
| [`jira-cards-mcp`](./jira-cards-mcp/FUTURE.md) | Optional Atlassian MCP later; **not built**; out of day-1 scope. |

## Conventions

- YAML frontmatter: `name`, `description` (when-to-use).
- Customer-facing language (no internal attribution).
- Prefer paste-ready templates; write durable decisions into git artefacts, not only chat.
- SoT is `APP_MANIFEST.yaml`; **generate** `COVERAGE.md` from it. Never hand-edit `COVERAGE.md` (read-only for humans).

## Experimental (not a skill)

- `prompts/estate-discovery-loop-v0.1.md` — overnight **inventory radar** (Phase A candidates only). Not operator-skill gated; see Field Guide.
