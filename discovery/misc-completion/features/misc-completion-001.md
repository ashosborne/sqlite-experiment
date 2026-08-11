# misc-completion-001 — completion vtab (shell tab-completion)

Slice: `misc-completion` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Suggests keywords/schema names for interactive completion; embedded by the shell

## Entrypoints (citations)

- `sqlite3_completion_init()` (other) — `ext/misc/completion.c:510`

## Inputs / outputs / observables

- SELECT candidate FROM completion(prefix[,wholeline]) — keywords, schema names, table/column names, pragma names

## Behaviour (as implemented)

- Eponymous TVF (init ext/misc/completion.c:510) unions candidate sources by phase (keywords → schemas → tables → columns...); used by the shell's tab-completion

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Candidates depend on currently-open schemas

## Dependencies

- vtab-core
- shell-cli

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/completion.c:510`
