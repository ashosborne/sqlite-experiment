# ADR 0035 — engine v37: one-hole partial→full wave

Status: accepted · Date: 2026-08-14 · Operator: Ash Osborne · Pack: v37 (supersedes v36)

## URI probe (the run's honesty gate)

On the pinned bare build (USE_URI compile default **off**):
- `sqlite3_open("file:/tmp/x.db")` treats the name as a **literal path** and fails
  CANTOPEN 14 (no `file:` directory) — no URI parsing runs. Pinned.
- `sqlite3_open_v2(..., SQLITE_OPEN_URI)` **does** parse per-open: real path created,
  `mode=ro` forces readonly (write rc 8), `mode=memory` keeps everything off disk,
  unknown `vfs=` errors `no such vfs: X`, query parameters are retrievable through
  `sqlite3_uri_parameter/int64/boolean` on the `db_filename` (which strips the query;
  `:memory:` reports the empty string). Pinned and implemented — modern parses **only**
  where C parses.
- The urifuncs **SQL** surface is ABSENT on the bare pin
  (`no such function: uri_parameter`, pinned refusal).

## Estate partial → FULL (5)

| Card | Former residual → evidence |
| --- | --- |
| connection-lifecycle-api-001 | URI parsing + open flags: full open_v2 matrix pinned (READONLY missing/existing, RW-without-CREATE, MEMORY flag, zero-flags MISUSE, uncreatable literal paths CANTOPEN) + the per-open URI behaviour above |
| misc-urifuncs-001 | URI parameter parsing now real (C-API accessors over really-parsed parameters, runtime anti-cheat); the SQL-function surface is pin-absent with a pinned refusal |
| backup-api-001 | multi-page lifecycle: 200-row source copies with content verified in the destination (count + sibling table), empty-pair pins still exact |
| backup-api-002 | remaining()/pagecount() move for real: pagecount>1, remaining == pagecount−1 after step(1), == pagecount−3 after step(2), 0 at DONE; runtime-sized anti-cheat |
| window-functions-002 | EXCLUDE (NO OTHERS / CURRENT ROW / GROUP / TIES) + offset RANGE (1P/1F, 2P/0F with peer-value grouping) + RANGE⊕EXCLUDE composition, all pinned with tie rows |

## Deepened, honestly still PARTIAL

| Card | This run | Remaining residual |
| --- | --- | --- |
| auth-callback-api-001 | s1–s4 are the probed C strings for INSERT / per-column UPDATE / DELETE / SELECT (21 then ordered READs) / PRAGMA / TRANSACTION / ATTACH / DETACH; SQLITE_IGNORE on DML pinned (INSERT/UPDATE skip silently, **DELETE proceeds** — truncate-opt only) | ~22 action codes not dispatched (ALTER/CREATE_INDEX/DROP sequences carry sqlite_master bookkeeping callbacks modern does not emit; function code 31; trigger/view s4 contexts) |
| backup-api-003 | same-connection write-between-steps restart re-pinned on a multi-page copy; destination sees the late write | cross-connection BackupUpdate page patching unpinned (modern copies the image at completion; C patches pages — observables pinned equal for the frozen scope) |
| prepare-statement-api-006 | sqlite3_stmt_isexplain (0/1/2) + sqlite3_stmt_explain mode switching (rc, column shape, bad mode) pinned real | EXPLAIN bytecode **row contents** — there is no VDBE; nested-loop EQP is not bytecode (charter law) |
| global-init-config-002 | after-init matrix pinned: threading modes/MEMSTATUS/URI → MISUSE, LOG legal, PCACHE_HDRSZ reports modern's real page-image accounting record size, bad op MISUSE | before-init behaviour of the remaining ~20 ops (heap/pcache/mmap/scratch configuration) unpinned |
| global-init-config-003 | ENABLE_TRIGGER / ENABLE_VIEW / DQS_DML with **real effects** (trigger firing suppressed, `access to view "vv" prohibited`, double-quoted-string DML acceptance vs C's hint error) + DEFENSIVE round-trip + bad-verb error | DQS_DDL, WRITABLE_SCHEMA, RESET_DATABASE, LEGACY_ALTER, TRUSTED_SCHEMA, load-extension gates; DEFENSIVE has no enforced effect |

## Not attempted

The stay-partial list from the charter (WAL, planner, pragma census, scalars, zlib,
va_list, mini-slots, decimal, regexp, dlopen, unlock-notify, census, pager/btree/vfs/
fts/wasm/jni/session/expert, vtab-core-001 mega, attach-detach-003 TEMP fire,
error-status-api-003 leftovers) — untouched. Opportunistic list skipped: the named
targets consumed the budget.

## Scoreboard

before 173 full / 53 partial / 78 none of 304 → after **185 full / 48 partial / 78 none
of 311** (5 estate flips + 7 composed engine-harvest37 cards; 5 tightened partials).
SQLite is NOT migrated.
