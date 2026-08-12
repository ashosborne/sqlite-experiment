# MORNING BRIEF — engine v28: mega harvest (run 38)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–37 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v28-mega-harvest. MAX_NEW_CASES 80 (used 33
after dropping the percentile batch). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @28 BOUND — mega-harvest law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v27 → **v28**
(versions/1–28 retained; ADR `0026-engine-v28-mega-harvest.md`; schema VALID; 34 laws).
Extension functions are honoured ONLY where the pinned baseline provides them; SCRIPT_TABLE
stays 0; no fake vtab-core/planner. WAL/VACUUM/blob/conn unchanged.

## 2. The two-level-pin catch (why percentile was dropped)

My harness force-linked ext/misc `.c` files, which hid whether a function is actually in
the baseline. Cross-checking prior run-11 misc-* goldens settled each honestly:
`compress/next_char/wholenumber/completion` prior goldens are **rc=0** (bundled → honest to
implement); `misc-percentile-001` is **rc=1** (percentile/median are NOT in the pinned
build). I implemented percentile, saw it break the prior bare-build golden, and **reverted
+ dropped the whole percentile batch** rather than greenwash a function the baseline lacks.

## 3. Attempted → outcome (flips table)

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| exec-convenience-api-002 (get_table) | none | **full** | — |
| auth-callback-api-002 (column IGNORE) | none | **full** | — |
| misc-nextchar-001 | none | **full** | — |
| error-status-api-003 (status64/db_status) | none | **partial** | rest of the op matrix is honest-zero |
| misc-compress-001 | none | **partial** | reversible RLE, not zlib byte-format (round-trips only) |
| misc-wholenumber-001 | none | **partial** | bounded generator; vtab-core module system absent |
| misc-completion-001 | none | **partial** | keyword+schema candidates; full shell phases absent |
| parser-grammar-002 | none | **partial** | unquoted keywords-as-identifiers real; quoted reserved-word table names residual |
| tokenizer-002 | partial | partial (tighter) | string/comment-aware complete() landed |
| pragma-surface-002 | partial | partial (tighter) | function_list / pragma_list TVFs added |
| misc-percentile-001 | none | **none (kept)** | percentile absent from pinned build — must not implement |
| engine-harvest28-001..004/006/007/008 | — | **new full ×7** | composed cards for the frozen batches |

## 4. Engine holes opened (real, reused everywhere)

`BETWEEN` in the expression parser; **blob ≠ text** equality (a real SQLite rule this
engine had wrong); `SELECT *` expansion over a store table; quoted-identifier `ident()`;
`CREATE VIRTUAL TABLE` registration + bounded wholenumber generator; completion / pragma
registry TVFs; string/comment-aware `sqlite3_complete`.

## 5. Anti-cheat + cargo

- `anti_cheat_harvest28` — runtime compress round-trip (cast-compared, since blob≠text)
  and next_char over a pid-seeded table.
- Per-surface pins demonstrate the missing behaviour (get_table NULL layout, auth IGNORE
  nulling, status alloc-tracking, wholenumber bounds).
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 644/644 PASS** (was 609; +33 golden twins, +2 anti-cheat/guard).
  All prior suites green; 589 pre-run goldens md5-verified intact (incl. the bare-build
  `misc-percentile-001` rc=1 that guarded the percentile decision).

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 37 | Run 38 |
| --- | --- | --- |
| **full (converted)** | 120 | **130** |
| partial | 52 | 57 |
| none (remaining) | 93 | **85** |
| behaviours known | 265 | 272 |

+10 fulls, +5 none→partial, none down 8. legacy_green 175 → 182. parity_green 0.

## 7. Not migrated

SQLite is **not migrated**. 130/272 behaviours run honestly in modern for frozen scope
only. compress is an RLE stand-in (not zlib bytes); vtabs are bounded generators, not a
module system; percentile is deliberately absent. Parity UNVERIFIED everywhere.

## 8. Next call

1. **vtab-core** — a real (small) module/xBestIndex surface would upgrade wholenumber /
   completion / series / prefixes from bounded-generator partials.
2. **analyze-stats-002 / planner** — stats-driven index choice + EQP pins (the honest
   way to flip the planner-load card).
3. **misc-csv-001** — only if bounded in-memory `data=` pins can avoid filesystem I/O.
