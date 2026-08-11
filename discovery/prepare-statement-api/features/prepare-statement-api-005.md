# prepare-statement-api-005 — Reset / finalize / auto-reprepare on schema change

Slice: `prepare-statement-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:21:09Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

reset preserves bindings; finalize frees; sqlite3Reprepare retries after SQLITE_SCHEMA.

## Entrypoints (citations)

- `sqlite3_finalize()` (api) — `src/vdbeapi.c:105`, `src/vdbeapi.c:134`, `src/prepare.c:904`

## Inputs / outputs / observables

- reset return code = error code of the previous step (v2 semantics); finalize returns last error; statement recompilation visible via sqlite3_stmt_status counters

## Behaviour (as implemented)

- sqlite3_reset (src/vdbeapi.c:134) rewinds the VM to pre-step state, preserving bindings and the compiled program
- sqlite3_finalize (src/vdbeapi.c:105) frees the statement (legal in any state)
- sqlite3Reprepare (src/prepare.c:904) recompiles from saved SQL after schema change, transferring bindings

## Validation rules found in code

- Auto-reprepare preserves bindings by parameter index — name→index mapping can shift if SQL text semantics changed underneath (schema rename)

## Edge cases found in code

- Reset does not clear bindings (clear_bindings does)
- Finalize during an open transaction leaves the transaction open

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/vdbeapi.c:105`
- `src/vdbeapi.c:134`
- `src/prepare.c:904`
