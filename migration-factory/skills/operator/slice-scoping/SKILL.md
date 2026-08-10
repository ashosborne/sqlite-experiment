---
name: Slice scoping
description: Use when an operator needs a clear SLICE_ID, seeds, optional entrypoints, and out-of-scope hints before App Discovery Phase A — turns fuzzy “migrate this area” into factory inputs.
---

# Slice scoping (operator skill)

Help humans define a **migrate-able slice** without turning Discovery into an estate crawl.

## Context hard gate

Read `migration-factory/docs/FIELD-GUIDE.md` (App Discovery inputs) and `docs/OPERATOR-RUNBOOK.md`. Refuse if Field Guide is missing.

## When to use

- Pathfinder kickoff; product says “do account opening / NCD / …”
- Scope is too wide (“whole monolith”) or too vague (“payments”)
- Need paste-ready Discovery Phase A inputs

## Non-goals

- Do not invent behaviour cards or accept/reject candidates (that is Discovery + `record-bind`)
- Do not merge OS host-group scoping into this skill

## Recipe

1. Ask (or infer from ticket) for: business name, likely repos/modules, known APIs, explicit exclusions.
2. Propose:
   - `SLICE_ID` — kebab-case, stable (`account-opening`)
   - `SLICE_SEED` — short natural language
   - `SEED_ENTRYPOINTS` (optional) — routes, classes, flows
   - `OUT_OF_SCOPE_HINTS` — neighbouring features to keep out
3. Confirm with operator; warn if seed alone is the only input (seed ≠ contract; bind still required after Phase A).
4. Emit a paste block for Discovery Phase A.

## Paste-ready output template

```text
## Slice inputs (operator-scoped)

SLICE_ID: <kebab-id>
SLICE_SEED: <phrase>
SEED_ENTRYPOINTS:
  - <route or symbol>
OUT_OF_SCOPE_HINTS:
  - <neighbouring capability>

Attach: migration-factory/docs/FIELD-GUIDE.md
Prompt: migration-factory/prompts/discovery-agent-v0.2.md
Schema: migration-factory/schemas/discovery-manifest.schema.md
Mode: Plan Mode / Phase A map only — STOP for human bind
```

## Exit

Operator can launch Discovery Phase A with non-empty `SLICE_ID` + seed (entrypoints/out-of-scope preferred).
