---
name: Matrix gate reply
description: Use when Test generation Phase A has proposed a case matrix in SME_BRIEF — produce paste-ready approve/drop/defer answers so Phase B specs can start.
---

# Matrix gate reply (operator skill)

Pathfinder-friendly **paste-ready** replies for the Test gen mid-gate.

## Context hard gate

Field Guide App Test generation + `prompts/test-generation-agent-v0.1.md`. Refuse without matrix/`SME_BRIEF.md`.

## When to use

- Test gen Phase A proposed cases; human must approve/drop/defer
- Operator wants a durable reply (PR comment or chat paste) without coaching theatre

## Non-goals

- Do not invent cases or expects
- Do not mark suite “green” (Test execution owns RECORD/REPLAY)
- Generic non-matrix gates → `compose-gate-reply`

## Recipe

1. Read proposed matrix rows (card_id → case_id → intent).
2. For each row: `approve` | `drop` | `defer` + reason if drop/defer.
3. Call out any row that lacks Discovery evidence → must `drop` or `defer`, not approve.
4. State which `CARD_IDS[]` / `CASE_IDS[]` enter Phase B.
5. Emit paste block.

## Paste-ready template

```text
## Test gen matrix gate — testgen/<SLICE_ID>/

| Case ID | Card ID | Decision | Note |
| --- | --- | --- | --- |
| <case> | <card> | approve | |
| <case> | <card> | drop | no evidence on card |
| <case> | <card> | defer | perf — not observable |

Phase B batch:
CARD_IDS: [<…>]
CASE_IDS: [<approved only>]

Assert mode remains TO_BE_RECORDED.
Do not invent expects. Hand off to Test execution after stubs land.

Approved by: <name>
At: <YYYY-MM-DD HH:MM TZ>
```

## Refuse

- Approving speculative ISTQB buckets not on the behaviour card
- “Approve all” when `needs-SME` cards are in batch without waiver
