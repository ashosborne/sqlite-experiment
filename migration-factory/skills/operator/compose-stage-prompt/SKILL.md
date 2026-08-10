---
name: Compose stage prompt
description: Use when the operator knows the next factory stage but needs a paste-ready agent prompt with inputs, attachments, and hard stops filled in.
---

# Compose stage prompt (operator skill)

Assemble the **next-stage run** from Field Guide + prompts tree.

## Context hard gate

Always include Field Guide + stage schema/prompt paths in the composed output. Refuse to compose “freestyle migrate” prompts.

## When to use

- “What do I paste for Test gen / Test exec / Conversion / Verification?”
- Switching slices or batches (`CARD_IDS[]`, `FEATURE_IDS[]`)
- After a gate reply unlocks the next stage

## Stage → prompt map

| Stage | Prompt | Schema / skill |
| --- | --- | --- |
| Discovery | `prompts/discovery-agent-v0.2.md` | `schemas/discovery-manifest.schema.md` |
| Test gen | `prompts/test-generation-agent-v0.1.md` | `schemas/testgen-traceability.schema.md` |
| Test exec | `prompts/test-execution-agent-v0.1.md` | `schemas/testexec-results.schema.md` |
| Architecture | skill `skills/architecture-pack/` (+ operator author/bind) | `schemas/architecture-pack.schema.*` |
| Conversion | `prompts/conversion-agent-v0.1.md` | BOUND PACK |
| Verification | `prompts/verification-agent-v0.1.md` | Verification artefacts |

Prefer specialised skills when they exist: `deepen-phase-b`, `architecture-pack-author`, `verify-parity`, `waive-characterization`.

## Paste skeleton

```text
STAGE: <name>
SLICE_ID: <id>
MODE / PHASE: <…>

Attach / read first (hard gate):
- migration-factory/docs/FIELD-GUIDE.md
- migration-factory/docs/OPERATOR-RUNBOOK.md
- <stage prompt path>
- <stage schema path>
- <prior artefacts paths>

Inputs:
- <table of required inputs>

Hard stops:
- <from prompt>

Deliverables path:
- <where files land>

Human gate after this run:
- <what operator must do next + which skill>
```

## Refuse

- Composing Conversion without BOUND pack citation (unless prompt is Architecture)
- Composing Test Exec RECORD against modern
- Skipping waiver skill when legacy cannot run
