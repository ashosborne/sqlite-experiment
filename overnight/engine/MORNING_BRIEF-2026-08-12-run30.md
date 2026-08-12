# MORNING BRIEF — engine v20: create_collation + registry-driven COLLATE (run 30)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–29 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v20-create-collation. MAX_NEW_CASES 40 (used 17).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @20 BOUND — collation-registration / registry-driven-compare laws

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v19 → **v20**
(versions/1–20 retained; ADR `0018-engine-v20-create-collation.md`; schema VALID; 23 laws).
New laws: **COLLATION-REGISTRATION LAW** (create_collation/_v2 semantics: NULL-delete,
overwrite+xDestroy, eTextRep matrix, per-connection scope, collation_needed factory) and
**REGISTRY-DRIVEN COMPARE LAW** (COLLATE name in =/range/ORDER BY and declared column
collations invoke the registered xCompare — a real C callback; closed-name-list answers
are a SCOPE_VIOLATION). WAL forbidden; SCRIPT_TABLE stays 0; `completeness: incomplete`.

## 2. APIs implemented (modern/src/{lib,eval,store}.rs)

| API | Semantics |
| --- | --- |
| `sqlite3_create_collation` / `_v2` | per-connection registry (v18 UDF-registry shape); NULL xCompare deletes; replace/delete/close fire xDestroy(pArg) — pinned counter 0/1/2 |
| eTextRep | 1/2/3/4/8 accepted; 0 and 99 → SQLITE_MISUSE (21), pinned |
| `sqlite3_collation_needed` | lazy factory invoked exactly on lookup miss; register-inside-callback works; decline leaves the C error |
| eval COLLATE resolution | builtins (BINARY/NOCASE/RTRIM + pinned rot13/uint/decimal) first, then registry → real xCompare invocation; unknown → `no such collation sequence: X` at prepare |
| Column `COLLATE` clause | `Col.coll` parsed from CREATE TABLE, persisted via create_sql (survives reopen), snapshotted into `Ctx.col_colls`; WHERE + ORDER BY honour it |
| ORDER BY | explicit COLLATE (incl. quoted names) or declared collation; unknown names error up front like C |

**rot13/uint verdict: NOT re-homed** — they stay built-in compare paths (charter allowed
either; built-ins keep the prior goldens byte-identical with zero churn). The registry is
consulted for every non-builtin name, so nothing user-registered can be shadowed except
the six builtin names, matching C's builtin precedence for the pinned scope.

## 3. Frozen cases (17 new, two-run deterministic, delegated HUMAN_ACCEPTED)

| Feature | Cases | Pins |
| --- | --- | --- |
| engine-collation-001 (register+compare) | C001–C010 | reverse collation flips ORDER BY, equality/range via xCompare, overwrite, delete-by-NULL (+later-use error), v2 xDestroy replace/close, pArg steering, eTextRep matrix, builtins unchanged, WHERE+ORDER BY composition, invocation counter |
| engine-collation-002 (schema/persistence) | C001–C005 | column COLLATE drives WHERE+ORDER BY, file reopen must re-register (error pinned), unknown-collation error shapes, case-insensitive + quoted names, RTRIM/NOCASE interaction |
| engine-collation-003 (collation_needed) | C001–C002 | factory registers lazily (statement succeeds), factory declines (error stands, name pinned) |

RECORD run `2026-08-12T1800Z-legacy-record-collation`; catalog19.json; all 428 pre-run
goldens md5-verified intact. Index/UNIQUE-with-custom-collation deliberately NOT pinned
(charter allowed skip; noted as residual on -002).

## 4. Card flips

| Card | Verdict |
| --- | --- |
| engine-collation-001 | **new, full / converted** (Batch A + anti-cheat closed) |
| engine-collation-002 | **new, full / converted** — residuals documented in notes (name-based declared-collation resolution; collation-aware UNIQUE/INDEX keys unpinned) |
| engine-collation-003 | **new, full / converted** (both factory pins frozen) |
| expr-codegen-001 | stays **partial** — broader affinity/collation resolution not claimed |
| misc-rot13-001 / misc-uint-001 | stay **full**, untouched (built-in paths, goldens green) |

## 5. Anti-cheat + cargo

- `anti_cheat_collation_runtime_order` — runtime-named reverse collation (pid-derived
  name absent from every golden) flips ORDER BY vs BINARY, ASC and DESC.
- `anti_cheat_collation_compare_called` — xCompare invocation counter ≥ 1 on ORDER BY,
  plus registry equality.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 469/469 PASS** (was 449; +17 golden twins, +3 anti-cheat/guard).
  Prior prepare / UTF-16 / UDF / index / rot13 / uint / order2 suites all green.

## 6. Scoreboard (impl_in_modern) — before → after

| State | Run 29 | Run 30 |
| --- | --- | --- |
| **full (converted)** | 85 | **88** |
| partial | 49 | 49 |
| none (remaining) | 103 | 103 |
| behaviours known | 237 | 240 |

legacy_green 147 → 150. parity_green 0. Nothing `verified`.

## 7. Not migrated

SQLite is **not migrated**. 88/240 behaviours run honestly in modern for frozen scope
only. No WAL, no planner, no VDBE, no ICU, no create_collation16/collation_needed16,
no collation-aware on-disk index keys. Parity UNVERIFIED everywhere (COMPARE never run).

## 8. Next call

1. **upsert-001 expression conflict targets** — `ON CONFLICT(expr/col-list)` matching
   against run-17 index machinery; the standing natural follow-on.
2. **errmsg16 / create_collation16 / create_function16** — finish the UTF-16 API
   end-to-end (all trivial wrappers over the run-29 codec).
3. **Collation-aware index keys** — the honest residual on engine-collation-002 if
   UNIQUE/INDEX + custom collation should be pinned.
