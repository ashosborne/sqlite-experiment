# MORNING BRIEF — engine v33: attached-trigger fire (run 43)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–42 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v33-attached-trigger, REQUIRE_BASELINE_PRESENCE_CHECK.
MAX_NEW_CASES 55 (used 20). REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @33 BOUND — ATTACHED-TRIGGER FIRE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v32 → **v33**
(versions/1–33 retained; ADR `0031-engine-v33-attached-trigger.md`; schema VALID; 39 laws).

## 2. The pin taught the model (probe before freeze)

The first harness draft wrote `CREATE TRIGGER trg ... ON aux.t` — and bare C refused:
`trigger trg cannot reference objects in database aux`. The probe run mapped the real C
rules, and the frozen batch pins them:

- **The trigger NAME carries the schema**: `CREATE TRIGGER aux.trg ... ON t` creates an
  aux object (`aux.sqlite_master` lists it; main's counts 0). Unqualified names live in
  main.
- **ON resolves strictly in the trigger's schema**: `ON main.m` from `aux.trg` →
  cross-schema error; `ON t` with `t` only in aux from a main trigger → `no such table:
  main.t`.
- **Body targets resolve strictly in the trigger's schema — no fallback**: collision
  (`log` in both) hits `aux.log` only; `log` only in main → CREATE succeeds but FIRE
  errors `no such table: aux.log`; missing → `no such table: aux.nolog`.

## 3. What landed — run-40's firing residual is cleared

| Behaviour | Evidence |
| --- | --- |
| Fire + body write | `INSERT INTO aux.t` fires `aux.trg`; unqualified body writes real `aux.log` rows (single, multi-row, expression, WHEN-gated) |
| Collision safety | `log` in both schemas → aux trigger writes aux only; main trigger writes main only |
| Entry paths | fire through qualified `aux.t` and through unqualified `INSERT INTO t` (DML now resolves main-first then attach-order) |
| Event/timing | AFTER UPDATE, AFTER DELETE, BEFORE INSERT, BEFORE→AFTER order — all on attached tables |
| Errors | both C error shapes at CREATE; fire-time `no such table: aux.X`; run-40 fixation guard (qualified body rejected) stays green |
| Teardown | DETACH removes attached triggers (no ghost after re-ATTACH); per-schema `sqlite_master` real (bare = main only, like C) |
| Two schemas | independent triggers fire into their own logs |

Incidental honest fixes forced by pins: missing-table DML errors now carry the qualified
name (`no such table: aux.t`); a plain `SELECT` trigger-body statement compiles as a no-op
so ON validation happens at CREATE like C.

## 4. Flips table

| Card | Before | After | Why |
| --- | --- | --- | --- |
| attach-detach-003 | partial (firing residual) | **partial** (residual cleared, new precise text) | firing real; leftovers named: TEMP-trigger cross-schema fire matrix, non-INSERT trigger bodies, URI/lock/txn edges |
| attach-detach-001 | partial | partial (note) | pin-forced deepen: unqualified DML resolution into attached schemas |
| engine-attach33-001/002/003/004 | — | **full** (composed) | exact frozen batches (8 + 7 + 4 + 1 cases) |

## 5. Anti-cheat + goldens

- 20 new HUMAN_ACCEPTED goldens (`tests/characterization/engine-attach33/`), two-run
  deterministic; prior 686 goldens untouched.
- 2 anti-cheat tests: runtime schema/table/trigger names with a runtime payload written
  by the fired body into the attached log; collision + DETACH-ghost check with runtime
  values.
- SCRIPT_TABLE.len()==0; no cheat sheet — fired rows land in the real attached store.

## 6. cargo

`cargo test` (modern): **739 passed / 0 failed** (was 717; +22 attach33 twins/anti-cheat).
attach30 / none29 / compile32 / vtab31 / harvest / ANALYZE / conn / blob / vacuum / WAL all green.

## 7. Scoreboard

before → after: **141 full / 62 partial / 79 none of 282** → **145 full / 62 partial / 79 none of 286**
(the four new composed cards are the full-count movement; attach-detach-003 stays an
honest partial with its residual text rewritten).

## 8. Not migrated

SQLite is NOT migrated. Left out on purpose: TEMP-trigger cross-schema fire matrix,
trigger bodies beyond INSERT..VALUES/RAISE/no-op SELECT, URI/encryption ATTACH maze,
DETACH-locked edges, cross-schema txn joins, xBestIndex pushdown, planner, unlock-notify
(absent on pin), WAL depth, wasm/jni/vfs/FTS/rtree/session.

## 9. Next call (pick one)

1. **xBestIndex deepen** — real constraint offers to vtab modules (EQ pushdown + HIDDEN
   argv path), cutting the vtab-core-002 residual.
2. **status matrix** — sqlite3_status/status64 beyond what conn/malloc runs pinned.
3. **another present-core none** — sweep COVERAGE for the next honest none-cutter.
