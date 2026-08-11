# global-init-config-003 — Per-connection configuration (db_config)

Slice: `global-init-config` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-db toggles: enable_fkey/trigger/view, defensive, dqs, lookaside sizing, resetdb gate...

## Entrypoints (citations)

- `sqlite3_db_config()` (api) — `src/main.c:967`, `src/sqlite.h.in:1748`

## Inputs / outputs / observables

- db_config op results (old-value out-params): ENABLE_FKEY/TRIGGER/VIEW, DEFENSIVE, WRITABLE_SCHEMA, DQS_DML/DDL, LEGACY_ALTER_TABLE, LOOKASIDE sizing, RESET_DATABASE, TRUSTED_SCHEMA...

## Behaviour (as implemented)

- sqlite3_db_config (src/main.c:967) per-connection toggles with (set,getPrior) protocol; DQS defaults historically ON for compat (dialect gate), TRUSTED_SCHEMA default ON, DEFENSIVE blocks writable_schema and dbpage writes

## Validation rules found in code

- LOOKASIDE resize only with no outstanding allocations → SQLITE_BUSY

## Edge cases found in code

- RESET_DATABASE two-step protocol (set, VACUUM, unset) wipes a db even when corrupt

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Downstream toggle inventory = the highest-value SME question (run-2 flag stands)
- Which db_config toggles does downstream set? (defaults differ by option)

## Evidence

- `src/main.c:967`
- `src/sqlite.h.in:1748`
