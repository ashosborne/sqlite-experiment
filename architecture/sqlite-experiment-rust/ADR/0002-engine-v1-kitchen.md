# ADR 0002 — Engine v1: the kitchen gets a real store

Status: accepted (BOUND with pack v4, Ash Osborne, 2026-08-11 Europe/London — the run-13 charter is
the bind act; ALLOW_PACK_SUPERSEDE under the full-autonomy delegation).

## What changes

Until v3, the Rust side was allowed to answer every frozen SQL script from a lookup table — see the
string, return the frozen rows. That satisfied the thin-rewrite law and it is **not a database**.

v4 changes the law for the kitchen slice: five kitchen goldens (engine-kitchen-001-C001..C005) plus
the two re-homed cases (ddl-schema-001-C001, dml-codegen-001-C001) must be answered from a **real
in-memory row store**. CREATE TABLE adds a table. INSERT appends rows whose values come from the
statement text. UPDATE/DELETE mutate matching rows. SELECT reads whatever is in the store at that
moment. The cheat-sheet (full-script lookup) is a SCOPE_VIOLATION for kitchen SQL, and an anti-cheat
test inserts a runtime-chosen integer that no lookup table can contain.

## What does not change

Standalone Rust, no `sqlite3.c` link, no wasm, no files/WAL/btree format. Non-kitchen pins stay
recognizer/state-machine, each still frozen against its C golden; the still-recognizer list is a
named known_risk in the pack. C003 stays BLOCKED. All prior HUMAN_ACCEPTED cases remain in scope.

## Honest limit

The store is a toy: a handful of statement shapes, integer/text columns, single-table SELECT with
optional ORDER BY, sqlite_master counting and changes counters. No planner, no durability, no
concurrency, no types beyond INTEGER/TEXT. **SQLite is not migrated.** This is the first increment
of `incremental-engine-rewrite` that actually stores a row.
