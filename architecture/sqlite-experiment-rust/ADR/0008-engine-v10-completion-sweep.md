# ADR 0008 — engine v10: completion sweep (pack v10)

Date: 2026-08-11 · Status: BOUND (supersedes pack v9; v1–v9 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 20)

## Context

Run 19 gave the operator a scoreboard (impl_in_modern): 11 full / 75 partial / 108 none.
The highest-yield completion work was not WAL or a planner but (a) the four date/time
behaviours sitting in Remaining with frozen goldens, and (b) partial behaviours whose gap
note named a bounded missing piece.

## Decision

1. **Completion-sweep law.** The increment's success metric is honest growth of
   `impl_in_modern: full` (and shrinking partial gaps). New goldens exist to unlock
   upgrades, never to inflate legacy_green alone. Inventory bump + COVERAGE regen are
   part of the same change set (factory hard rule from run 19).
2. **Real date/time engine** (`modern/src/datetime.rs`): julian-day arithmetic exactly
   as date.c (computeJD/computeYMD/computeHMS on iJD milliseconds), modifiers
   (±N days/hours/minutes/seconds/months/years with month-overflow normalization,
   `weekday N`, `start of day/month/year`, `unixepoch`), strftime directive set, and
   timediff with iterative year/month reduction. Clears all four v8 date/time defers +
   the misc-func-packs defer (decimal_mul trailing-zero trim discovered from C bytes).
   `now`/`localtime` deliberately error (nondeterministic / TZ) — not faked.
3. **Bounded partial-gap closures, all computed:** INTERSECT/EXCEPT set-op fold; CHECK +
   NOT NULL + OR FAIL/ABORT; FK ON DELETE SET NULL / RESTRICT; CREATE/DROP VIEW with
   real view expansion + write rejection; generalized trigger matrix (BEFORE/AFTER ×
   INSERT/UPDATE/DELETE, old.*/new.*, WHEN) with per-row firing; printf
   flags/width/precision + d,i,u,f,e,E,g,G,x,X,o,s,c,q,Q; scalar batch (round, trim
   family, replace, instr, scalar min/max, sign, char, unhex, concat, concat_ws,
   octet_length, unicode); rot13 collation; pragma batch (foreign_keys, busy_timeout
   set-returns-value, encoding, page_size, journal_mode, synchronous, locking_mode,
   read_uncommitted, trusted_schema, threads, analysis_limit, reverse_unordered_selects,
   …); eager name resolution ("no such column", "ambiguous column name") at prepare
   time even for empty sources.
4. **37 new cases** frozen on pinned C (two-run determinism, delegated stamp) across
   engine-datetime / setops / constraints / views / triggers / funcs / pragma; all 37
   replay byte-identical through the executor; 5 v8 defers reclaimed, goldens untouched.
5. **Anti-cheat:** runtime day-of-month through strftime/date/unixepoch; runtime CHECK
   bound + INTERSECT value; SCRIPT_TABLE still empty.

## Consequences

- cargo suite 208/208; oneshot replays 56 (was 51); prior goldens byte-identical.
- Still NOT: WAL, cost-based planner, flattening, RIGHT/FULL joins, localtime/now,
  RAISE/recursive triggers, OR ROLLBACK (no transactions), full pragma dispatcher.
  SQLite is NOT migrated.
