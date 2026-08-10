---
name: Waive characterization
description: Use when Test execution cannot run on legacy (e.g. no legacy runtime / license / harness). Creates WAIVED_* evidence + ADR stub so Architecture/Conversion may proceed without claiming PARITY.
---

# Waive characterization (operator skill)

Pathfinder fork: **Test Gen done → waive Test Exec → Architecture → Conversion**.  
Verification still owns any future **PARITY** claim — waiver ≠ green.

## Context hard gate

Field Guide Test execution + Operator Runbook WAIVED_PATHFINDER row. Refuse to “silently skip” without artefacts.

## When to use

- Legacy runtime unavailable (license, env, secrets)
- Operator explicitly chooses pathfinder progress over baseline goldens
- Need Conversion unlocked **without** `REPLAY_GREEN`

## Non-goals

- Do not invent goldens or mark `REPLAY_GREEN`
- Do not set `PARITY=GREEN`
- Do not BIND packs in this skill

## Recipe

1. Confirm Test gen pack exists (specs/stubs + TRACEABILITY) for `SLICE_ID`.
2. Choose waiver code, e.g. `WAIVED_PATHFINDER` (license), `WAIVED_ENV`, `WAIVED_SECRETS`.
3. Write artefacts:

```text
testexec/<SLICE_ID>/
  WAIVED_PATHFINDER.md   # or WAIVED_<CODE>.md
  REPORT.md              # points at waiver; no false greens
```

4. ADR stub (recommended): `architecture/<MIGRATION_ID>/ADR/00xx-waive-characterization-<slice>.md` **or** under `discovery/<SLICE_ID>/ADR/` if pack folder not yet created.
5. State explicitly: **Conversion may proceed**; **no PARITY claim**; Verification required before customer parity sign-off.
6. Architecture BIND may proceed only with waiver cited in pack `evidence` / risks (human still BINDs).

## Paste-ready WAIVED_PATHFINDER.md

```text
# WAIVED_PATHFINDER — <SLICE_ID>

## Decision
Test execution RECORD/REPLAY against legacy is waived for this pathfinder slice.

## Code
WAIVED_PATHFINDER

## Reason
<e.g. legacy runtime / license / harness not available in agent environment>

## Preconditions met
- [ ] Discovery bound + documented cards for in-scope features
- [ ] Test gen TRACEABILITY + specs/stubs present for waived batch
- [ ] CASE_IDS / CARD_IDS covered by waiver: [<ids>]

## Explicit non-claims
- No REPLAY_GREEN
- No PARITY=GREEN
- Goldens are not frozen for these cases

## Unlocks
- Architecture DRAFT/BIND may proceed with this waiver cited
- Conversion may proceed under BOUND pack
- Verification COMPARE remains blocked until legacy RECORD exists or a later waiver policy says otherwise — default: **Verification cannot claim parity**

## Authority
Waived by: <name>
At: <YYYY-MM-DD HH:MM TZ>
Ticket: <id>
Review by: <architect / SME>
```

## ADR stub (minimal)

```text
# ADR: Waive characterization for <SLICE_ID>

Status: Accepted (pathfinder)
Context: Legacy Test Exec unavailable (<reason>).
Decision: WAIVED_PATHFINDER; convert without frozen goldens.
Consequences: No parity claim; must revisit RECORD before production parity sign-off.
```

## Refuse

- Waiver without Test gen artefacts
- Using waiver to skip Discovery bind
- Claiming suite green in chat after waiver
