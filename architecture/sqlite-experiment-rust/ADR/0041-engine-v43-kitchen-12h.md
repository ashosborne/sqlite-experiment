# ADR 0041 — engine v43: the kitchen 12-hour close (JSON array-path, pragma projections, TEMP schema)

Status: accepted (run 53, pack v43)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

Three named kitchen leftovers, probe-first, no parked cards touched. SQLite is
NOT migrated.

## JSONB probe (decides json-funcs-002)

`SELECT jsonb('[]')` returns a value and `jsonb_extract('[1]','$')` works on the
pin — **JSONB is PRESENT**. Per the charter rule, JSONB is therefore NOT
implemented this pack and json-funcs-002 **stays partial with JSONB as its one
residual**. Array-path mutation is the part that lands.

## A — JSON array paths (json-funcs-002)

Probed C and implemented on the existing tree mutators:
- `$.a[N]` / `$[N]`: set and replace overwrite an existing element; **out-of-
  range is a no-op** for set/insert/replace (only `idx == len` appends via set/
  insert); insert never overwrites an existing index.
- `json_remove` of an array index **shifts** the remainder; OOB is a no-op.
- `$.a[#]` appends; `$.a[#-K]` addresses from the end.
- nested `$.a[N].b` mutates/removes; `json_extract` reads by index.
- a **NULL document** yields NULL (not an error).
`$.*` wildcard extract returns NULL on this pin (not a multi-value form) — no
cheap win, json-funcs-001 left untouched.

## B — pragma index_list / foreign_key_list (pragma-surface-002 → FULL)

Real projections over live schema, for both `PRAGMA x(t)` and
`pragma_x('t')` TVF forms, computed once into the eval Ctx (avoids a nested
store borrow):
- **index_list**: `seq, name, unique, origin, partial` — explicit indexes
  newest-first then synthesized UNIQUE/PK autoindexes (`sqlite_autoindex_<t>_<n>`,
  origin `u`/`pk`), partial-index flag from a `WHERE`; an INTEGER PRIMARY KEY
  table reports zero rows.
- **foreign_key_list**: `id, seq, table, from, to, on_update, on_delete, match`
  — reverse declaration order, composite keys share an `id` across `seq` 0..n,
  action words `NO ACTION`/`CASCADE`/`SET NULL`, `match` `NONE`.
Both were the card's last named residual → **pragma-surface-002 flips full**.

## C — TEMP schema (composed engine-temp43)

TEMP objects now live in a real `temp` schema, never aliased to main:
- `CREATE TEMP TABLE` / `CREATE TEMPORARY TABLE` store under `temp.<name>`;
  unqualified resolution is **temp-first** (a TEMP table shadows a same-named
  main table), `main.`/`temp.` qualifiers force the schema.
- `sqlite_temp_master` is a real catalog SELECT (bare names); `sqlite_master`
  excludes temp objects.
- unqualified `DROP TABLE` removes the TEMP object first; `DROP TABLE temp.x`
  is explicit.
- `CREATE TEMP TRIGGER` now **fires** on TEMP-table DML and shows in
  `sqlite_temp_master`.
- TEMP tables/triggers are excluded from the durable file image.

## D — auth TEMP outer codes (auth-callback-api-001 stays partial)

`CREATE TEMP TABLE` fires `SQLITE_CREATE_TEMP_TABLE` (4) and dropping a TEMP
table fires `SQLITE_DROP_TEMP_TABLE` (13), both `s1 = table, s3 = temp`; DENY is
rc 23. The `sqlite_temp_master` catalog callback tail C emits is **not frozen**
(FORBID_FAKE_AUTH_MASTER). auth-001 stays partial (remaining TEMP index/trigger/
view codes and the master bookkeeping families stay named).

## Estate outcome

| Card | Outcome |
|---|---|
| pragma-surface-002 | **partial → full** — index_list + fk_list projections; residual empty |
| json-funcs-002 | stays partial — array-path landed; JSONB (pin-present) is the one residual |
| json-funcs-001 | untouched — `$.*` wildcard is not a cheap pin |
| attach-detach-003 | stays partial — TEMP-trigger fire cleared; cross-schema TEMP fire + non-trivial bodies remain |
| auth-callback-api-001 | stays partial — TEMP codes 4/13 dispatched; master tails / other TEMP codes remain |
| engine-harvest43-001..002, engine-temp43-001..002 | composed full |

Parked list unchanged (VDBE/EXPLAIN, planner/flattening, pager/btree/WAL-multi,
parse.y, va_list, dlopen, xBestIndex trio, zlib, NFA, hash/ChaCha20,
CACHE_SPILL/scanstatus, FTS/rtree/session, ENABLE-off nones).

Scoreboard: 220 full / 42 partial / 78 none of 340 → **225 full / 41 partial /
78 none of 344** (+4 composed). `SCRIPT_TABLE.len()==0`. Prior JSON / pragma TVF
/ attach33 / harvest41 / harvest42 suites green.
