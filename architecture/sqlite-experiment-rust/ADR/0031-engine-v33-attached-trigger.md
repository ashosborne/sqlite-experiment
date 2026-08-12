# ADR 0031 — engine v33: attached-trigger fire (schema-strict body resolution)

Status: accepted · Date: 2026-08-13 · Operator: Ash Osborne · Pack: v33 (supersedes v32)

## Context

Run 40 (attached-schema ownership) left one named residual on `attach-detach-003`:
*"firing a trigger whose body targets an attached table (unqualified body resolution at
trigger execution) is not implemented"* — the pinned case engine-attach30-003-C003 was
dropped. DDL-time fixation (qualified body DML rejected) landed in runs 39/40; this run is
execution time.

## What the bare C pin actually does (probed before freezing)

The first harness draft used `CREATE TRIGGER trg ... ON aux.t` and C refused it — the pin
taught the real model:

- **The trigger name carries the schema.** `CREATE TRIGGER aux.trg ... ON t` creates an
  aux-schema object (listed by `aux.sqlite_master`, absent from main's). An *unqualified*
  trigger name lives in **main**, so `CREATE TRIGGER trg ... ON aux.t` errors
  `trigger trg cannot reference objects in database aux`.
- **ON resolves strictly in the trigger's schema.** `ON t` from `aux.trg` binds `aux.t`;
  `ON main.m` from `aux.trg` errors `trigger trg cannot reference objects in database
  main`; `CREATE TRIGGER trg ... ON t` with `t` only in aux errors `no such table: main.t`.
- **Body targets resolve strictly in the trigger's schema, no fallback.** With `log` in
  both schemas, an aux trigger writes `aux.log` only. With `log` only in main, the aux
  trigger *creates* fine but *fires* `no such table: aux.log`. A missing target fires
  `no such table: aux.nolog`.
- Fixation (run-40) unchanged: a qualified body target is rejected at CREATE
  (`qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within
  triggers`).
- DETACH removes the schema's triggers along with its tables; re-ATTACH is fresh.

## Decision (modern)

- `Trigger` carries its owning schema. CREATE TRIGGER resolves at exec time: schema from
  the (possibly qualified) trigger name; ON clause validated strictly against that schema
  with C's two error shapes; the canonical store key (`aux.t`) is what fire matching uses.
- `fire_triggers` resolves each body target inside the trigger's schema only —
  `{schema}.{target}` for attached triggers, bare for main — and errors
  `no such table: {schema}.{target}` when absent (fire time, like C).
- Unqualified DML (`INSERT INTO t`) resolves main-first then attach-order into attached
  tables (needed for the pinned fire-through-unqualified-INSERT case), at both
  prepare-check and exec.
- `sch.sqlite_master` is a real per-schema catalog view (bare `sqlite_master` = main
  only, matching C); DETACH tears down the schema's triggers.

## Fire paths pinned

AFTER INSERT (single + multi-row + WHEN-gated + two attached schemas + unqualified-INSERT
entry), AFTER UPDATE, AFTER DELETE, BEFORE INSERT, BEFORE+AFTER order — all on attached
tables, all mutating real attached rows.

## Residual cleared / deliberate leftovers

- Cleared from attach-detach-003: unqualified body resolution at trigger execution
  against attached tables (the run-40 dropped-case residual).
- Left (named, unclaimed): URI/encryption ATTACH maze, DETACH-locked edges, cross-schema
  transaction joins, TEMP-trigger cross-schema fire matrix (TEMP triggers remain
  accepted-but-not-fired per run-39 scope), trigger bodies beyond INSERT..VALUES
  (UPDATE/DELETE/INSERT-SELECT bodies — pre-existing engine residual, unchanged).
