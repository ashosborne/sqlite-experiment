# shell-cli-002 — Dot-command surface

Slice: `shell-cli` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Meta-commands (.mode/.import/.dump/.schema/...) with own parser and modes stack.

## Entrypoints (citations)

- `do_meta_command()` (other) — `src/shell.c.in:9773`, `src/shell.c.in:459`

## Inputs / outputs / observables

- ~60 dot-commands' outputs (.mode list/csv/json/table/box..., .schema, .dump, .import, .read, .open, .backup...); .mode stack push/pop

## Behaviour (as implemented)

- do_meta_command (src/shell.c.in:9773) dispatches dot-commands with its own arg tokenizer (DotCmdLine src/shell.c.in:459); .dump emits schema+data SQL with proper quoting; .import parses per-mode separators into a table (auto-create with header row)

## Validation rules found in code

- Unknown dot-command → error line, not fatal (unless bail)
- Arg count validation per command

## Edge cases found in code

- .dump of virtual tables emits CREATE VIRTUAL TABLE but data only for some; .import type inference is text-only (all TEXT)

## Dependencies

- shell-cli-001

## Assumptions / unknowns

- Cluster-at-bind note stands: card pins the dispatcher; per-command depth = Test-gen decision on the downstream-used subset
- Which dot-commands does downstream automation actually use? (large surface — cluster at bind)

## Evidence

- `src/shell.c.in:9773`
- `src/shell.c.in:459`
