# MORNING BRIEF — engine v29: none-batch (baseline-honest) (run 39)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–38 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v29-none-batch, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 6). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @29 BOUND — none-batch law (presence checks are mandatory)

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v28 → **v29**
(versions/1–29 retained; ADR `0027-engine-v29-none-batch.md`; schema VALID; 35 laws).
A surface is implemented only after a bare-amalgamation presence check passes;
force-linking ext/misc to fake presence is a SCOPE_VIOLATION.

## 2. Baseline presence checks (the heart of this run)

A no-extension compile of `sqlite3.c` alone (the factory harness baseline, NOT the shell CLI)
settled each candidate:

| Surface | Bare build | Decision |
| --- | --- | --- |
| misc-fossildelta-001 (`delta_create`) | absent | **stay none** (prior rc=0 golden was force-linked) |
| misc-utilities-001 (`eval`) | absent | **stay none** |
| introspection-vtabs-001 (`dbstat`/`dbpage`/`bytecode`) | absent | **stay none** (prior golden already rc=1) |
| misc-stmt-001 (`sqlite_stmt`) | absent | **stay none** |
| malloc-subsystem-002 (lookaside slab + variadic config) | present in C, not honestly modellable | **stay none** |
| attach-detach-003 (qualified-name-in-trigger rule) | present (core) | **implement** |

The run-38 percentile lesson generalized: several "legacy_green" misc cards were recorded
with extensions force-linked and are NOT in the pinned bare build. Leaving them none is the
honest call; this brief + ADR are the record.

## 3. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| attach-detach-003 | none | **partial** | attached-schema DDL + cross-db VIEW fixation need real attached tables (attach-001/002 partial); proven on main schema only |
| engine-none29-001 | — | **new full** | composed card for the frozen batch |
| misc-fossildelta-001 / misc-utilities-001 / introspection-vtabs-001 / malloc-subsystem-002 | none | **none (kept)** | absent-from-baseline / not-modellable, documented |

## 4. What landed

Inside a **non-TEMP** trigger body, a qualified table name (`schema.table`) in
INSERT/UPDATE/DELETE is rejected at CREATE with C's exact message and the trigger is not
created; TEMP triggers are exempt (pinned rc=0); qualified names inside a trigger SELECT are
allowed; a multi-statement body with one qualified DML fails as a whole. `split_statements`
now keeps `CREATE TEMP/TEMPORARY TRIGGER ... END` bodies whole.

Dropped honestly: `engine-none29-001-C004` (its INSERT-SELECT trigger body isn't executable
in modern's trigger engine).

## 5. Anti-cheat + cargo

- `anti_cheat_none29` — runtime table names: a qualified DML target is still rejected with
  the exact message; the unqualified twin fires and inserts.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 652/652 PASS** (was 644; +6 golden twins, +2 anti-cheat/guard). The
  `split_statements` change touched every trigger suite — all green; 622 pre-run goldens
  md5-verified intact.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 38 | Run 39 |
| --- | --- | --- |
| **full (converted)** | 130 | **131** |
| partial | 57 | 58 |
| none (remaining) | 85 | **84** |
| behaviours known | 272 | 273 |

+1 full (composed), +1 none→partial (attach-detach-003). legacy_green 182 → 183.
Deliberately small: 4 target cards were proven absent/unmodellable and left none.

## 7. Not migrated

SQLite is **not migrated**. 131/273 behaviours run honestly in modern for frozen scope only.
Four surfaces this run were left unimplemented because the pinned baseline lacks them or
they cannot be honestly modelled. Parity UNVERIFIED everywhere.

## 8. Next call

1. **attached-schema DDL** (deepen attach-detach-001/002) — would unlock the aux3/cross-db
   half of attach-detach-003.
2. **vtab-core** — a real small module/xBestIndex surface (upgrades wholenumber/completion).
3. **A genuinely-present core none** — e.g. more of the sqlite3_limit/status matrix, or
   savepoint/RELEASE flush edges; extension-heavy nones are largely absent from the baseline.
