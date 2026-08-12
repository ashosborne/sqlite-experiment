# MORNING BRIEF — engine v30: attached-schema ownership (run 40)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–39 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v30-attached-schema, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 14). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @30 BOUND — attached-schema law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v29 → **v30**
(versions/1–30 retained; ADR `0028-engine-v30-attached-schema.md`; schema VALID; 36 laws).
ATTACH/DETACH + cross-db fixation are core (bare-amalgamation) behaviour — every pin came
from a no-extension C harness. WAL/VACUUM/blob/conn unchanged.

## 2. What landed — a second schema is now a real object namespace

| Behaviour | Evidence |
| --- | --- |
| Ownership | `CREATE/INSERT/SELECT/UPDATE/DELETE` on `aux.t` via `schema.table` store keys; two attached schemas each own tables |
| Resolution | qualified `aux.t` + unqualified `t` (main-first, main wins collisions), `main.m`/`m` all resolve via `eval_snapshot` aliases |
| Errors | dup/reserved ATTACH → "database X is already in use"; DETACH main → "cannot detach database main"; DETACH missing → "no such database: X" |
| DETACH teardown | removes the schema and all its objects; later qualified access fails; re-ATTACH is fresh |
| Durability | file-backed aux loads on ATTACH, saves on DETACH/close; create→reopen(re-ATTACH) sees rows; main image excludes attached tables |
| pragma_database_list | now carries `(seq, name)`, main at seq 0 |
| attach-003 aux residual | non-TEMP trigger in an attached schema rejects qualified DML; attached-schema VIEW referencing another schema errors "view vv cannot reference objects in database main" |

Incidental honest fix: `ORDER BY <non-projected column>` no longer mis-sorts by column 0
(it's skipped) — which is what let `ORDER BY seq` over pragma_database_list order correctly.

## 3. Flips table

| Card | Before | After | Residual |
| --- | --- | --- | --- |
| attach-detach-001 | partial (namespace count only) | **partial (ownership real)** | URI/encryption maze, DETACH-locked edges, cross-schema txn-join |
| attach-detach-002 | partial (list only) | **partial (teardown real)** | "database is locked" DETACH not modelled |
| attach-detach-003 | partial (main only) | **partial (aux reclaimed)** | firing a trigger whose body targets an attached table (unqualified body resolution) |
| engine-attach30-001/002/003 | — | **new full ×3** | composed cards for the frozen batches |

The three attach cards stay honest partials — real ownership/teardown/fixation, with named
residuals — rather than full, because URI/lock/txn edges and attached-trigger firing remain.

## 4. Dropped honestly

- `engine-attach30-003-C003`: firing an attached-schema trigger (unqualified body resolves to
  the attached table) isn't executed by modern's trigger engine.
- `engine-attach30-002-C003`'s `sqlite_master WHERE 0` probe: sqlite_master in the eval path
  is a separate gap; the meaningful re-ATTACH-freshness checks were kept.

## 5. Anti-cheat + cargo

- `anti_cheat_attach30` — runtime schema + table name: CREATE/INSERT on `auxNNN.tNNN`, a
  runtime payload read back, then DETACH makes the qualified name disappear (rc 1).
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 668/668 PASS** (was 652; +14 golden twins, +2 anti-cheat/guard). The
  multi-schema + ORDER BY changes touched broad paths — all prior suites green; 628 pre-run
  goldens md5-verified intact.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 39 | Run 40 |
| --- | --- | --- |
| **full (converted)** | 131 | **134** |
| partial | 58 | 58 |
| none (remaining) | 84 | 84 |
| behaviours known | 273 | 276 |

+3 fulls (composed cards); the three attach partials deepened materially (residuals shrank)
without flipping full. legacy_green 183 → 186. parity_green 0.

## 7. Not migrated

SQLite is **not migrated**. 134/276 behaviours run honestly in modern for frozen scope only.
Attached schemas own tables but URI/lock/txn edges and attached-trigger firing remain; no
planner. Parity UNVERIFIED everywhere.

## 8. Next call

1. **attached-trigger firing** — unqualified body resolution at trigger execution would let
   attach-detach-003 (and triggers on attached tables) go further.
2. **vtab-core** — a real small module/xBestIndex surface.
3. **status/limit matrix** or **savepoint RELEASE flush edges** — genuinely-present cores.
