# ADR 0027 — engine v29: none-batch (baseline-honest)

Status: accepted (pack v29 BOUND, Ash Osborne, delegated autonomy run 39)

## Baseline presence checks (mandatory, run first)

A direct no-extension compile of the amalgamation (`/tmp/presence.c` against `sqlite3.c`
alone) settled each candidate. This is the factory harness baseline — NOT the shell CLI.

| Surface | Bare-build probe | Decision |
| --- | --- | --- |
| `delta_create` (misc-fossildelta-001) | `no such function` | **absent → stay none** (prior rc=0 golden was force-linked) |
| `eval` (misc-utilities-001) | `no such function` | **absent → stay none** |
| `dbstat` (introspection-vtabs-001) | `no such table` | **absent → stay none** (prior golden already rc=1) |
| `sqlite_dbpage` / `bytecode` / `sqlite_stmt` | `no such table` | **absent → stay none** |
| lookaside slab + variadic `db_config` (malloc-subsystem-002) | present in C but real slab counters (48 slots / 116 hits) + variadic config | **not honestly modellable → stay none** |
| qualified-name-in-trigger rejection (attach-detach-003) | core rule, present (rc=1 with exact message) | **implement** |

The lesson from run 38 (percentile) generalized: several "legacy_green" misc cards were
recorded with extensions force-linked and are NOT in the pinned bare build. Implementing
them would be greenwash, so they are left none with this ADR as the record.

## What landed

`attach-detach-003` core rule: inside a **non-TEMP** trigger body, a qualified table name
(`schema.table`) in INSERT / UPDATE / DELETE is rejected at CREATE with C's exact message
(`qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within
triggers`), and the trigger is not created. TEMP triggers are exempt (pinned rc=0).
Qualified names inside a trigger's SELECT are allowed. A multi-statement body with one
qualified DML fails as a whole. `split_statements` now keeps `CREATE TEMP/TEMPORARY
TRIGGER ... END` bodies whole.

Residual (kept partial): the attached-schema (aux3) DDL and cross-db VIEW fixation cases
in the original golden need real attached-schema tables, which modern does not have
(attach-detach-001/002 are partial); the rule is proven on the main schema only.

## Dropped this run (honest)

- `engine-none29-001-C004`: its trigger body is INSERT-SELECT, which modern's trigger
  engine does not execute — cannot reproduce the fired row count honestly.
- Everything absent-from-baseline above: no cases frozen, cards stay none.

## Consequences

6 goldens replay byte-identical; anti-cheat proves the rejection over runtime table names;
cargo 652/652; all prior goldens intact. attach-detach-003 flips none → **partial**;
composed card engine-none29-001 full. A deliberately small, zero-greenwash run.
