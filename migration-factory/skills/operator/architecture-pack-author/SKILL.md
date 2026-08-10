---
name: Architecture PACK author
description: Use when composing the authoring prompt or checklist for a DRAFT Architecture PACK.yaml — routes to the architecture-pack factory skill; does not BIND.
---

# Architecture PACK author (operator skill)

Compose a **PACK authoring** run (DRAFT only). Human BIND is a separate skill.

## Context hard gate

- `docs/FIELD-GUIDE.md` (Architecture section)
- `skills/architecture-pack/SKILL.md`
- `schemas/architecture-pack.schema.md` + `.json`

## When to use

- `REPLAY_GREEN` for in-scope cases **or** valid `WAIVED_*` on file
- Need to-be target before Conversion
- Optional early DRAFT before green (allowed); BIND still blocked without green or waiver

## Recipe

1. Gather `MIGRATION_ID`, `SLICE_ID`, target stack intent, non-goals, evidence links (Discovery, TRACEABILITY, waiver if any).
2. Paste prompt that invokes Architecture pack skill / Cloud Agent author path.
3. Require `status: DRAFT` only — forbid agent-set `BOUND`.
4. Point output to `architecture/<MIGRATION_ID>/PACK.yaml`.

## Paste-ready authoring prompt

```text
Author a DRAFT Architecture PACK (do not BIND).

Read first (hard gate):
- migration-factory/docs/FIELD-GUIDE.md
- migration-factory/skills/architecture-pack/SKILL.md
- migration-factory/schemas/architecture-pack.schema.md
- migration-factory/schemas/architecture-pack.schema.json

MIGRATION_ID: <id>
SLICE_ID: <id>
Evidence:
- discovery/<SLICE_ID>/
- testgen/<SLICE_ID>/TRACEABILITY.yaml
- testexec results OR testexec/<SLICE_ID>/WAIVED_*.md

Operator intent (sparse OK):
- Target stack: <…>
- Patterns: <…>
- Forbidden / do-not-port: <…>
- Non-goals: <…>

Write architecture/<MIGRATION_ID>/PACK.yaml with status: DRAFT.
Add ADR explaining choices. Include mapping_rules, edit_surface, quality_gates, known_risks.
If slice deltas needed: slice-overlays/<SLICE_ID>.yaml (also DRAFT).
STOP for human BIND (skill: architecture-pack-bind). Never set status: BOUND.
```

## Exit

DRAFT pack PR → operator runs `architecture-pack-bind`.
