# recover-002 — Recovery output as SQL callback stream

Slice: `recover` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

init_sql variant emits SQL statements to a callback instead of writing a db.

## Entrypoints (citations)

- `sqlite3_recover_init_sql()` (api) — `ext/recover/sqlite3recover.c:2795`, `ext/recover/sqlite3recover.c:2739`

## Inputs / outputs / observables

- Callback receives CREATE/INSERT SQL statements in dependency order instead of writing a db

## Behaviour (as implemented)

- recover_init_sql (ext/recover/sqlite3recover.c:2795) same pipeline, output as SQL text stream to the callback (:2739 doc region)

## Validation rules found in code

- Callback non-zero return aborts recovery

## Edge cases found in code

- Emitted SQL includes transaction framing suitable for piping into a fresh db

## Dependencies

- recover-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/recover/sqlite3recover.c:2795`
- `ext/recover/sqlite3recover.c:2739`
