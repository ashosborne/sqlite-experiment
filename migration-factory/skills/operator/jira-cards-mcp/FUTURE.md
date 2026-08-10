# FUTURE: behaviour cards → Jira (Atlassian MCP)

**Status:** **Not built.** Optional Atlassian MCP later. **Out of day-1 scope** — do not implement an MCP server, Slack bot, or Jira bot as part of the current operator pack.

## Intent

When ready, an optional Atlassian MCP tool could map Discovery behaviour cards (and maybe Test gen cases) into Jira issues for tracking Conversion waves.

## Non-goals (now)

- No MCP implementation in this folder yet
- No Slack bot accounts / Slack bot creation
- No GitHub PAT creation
- Not required for pathfinder day-1 operator routing

## Likely inputs (sketch only)

- `discovery/<SLICE_ID>/features/*.md`
- Bound MANIFEST IDs
- Optional: TRACEABILITY case links

## Operator workaround today

Create Jira issues manually from `SME_BRIEF.md` / behaviour card titles; link ticket IDs in Conversion PRs.

When this graduates to a skill, add `SKILL.md` and a row in `skills/operator/README.md` + the Operator Runbook routing table.
