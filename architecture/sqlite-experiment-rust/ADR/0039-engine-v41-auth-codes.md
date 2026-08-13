# ADR 0039 — engine v41: authorizer action codes without sqlite_master fakery

Status: accepted (run 51, pack v41)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

auth-callback-api-001's residual after v37/v39 was "~22 action codes not
dispatched". This run grew the dispatch to every outer code the kitchen
actually runs, pinned with probed s1–s4 strings and per-code IGNORE/DENY
effects — and froze **nothing** that lists a sqlite_master /
sqlite_temp_master bookkeeping walk modern does not perform. The card **stays
partial**. SQLite is NOT migrated.

## Per-code probe record (pinned bare build, two-run deterministic)

| Code | C's log shape | IGNORE | DENY | Outcome |
|---|---|---|---|---|
| FUNCTION 31 | `[21][31\|~\|abs\|~\|~][20\|t\|a\|main\|~]` — compile time, once per occurrence, outer-then-inner for `coalesce(min(a),999)`; `count(*)` reads the table with an EMPTY column and NULL schema `[20\|t\|{}\|~\|~]` | the function yields NULL and **de-aggregates**: `SELECT min(a) FROM t` returns 3 NULL rows; `coalesce(min(a),999)` → 999 per row; the column read degrades to the empty-column form | prepare fails at **plain rc 1** with `not authorized to use function: NAME` (also for a nested call, naming the inner function) | LANDED (41-001) |
| SAVEPOINT 32 | `[32\|BEGIN\|s1]`, `[32\|RELEASE\|s1]`, `[32\|ROLLBACK\|s2]` — s1 is the verb, s2 the savepoint name; plain BEGIN/ROLLBACK stay code 22 | n/a | generic rc 23; fires before the savepoint-exists check (denying `RELEASE` of a missing name is still rc 23) | LANDED (41-002) |
| ANALYZE 28 | `[28\|t\|~\|main\|~]` per analyzed table | n/a | rc 23 blocks | LANDED outer (41-003) — the sqlite_stat1 READ/DELETE tail C also emits is **not frozen** |
| ALTER_TABLE 26 | `[26\|main\|t\|~\|~]` — s1/s2 **inverted** vs CREATE_TABLE (s1 = database, s2 = table); fires for RENAME and ADD COLUMN | rc 0, rename silently **no-ops** (old name survives) | rc 23 | LANDED outer (41-004) — C's ~60-event sqlite_master/sqlite_temp_master rename walk is **not frozen** |
| DROP_TABLE 11 | `[11\|t\|~\|main\|~]` | n/a | rc 23, table survives | LANDED outer (41-005) — the surrounding master DELETEs are **not frozen** |
| CREATE_VTABLE 29 / DROP_VTABLE 30 | `[29\|vt\|ser\|main\|~]` / `[30\|vt\|ser\|main\|~]` (s2 = module; a vtab drop fires 30, not 11) | n/a | rc 23, object not created / not dropped | LANDED outer (41-005) |
| view s4 | `[21][20\|t\|a\|main\|v][20\|t\|b\|main\|v][20\|v\|a\|main\|~]…[21\|~\|~\|~\|v]` — the WHOLE view body's base reads carry s4 = view regardless of the outer projection; then the projected view columns with s4 NULL; then a nested SELECT consult | — | — | LANDED (41-006) |
| trigger s4 | `[18\|t\|~\|main\|~][18\|log\|~\|main\|tr][20\|t\|a\|main\|tr]` — body events carry s4 = trigger name | — | — | LANDED (41-006) |

## Skipped / presence-check record

- **REINDEX 27** — C fires per index including `sqlite_autoindex_*`; the kitchen
  does not parse REINDEX. No fake rebuild invented. Stays named.
- **RECURSIVE 33** — scan-gated (unused `WITH RECURSIVE` is silent; a scanned
  CTE fires `[33\|~\|~\|~\|c]` among s4=c SELECT consults). Modern has no
  recursive-CTE execution — **pin-absent**, not implemented, stays named.
- **CREATE_INDEX 1 / DROP_INDEX 10, CREATE_VIEW 8 / DROP_VIEW 17,
  CREATE_TRIGGER 7 / DROP_TRIGGER 16** — their C logs are dominated by
  sqlite_master INSERT/UPDATE/DELETE/READ bookkeeping (the auth2-2.1 shape).
  Freezing them would require inventing catalog DML modern does not perform.
  Forbidden by the v41 law; stay named.
- **TEMP family (3–6, 12–15)** — distinct temp catalog; not aliased to the
  main-schema codes; stays named.

## Implementation notes

Dispatch grew inside the real per-statement consult (store::auth_stmt_precheck)
plus a compile-time SELECT consult on the prepare path (FUNCTION events + the
view s4 walk fire at prepare, like C). The per-statement IGNORE set makes
ignored functions evaluate to NULL without touching their arguments — which
also de-aggregates them and degrades the table read to the empty-column form.
The read prepass now expands only BARE `*` items (`count(*)` no longer reads
every column) and adds the empty-column fallback.

**Pre-existing engine bug found by the coalesce pin:** scalar functions over
aggregates (`coalesce(min(a),999)`) evaluated the aggregate per-row instead of
aggregating. Fixed (aggregate-context argument evaluation).

## Estate outcome

auth-callback-api-001 **stays partial** — residual rewritten to name exactly:
the DDL sqlite_master bookkeeping families (index/view/trigger full logs and
the unfrozen catalog tails), TEMP, REINDEX (unparsed), RECURSIVE (pin-absent).
Composed engine-harvest41-001..006 full for the frozen batches.
auth-callback-api-002 untouched (already full).

Scoreboard: 208 full / 42 partial / 78 none of 328 → **214 full / 42 partial /
78 none of 334** (+6 composed). `SCRIPT_TABLE.len()==0`. Run-47 auth goldens
(s1–s4, DELETE-proceeds), auth-002 IGNORE→NULL, and all prior suites green.
