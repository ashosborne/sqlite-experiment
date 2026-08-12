# ADR 0011 — engine v13: prepare/bind through the real engine (pack v13)

Date: 2026-08-12 · Status: BOUND (supersedes pack v12; v1–v12 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 23)

## Context
sqlite3_exec has run the real kitchen/eval engine since v8-v11, but the statement API
was still a three-pin recognizer: prepare matched literal strings, bind was bind_int
only, columns were hard-coded coercions, step was a per-golden state machine. Six
prepare-statement cards were Partial for exactly that reason.

## Decision — how prepare executes (plainly)
**Shared engine, not a mini-VDBE.** prepare slices the first statement (top-level ';'
outside strings; pzTail points at the remainder), scans parameters (?, ?N, :name),
resolves names at prepare time exactly where C does (dry-running SELECTs with NULL
parameters — side-effect free — yields "no such table:/no such function:" and the
column-name list; DML targets are checked against the catalog), and stores the SQL.
step substitutes typed bound values into the statement as SQL literals and executes
through the SAME store/eval engine as sqlite3_exec: SELECT/PRAGMA materialize typed
rows on the first step (nested-loop eval; explicitly NOT a bytecode VDBE), DML/DDL run
through the script engine with real error codes (constraint → 19). Autoreset
(OMIT_AUTORESET=off) re-executes after DONE. reset preserves bindings, discards rows.

## Coverage
Binds: null / int / int64 / double / text / blob (values copied — TRANSIENT-safe),
parameter count / name / index, SQLITE_RANGE out-of-range. Columns: count / name
(expression spelling) / type / int / int64 / double / text / blob / bytes with real
coercions (text integer-prefix, real truncation), NULL and out-of-range behaviour as
pinned. stmt_readonly/stmt_busy are real statement properties. `Val::Real` added
end-to-end (store, file serial 7, literals) — bound doubles insert and persist.
26 bespoke goldens frozen on pinned C (two-run gate, delegated stamp) all replay
byte-identical; the 3 legacy recognizer pins now pass through the real path unchanged.

## Consequences
cargo 297/297; anti-cheat 15/15 (runtime binds drive computed sums, prepared DML
visible to later SELECTs, mixed runtime column types). Honestly out of scope:
sqlite3_prepare (v1) / v3 prepFlags, UTF-16 variants, EXPLAIN, auto-reprepare on
schema change, sqlite3_value objects. SQLite is NOT migrated.
