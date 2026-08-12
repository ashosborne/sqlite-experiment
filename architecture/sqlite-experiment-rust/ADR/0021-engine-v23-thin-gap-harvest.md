# ADR 0021 — engine v23: thin-gap harvest

Status: accepted (pack v23 BOUND, Ash Osborne, delegated autonomy run 33)

## Context

After the v22 WAL slice, COVERAGE named many one-hole Partials. This run harvests the
smallest named residuals for honest full flips. WAL was explicitly not deepened.

## Attempted → outcome

| Card | Named residual | Outcome |
| --- | --- | --- |
| loadext-api-002 | cancel/reset auto-extension absent | **flipped full** — multi-entry ordered registry, cancel (1/0 pinned), reset, duplicate collapse |
| malloc-subsystem-001 | memory_used/highwater absent | **flipped full** — accounting in `sized_alloc` (every sqlite3_free'd pointer originates there); sticky highwater + reset pinned |
| window-functions-001 | first/last/nth_value, ntile, percent_rank, cume_dist absent | **flipped full** — all six + named `WINDOW ... AS` clause + ROWS ... AND UNBOUNDED FOLLOWING frame |
| error-status-api-002 | one limit id pinned | **flipped full** — id matrix defaults/prior/clamp (COLUMN, ATTACHED, ...) + VARIABLE_NUMBER prepare-time enforcement with C's message |
| error-status-api-001 | errstr + extended-code matrix absent | **flipped full** — errstr table (extended codes fall through to base, pinned via 787/2067); extended constraint codes 2067/1299/275/787 on real error paths; UNIQUE/CHECK messages now qualified like C |
| foreign-keys-001 | deferred FKs absent | **flipped full** — DEFERRABLE INITIALLY DEFERRED + PRAGMA defer_foreign_keys (resets at txn end); COMMIT-time validation; failed COMMIT leaves the transaction open (pinned) |
| printf-format-002 | vmprintf / snprintf absent | **partial (tighter)** — snprintf landed (truncation/NUL/n<=0 pinned); `sqlite3_vmprintf` requires C va_list, which stable Rust cannot define — an honest platform residual, not deferred laziness |
| printf-format-003 | raw append + vappendf absent | **partial (tighter)** — `sqlite3_str_append` landed; vappendf has the same va_list residual |
| auth-callback-api-001 | only SQLITE_SELECT deny | **partial (tighter)** — INSERT/UPDATE/DELETE/CREATE_TABLE/PRAGMA deny paths landed; residual: per-object callback arguments (s1–s4), SQLITE_IGNORE column semantics, remaining action codes |
| tokenizer-002 | full nesting grammar | **partial (tighter)** — real BEGIN/CASE/END token scan (nested + multi-statement bodies pinned); residual: string-literal-aware lexing in complete() |

## The unplanned deep fix: REAL rendering (fpdec.rs)

The window pins exposed that modern rendered REALs via Rust shortest-round-trip, which
differs from SQLite's own dtoa in visible digits (C renders 1.0/3.0 as
`0.33333333333333332` — an artifact of its 18-digit convert + round-to-17, not
correctly-rounded printf output). `modern/src/fpdec.rs` is a faithful port of
`sqlite3FpDecode` / `Fp2Convert10` / `Fp10Convert2` (power-of-ten tables, 128-bit
multiplies, the %!.17g precision-reduction round-trip logic) plus the printf `%!g`
assembly. All 519 tests — every prior REAL pin — replay through the port.

## Consequences

24 goldens (engine-harvest23-001..009) frozen; 6 umbrella Partials flipped full; 4
tightened. 9 composed engine-harvest23-* cards added full for exactly their pins.
WAL claims unchanged. Not migrated.
