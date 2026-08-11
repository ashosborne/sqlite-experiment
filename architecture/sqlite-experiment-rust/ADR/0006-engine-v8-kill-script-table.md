# ADR 0006 — engine v8: kill the script table (pack v8)

Date: 2026-08-11 · Status: BOUND (supersedes pack v7; v1–v7 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 17)

## Context

After v7, kitchen + file paths ran real DDL/DML on a real store, but 70 catalogue
scripts were still answered by `script_table.rs`: exact frozen SQL string → pinned
rows. That is a cheat sheet, not an engine, and it silently capped what "migrated"
could honestly mean for those slices.

## Decision

1. **Cheat-sheet ban.** `sqlite3_exec` may never answer in-scope SQL by looking up
   the whole statement string (or hash, or disguised full-SQL match). script_table.rs
   is now an **empty table** kept only so the module path stays stable.
2. **Expression/SELECT law.** SELECT lists, scalars, aggregates, CASE/iif, CAST,
   LIKE/GLOB (with ESCAPE + case_sensitive_like), IN, IS, UNION/UNION ALL, ORDER BY,
   simple WHERE, and PRAGMA get/set are evaluated by `store.rs` + new `eval.rs`
   (tokenizer → Pratt parser → typed values) from values and connection state.
3. **Function law.** Builtins/misc functions are computed from arguments:
   printf/upper/length/substr/coalesce/typeof/hex, json_* (`json.rs`, real JSON
   parser + path), sha1/sha3 (real digests), base64, rot13, decimal add/cmp,
   ieee754, zorder, tointeger/toreal, uint collation, tiny REGEXP, generate_series/
   prefixes/json_each row sources, row_number/sum OVER (two pinned frame shapes).
4. **Honest defers.** 19 pins were removed *without* replacement rows: their tests
   are deleted, no store coverage is claimed, goldens stay frozen for a future run
   (list in PACK boundaries.deferred_cases_v8 — ANALYZE, VACUUM, dbstat, csv/
   completion/wholenumber vtabs, zlib compress, fossil delta, next_char, eval(),
   cross-schema trigger, C registry counts, 4 date/time pins, decimal_mul).
5. **Anti-cheat.** `anti_cheat_v8.rs` feeds runtime-varying literals through exec
   (expr, pragma round-trip, json) — values that cannot exist as pins — and asserts
   `SCRIPT_TABLE.len() == 0` and that unknown SQL still fails honestly.

## Consequences

- 50 former pins now execute for real (store/eval), byte-identical to the frozen C
  goldens; 131 prior-catalogue tests + 5 anti-cheat tests green.
- The engine is still NOT full SQL: no joins/planner, no WAL, no crash recovery,
  no full integrity_check, deferred Bucket-F extensions above.
- parity_green stays 0; no claim that SQLite is migrated.
