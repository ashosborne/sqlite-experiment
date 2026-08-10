---
name: Verify parity
description: Use when modern conversion needs parity proof — route to the Verification agent (COMPARE vs frozen goldens); do not invent COMPARE in chat. Stub/day-2 depth OK; name locks the path.
---

# Verify parity (operator skill)

**Named so operators do not invent Verification in chat.**  
This skill **routes / composes** a Verification agent run; it is **not** a substitute COMPARE engine. Do **not** invent COMPARE results, greening, or golden diffs in operator chat.

## Context hard gate

Field Guide Verification section + `prompts/verification-agent-v0.1.md`. Refuse “just check it looks right.”

## When to use

- Conversion handoff `PARITY=UNVERIFIED`
- Frozen legacy goldens exist for in-scope cases
- Customer / SME asks “are we at parity?”

## When **not** to use (refuse)

- **No goldens** and **no waiver disposition** for deferred verification — refuse parity claim; say Verification cannot run COMPARE; schedule legacy RECORD **or** document deferred-verification disposition explicitly
- Only `WAIVED_*` exists and policy says Verification is deferred — **cannot** claim parity; optional: schedule legacy RECORD first
- Temptation to RECORD on modern or edit `*.approved.*`
- Chat eyeballing / invented COMPARE as GREEN

## Recipe (day-1 stub; day-2 depth OK)

1. Confirm entry: Conversion handoff, BOUND pack cite, **goldens present** (or explicit deferred-verification disposition — then do not launch COMPARE).
2. Launch / compose Verification agent with Field Guide + verification-agent prompt (`MODE=COMPARE`, `TARGET=modern`, `FORBID_RECORD=true`).
3. Goldens are **read-only** — never rewrite from modern.
4. Point artefacts to `verification/<SLICE_ID>/<RUN_ID>/`.
5. Human gate on FAIL: fix Conversion vs intentional delta (delta requires new legacy RECORD — out of Verification).

## Paste-ready Verification kickoff

```text
Run App Verification (COMPARE only). Do not invent results in chat.

Read first (hard gate):
- migration-factory/docs/FIELD-GUIDE.md
- migration-factory/prompts/verification-agent-v0.1.md

SLICE_ID: <id>
RUN_ID: <id>
MODE: COMPARE
TARGET: modern
GOLDENS: read-only
FORBID_RECORD: true

Inputs:
- Conversion handoff (PARITY=UNVERIFIED)
- pack_id@version: <…>
- FEATURE_IDS: [<…>]
- Golden paths: tests/characterization/<SLICE_ID>/.../*.approved.*

Deliver:
- verification/<SLICE_ID>/<RUN_ID>/PARITY.yaml  (GREEN|FAIL|BLOCKED)
- narrative/ + evidence/
- No golden writes

Do not claim parity in chat without PARITY.yaml.
```

## Exit

`PARITY` set only by Verification artefacts — not by operator skill prose or chat COMPARE.
