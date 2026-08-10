---
name: Architecture pack
description: Use when defining or validating the to-be target for an App modernisation slice — emits a machine-readable BOUND-able PACK.yaml that Conversion must obey (not a standing Architecture agent).
---

# Architecture pack (factory skill)

You help produce or validate an **architecture PACK** for the App modernisation factory.

This is **not** a standing Architecture agent and **not** a greenfield redesign exercise.  
Output is a **versioned decision artifact** Conversion hard-requires. Chat guidance without a PACK file is insufficient.

## Context hard gate (mandatory)

Before producing or validating a PACK, **read**:
1. `migration-factory/docs/FIELD-GUIDE.md` (factory rhythm, stage split, glossary)
2. Field Guide **Architecture (skill)** section (DRAFT → human BOUND; Conversion never authors architecture)
3. `migration-factory/schemas/architecture-pack.schema.md` and `architecture-pack.schema.json`

**Refuse to run** if the Field Guide path is missing / unreadable. Do not invent factory rules or PACK shape from memory.

## When to use

- Pathfinder / slice has Discovery bound + Test execution `REPLAY_GREEN` (or operator explicitly drafts DRAFT early — still cannot BIND until REPLAY_GREEN)
- Operator needs a to-be target that may differ per migration (.NET 8 strangler vs other stacks)
- Conversion is about to run and needs a `BOUND` pack

## Non-goals

- Do not invent product features, UX upgrades, or estate-wide re-platform visions
- Do not hardcode one vendor stack into the Skill personality — packs are per migration; templates are examples only
- Do not let Conversion “improve” architecture mid-flight — that requires a new pack version + re-bind
- Do not BIND in chat; BIND = git `status: BOUND` + human `bound_by` / `bound_at`

## Preconditions

Prefer (required before BIND):

- Field Guide path (default `migration-factory/docs/FIELD-GUIDE.md`) — **required before any PACK authoring/validation**
- `discovery/<SLICE_ID>/` bound MANIFEST + behaviour cards
- Characterization TRACEABILITY with in-scope cases `REPLAY_GREEN`
- Operator intent: target stack, style, non-goals (can be sparse; Skill structures them)

If REPLAY_GREEN missing: you may author `status: DRAFT` only; refuse to mark BOUND.

## Produce pack (write path)

```text
architecture/<MIGRATION_ID>/
  PACK.yaml                 # machine contract (required)
  ADR/*.md                  # short rationale (recommended)
  diagrams/                 # optional
  slice-overlays/<SLICE_ID>.yaml   # deltas only
  templates/                # optional reference_skeleton pointers
  versions/v<N>/PACK.yaml   # retained on SUPERSEDE (do not overwrite in place)
```

1. Load schema; gather intent + Discovery/Testexec evidence links.
2. Fill **all required** PACK fields (see schema). Empty required keys = invalid.
3. Add `mapping_rules` (legacy pattern → target pattern) Conversion will follow.
4. Fill `forbidden` + `non_goals` aggressively — this is how freestyle dies.
5. Set `status: DRAFT`, `version`, `pack_id`.
6. Write `ADR/0001-...md` explaining choices in plain language (no coaching theatre).
7. Stop for **human BIND**. **Never** set `status: BOUND` from this Skill path — only a human (or a dedicated human-driven bind checklist commit) flips DRAFT→BOUND with `bound_by` / `bound_at`. Agents may author DRAFT and validate only.

## Validate pack

When asked to validate:

- Schema-valid?
- `status` appropriate for next step?
- Evidence links present for BIND?
- Overlays conflict with parent?
- Quality gates reference real TRACEABILITY / test commands?
- Report `ARCHITECTURE_GAP` list; do not invent fills for ambiguous keys.

## Templates

Org may keep example packs (e.g. `.NET 8 Minimal API strangler`). Copy as starting DRAFT, then adapt — never assume the template is law unless BIND says so.

## Versioning / SUPERSEDE

BOUND packs are immutable. To change: write `versions/v<N>/PACK.yaml` (retain prior), bump `version`, set old pack `SUPERSEDED`, human re-BINDs the new DRAFT. Never overwrite `PACK.yaml` history in place — `pack_id@version` citations must stay resolvable.

## Handoff to Conversion

Conversion may start only when:

- Effective pack for slice (`PACK.yaml` + overlay) has `status: BOUND`
- Preconditions in `evidence` satisfied
- Task cites `pack_id@version`

If Conversion asks you to reinterpret a thin pack: emit `ARCHITECTURE_GAP` and stop — do not freestyle.
