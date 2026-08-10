---
name: Compose gate reply
description: Use when a human must answer a factory mid-gate (SME sign-off, golden approval, Conversion plan gate, Verification FAIL triage) — drafts a durable approve/waive/block reply.
---

# Compose gate reply (operator skill)

Generic **human gate** composer. For Test gen matrices prefer `matrix-gate-reply`; for Discovery candidate binds prefer `record-bind`.

## Context hard gate

Field Guide section for the active stage + Operator Runbook refuse rows.

## When to use

- SME Discovery pack sign-off
- Golden approval after Test Exec RECORD
- Conversion plan mid-gate (mandatory/optional)
- Verification FAIL: fix Conversion vs intentional delta path
- Explicit waiver language needed (short of full `waive-characterization`)

## Recipe

1. Identify gate type + artefacts under review.
2. Decision: `approve` | `approve-with-nits` | `waive` | `block` | `defer`.
3. Require: who, when, scope IDs, residual risks, next stage unlock.
4. If `waive` for missing legacy exec → redirect to `waive-characterization` (needs `WAIVED_*` + ADR).
5. Emit paste-ready reply.

## Templates

### SME Discovery sign-off

```text
SME sign-off — discovery/<SLICE_ID>/
Decision: APPROVE for Test gen
Accepted cards: [<ids>]
Deferred/blocked acknowledged: [<ids>]
Nits (non-blocking): <…>
Signed: <name> <YYYY-MM-DD TZ>
Unlocks: Test generation Phase A
```

### Golden approval (Test Exec)

```text
Golden approval — tests/characterization/<SLICE_ID>/
MODE: RECORD outcomes reviewed
CASE_IDS approved as goldens: [<ids>]
BLOCKED left explicit: [<ids>]
REBASE_GOLDENS: not granted
Approved: <name> <YYYY-MM-DD TZ>
Unlocks: Architecture BIND (if DRAFT ready) / Conversion prechecks
```

### Conversion plan gate

```text
Conversion plan gate — pack <pack_id@version>
Decision: APPROVE | BLOCK
FEATURE_IDS: [<ids>]
edit_surface respected: yes/no
known_risks acknowledged: <…>
REQUIRE_PLAN_GATE satisfied.
<name> <YYYY-MM-DD TZ>
```

## Refuse

- Chat-only Architecture BIND (use `architecture-pack-bind`)
- Approving modern greening by editing `*.approved.*`
