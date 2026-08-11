# shell-cli-001 — CLI entry and SQL input loop

Slice: `shell-cli` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

argv handling, rc files, interactive vs batch input, exit codes.

## Entrypoints (citations)

- `sqlite3 CLI main()` (job) — `src/shell.c.in:13784`, `src/shell.c.in:13237`, `src/shell.c.in:13121`

## Inputs / outputs / observables

- Exit codes; -bail/-batch/-cmd flag behaviour; rc file loading (~/.sqliterc); stdin vs interactive prompts

## Behaviour (as implemented)

- main (src/shell.c.in:13784) parses argv (two-pass: pre-rc then post-rc flags), opens the db (deferred until first use), runs -cmd/dot/SQL args then process_input (src/shell.c.in:13237) line loop; runOneSqlLine (src/shell.c.in:13121) executes via shell_exec with error reporting to stderr

## Validation rules found in code

- Unbalanced quotes keep accumulating lines (continuation prompts)
- Non-zero exit on error only with .bail on / -bail

## Edge cases found in code

- Preprocessed from shell.c.in by tool/mkshellc.tcl (out-of-scope path) — template is the cited source; embeds several ext/misc extensions at build

## Dependencies

- connection-lifecycle-api
- prepare-statement-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/shell.c.in:13784`
- `src/shell.c.in:13237`
- `src/shell.c.in:13121`
