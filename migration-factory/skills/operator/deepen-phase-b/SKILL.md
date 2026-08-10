---
name: Deepen Phase B
description: Use after a human bind to compose the Discovery Phase B deepen prompt — behaviour cards only for accepted candidates; Cloud Agent / PR-shaped output.
---

# Deepen Phase B (operator skill)

Turn a completed bind into a **paste-ready Phase B** Discovery run.

## Context hard gate

Field Guide App Discovery Phase B + `prompts/discovery-agent-v0.2.md`. Refuse if bind not recorded.

## When to use

- MANIFEST shows accepted candidates; Phase A stopped correctly
- Operator wants deepen prompt without re-litigating scope

## Preconditions

- Bind record exists (`record-bind` output or equivalent in MANIFEST)
- `SLICE_ID` known
- Rejected/deferred listed (so agent does not “helpfully” deepen them)

## Recipe

1. Collect accepted IDs (batch ≤ risk-ordered top N if large).
2. Compose prompt that: attaches Field Guide + schema; cites bind; forbids deepening rejects/defers; asks for behaviour cards + MANIFEST update + PR under `discovery/<SLICE_ID>/`.
3. Remind: SME sign-off on pack unlocks Test gen.

## Paste-ready Phase B prompt

```text
You are the App Discovery agent in Phase B (deepen only).

Context hard gate — read first:
- migration-factory/docs/FIELD-GUIDE.md
- migration-factory/schemas/discovery-manifest.schema.md
- migration-factory/prompts/discovery-agent-v0.2.md

SLICE_ID: <slice>
Bind source: discovery/<SLICE_ID>/ (MANIFEST + bind record)
ACCEPTED_FEATURE_IDS:
  - <id>
  - <id>
Do NOT deepen rejected or deferred IDs.
Do NOT invent accepts.

Produce behaviour cards under discovery/<SLICE_ID>/features/ for accepted only.
Update MANIFEST.yaml; keep needs-SME/blocked explicit.
Prefer a PR under discovery/<SLICE_ID>/.
Stop when cards + SME_BRIEF sign-off checklist are ready for human SME sign.
```

## Exit

Operator pastes into Cloud Agent / strong model session; artefacts land under `discovery/<SLICE_ID>/`.
