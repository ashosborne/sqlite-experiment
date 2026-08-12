# ADR 0034 — engine v36: partial→full harvest (overnight residual sweep)

Status: accepted · Date: 2026-08-14 · Operator: Ash Osborne · Pack: v36 (supersedes v35)

## Attempted cards and outcomes

36 new goldens over two waves (27 + 9), every pin probe-first on the bare build.

### Estate partial → FULL (named residual cleared on the pin)

| Card | Former residual | How it cleared |
| --- | --- | --- |
| tokenizer-002 | (audit) string-literal lexing chain ended at run-38 | audit pins: comments/strings gate `;`, quoted/nested END, whitespace-only |
| parser-grammar-002 | quoted reserved words as TABLE names | full CRUD + qualified + master + all three quote styles, end-to-end |
| attach-detach-002 | "database is locked" DETACH with open statement | active-statement schema-read gate; main-only statements don't lock |
| json-funcs-003 | json_valid flags argument + JSONB validation | flags 1/2/6 text matrix (strict + JSON5 validators), 4/8 JSONB byte walker, 1..15 range error |
| foreign-keys-003 | drop-order edges beyond pins | immediate-block / child-first / NULL-children / deferred-drop-COMMIT-catch / both-in-txn matrix |
| vacuum-001 | pending page_size / auto_vacuum apply; VACUUM \<schema\> | pending-until-VACUUM semantics pinned + applied; schema-scoped rebuild + unknown-database error |
| vacuum-002 | URI filename INTO forms | **pin-absent evidence**: USE_URI is off on this build — `file:...` targets are literal paths that fail `unable to open database` (pinned) |
| vtab-core-002 | vtab_config negotiation; HIDDEN constraints via xBestIndex/xFilter argv | sqlite3_vtab_config (OK in ctor ops 1..3, MISUSE outside/bad); REAL EQ constraint offers with C-layout arrays, consumed argvIndex delivers the value to xFilter (module bounds the scan — pinned + runtime anti-cheat) |
| connection-lifecycle-api-002 | unfinished backup coupling; post-close matrix | src close refuses BUSY (pinned errmsg), dst close defers teardown to backup_finish, step/finish error after deferral, double-close answers MISUSE via a safe tombstone (C reaches the same rc through its magic-number guard; deeper use-after-close stays unfrozen UB by design) |
| blob-io-api-001 | attached-schema / UTF-16 names / WITHOUT ROWID / open-inside-txn | zDb resolves attached store keys; WITHOUT ROWID refusal (C text); txn write-through pinned durable; **UTF-16: no UTF-16 blob_open API exists on this pin** — non-ASCII UTF-8 names pinned instead |

### Deepened, honestly still PARTIAL

| Card | This run | Remaining residual |
| --- | --- | --- |
| error-status-api-003 | sqlite3_stmt_status landed: FULLSCAN_STEP = really-visited rows−1 per unindexed scan (exact values + accumulation pinned + runtime anti-cheat), RUN cycles, MEMUSED real footprint, VM_STEP as step events (predicate only — no VDBE, magnitudes not claimed) | CACHE_SPILL under real pressure; VM_STEP magnitudes; scanstatus |
| analyze-stats-001 | attached-schema ANALYZE into `<schema>.sqlite_stat1` (table + whole-schema scopes; CREATE INDEX schema resolution); PRAGMA optimize per the pinned missing-stats contract | optimize usage/staleness gating heuristics not modelled; stat4 + sz=/unordered stay pin-absent (probe evidence run-37/46) |

### Not attempted (structural stay-partials, untouched)

WAL multi-conn, planner/VDBE, remaining ~30 pragmas, ~60 scalars, zlib byte-format,
va_list ABI, lookaside mini-slots, decimal precision, regexp NFA, dlopen, unlock-notify,
compile-options guard census, pager/btree/vfs/fts/wasm/jni/session/expert,
attach-detach-003 TEMP fire matrix, vtab-core-001 mega lifecycle.

## xBestIndex decision

ALLOW_XBESTINDEX exercised: modern now builds real `sqlite3_index_constraint(_usage)`
arrays (C layout), offers a detected `col = literal` EQ term on single-vtab FROMs, and
delivers consumed values through xFilter argv. The engine still applies WHERE afterwards
(omit is honoured implicitly — re-checking is safe). Under-claim: one simple EQ term is
offered (C offers every usable term of arbitrary shape); series/prefixes/wholenumber
umbrella cards are NOT re-homed and keep their harvest notes.

## Scoreboard

before 152 full / 63 partial / 78 none of 293 → after **173 full / 53 partial / 78 none
of 304** (10 estate flips + 11 composed engine-harvest36 cards; the two deepened cards
keep precise residual text). SQLite is NOT migrated.
