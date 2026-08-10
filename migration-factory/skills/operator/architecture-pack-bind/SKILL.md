---
name: Architecture PACK bind
description: Use when a human architect binds a DRAFT PACK (and overlay) after review — BIND checklist + git BOUND flip with bound_by/bound_at; never agent self-bind; require WAIVED_* or REPLAY_GREEN.
---

# Architecture PACK bind (operator skill)

Human-only path from **DRAFT → BOUND**. Agents may author DRAFT; **only a human** flips `status: BOUND`.

## Context hard gate

- `docs/FIELD-GUIDE.md` (Architecture section)
- `skills/architecture-pack/SKILL.md` (versioning / SUPERSEDE)
- `schemas/architecture-pack.schema.md` + `.json`

Refuse if Field Guide or architecture-pack skill/schema are missing.

## When to use

- DRAFT `PACK.yaml` reviewed by an architect / CODEOWNERS
- In-scope `REPLAY_GREEN` **or** cited `WAIVED_*` on file
- Overlay present → parent pack **and** overlay must both BIND

## Preconditions (refuse BIND if missing)

- [ ] Schema-valid PACK (+ overlay if any)
- [ ] Required themes filled (stack, contracts, layout, `edit_surface`, `mapping_rules`, non_goals, quality_gates, risks, meta)
- [ ] Evidence links present (Discovery / TRACEABILITY / Test Exec or waiver)
- [ ] Characterization green **or** waiver artefact cited (`WAIVED_*`)
- [ ] CODEOWNERS / architect authority respected
- [ ] Human (not agent) will set `bound_by` / `bound_at`

## Nits + BIND recipe

1. List nits: **blocking** vs **non-blocking**.
2. Blocking nits → return to `architecture-pack-author` (new DRAFT commit); do **not** BIND.
3. If clean / nits-only: human commit sets:
   - `status: BOUND`
   - `bound_by`, `bound_at`
   - Same for overlay if present
4. Remind immutability / SUPERSEDE: BOUND packs must not be overwritten in place. Later changes → `versions/vN/` snapshot + `SUPERSEDED` + new DRAFT + **re-bind**. See `skills/architecture-pack/SKILL.md` § Versioning / SUPERSEDE and `schemas/architecture-pack.schema.md`.

## Paste-ready BIND checklist

```text
## PACK BIND checklist — architecture/<MIGRATION_ID>/

pack_id: <id>
version: <semver or vN>
overlay: slice-overlays/<SLICE_ID>.yaml | none

Blocking nits: (none | list)
Non-blocking nits: <…>

Evidence:
- REPLAY_GREEN case set: [<ids>] OR
- Waiver: testexec/<SLICE_ID>/WAIVED_<CODE>.md

Decision: BIND
bound_by: <architect name>
bound_at: <YYYY-MM-DDTHH:MM:SSZ or local+TZ>

Git actions:
1. Set status: BOUND on PACK.yaml (+ overlay)
2. Set bound_by / bound_at (human identity — not an agent)
3. Commit with CODEOWNERS-approved reviewer if required
4. Unlock Conversion citing pack_id@version

SUPERSEDE reminder: future edits → versions/vN/ + SUPERSEDE + re-bind
(see skills/architecture-pack/SKILL.md + schemas/architecture-pack.schema.md)

FORBIDDEN: agent or chat self-bind without this checklist + git meta
```

## Refuse

- Agent or chat self-bind (no human `bound_by` / git meta)
- BIND without `REPLAY_GREEN` **and** without cited `WAIVED_*`
- BIND with empty `mapping_rules` / `edit_surface`
- Overwriting a BOUND pack in place instead of SUPERSEDE + re-bind
