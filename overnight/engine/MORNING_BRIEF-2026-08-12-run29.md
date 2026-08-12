# MORNING BRIEF — engine v19: UTF-16 prepare family + column16 accessors (run 29)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–28 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v19-utf16-prepare. MAX_NEW_CASES 40 (used 18).
REQUIRE_INVENTORY_BUMP honoured in-commit.

## 1. Pack @19 BOUND — UTF-16 prepare / UTF-16 column laws

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v18 → **v19**
(versions/1–19 retained; ADR `0017-engine-v19-utf16-prepare.md`; schema VALID; 21 laws).
New laws: **UTF-16 PREPARE LAW** (real codec through the shared prepare core, UTF-16
pzTail into the caller's buffer, whitespace/syntax semantics as pinned) and **UTF-16
COLUMN LAW** (text16/bytes16/name16/decltype16 derive from the same engine values as the
UTF-8 twins; bytes16 = 2 × code units; hardcoded UTF-16 blobs = SCOPE_VIOLATION).
WAL still forbidden; SCRIPT_TABLE stays 0; `completeness: incomplete`.

## 2. APIs implemented (modern/src/lib.rs + store.rs)

| API | Semantics |
| --- | --- |
| `sqlite3_prepare16` / `_v2` / `_v3` | UTF-16LE buffer (nByte in **bytes**, <0 = to NUL) decoded → shared UTF-8 prepare core; `_v3` prepFlags keep v13 honest no-op |
| UTF-16 `pzTail` | consumed UTF-8 prefix mapped back to code units; tail points into the **caller's** UTF-16 buffer (two-statement pin replays) |
| `sqlite3_column_text16` / `bytes16` | engine value rendered then encoded UTF-16; bytes16 = 2 × units (emoji = 4); NULL/before-step/done/OOR → NULL/0 |
| `sqlite3_column_name16` | colname as UTF-16 |
| `sqlite3_column_decltype` / `decltype16` | NEW declared-type tracking: `store::stmt_decltypes` parses CREATE TABLE decls; bare-column items resolve, expressions → NULL |
| `sqlite3_bind_text16` | decodes + copies (mirrors bind_text) |

**Endian/BOM decision:** native-LE UTF-16 without BOM is what the harness feeds and the
goldens pin. A BOM case was drafted but **not frozen** — BOM/endianness behaviour is not
claimed (documented in ADR 0017).

Two engine bugs surfaced by real UTF-16 input, both fixed:
- `eval::find_kw_top` panicked slicing multi-byte SQL (`'café 😀'`) — now byte-scans.
- `SELECTT 1` mis-dispatched as SELECT+junk — statement keywords now word-boundary
  matched, so the pinned `near "SELECTT": syntax error` replays.

## 3. Frozen cases (18 new, two-run deterministic, delegated HUMAN_ACCEPTED)

| Feature | Cases | Pins |
| --- | --- | --- |
| engine-utf16-001 (prepare16 family) | C001–C010 | v2/v1/v3 basics, UTF-16 pzTail, exact nByte, whitespace→OK+NULL, syntax error rc+errmsg, UTF-16 CREATE/INSERT/SELECT, `'café 😀'` literal via text/text16, bind_text16 round trip |
| engine-utf16-002 (column16) | C001–C008 | text16 coercion matrix (NULL/int/real/text/blob), bytes16 lengths (hi=4, héllo=10, 😀=4), name16, decltype16 INTEGER/TEXT/NULL-for-expr, text→text16→text, before-step/done/OOR, empty-vs-NULL, UTF-8-stored non-ASCII as text16 |

RECORD run `2026-08-12T1600Z-legacy-record-utf16`; catalog18.json; no prior golden touched
(md5 sweep of all 410 pre-run goldens verified intact).

## 4. prepare-statement-api-001 verdict: **FLIPPED to full / converted**

All three UTF-16 prepare variants work for the frozen scope (v1/v2/v3 as pinned), closing
the card's sole named gap (UTF-8 prepare_v2/v3 + prepFlags/auto-reprepare/EXPLAIN landed
v13/v14). `parity: UNVERIFIED` — Verification has not run.

## 5. util-primitives-001 note (not greenwashed)

Real UTF-8↔UTF-16 codec now lands in modern for the prepare16/column16 surface; notes
updated. Card **stays partial**: string hash tables and internal hash/PRNG primitives are
still absent — codec alone was never enough for full.

## 6. Anti-cheat + cargo

- `anti_cheat_prepare16_runtime` — prepare16_v2 of runtime-built SQL (pid-seeded literals
  absent from every golden); step/column match computed values.
- `anti_cheat_column_text16_roundtrip` — runtime `Zürich😀-<pid>` bound via bind_text16,
  read back via text16 + bytes16 + UTF-8 twin.
- `script_table_still_empty` — SCRIPT_TABLE.len() == 0.
- **cargo test: 449/449 PASS** (was 428; +18 golden twins, +3 anti-cheat/guard).
  All prior prepare / column / UDF / file / index suites green.

## 7. Scoreboard (impl_in_modern) — before → after

| State | Run 28 | Run 29 |
| --- | --- | --- |
| **full (converted)** | 82 | **85** |
| partial | 50 | 49 |
| none (remaining) | 103 | 103 |
| behaviours known | 235 | 237 |

Flips: prepare-statement-api-001 partial→full; new full cards engine-utf16-001/002.
prepare-statement-api-004 stays full (UTF-16 twins noted — no double-count).
legacy_green 129 → 147. parity_green 0. Nothing `verified`.

## 8. Not migrated

SQLite is **not migrated**. 85/237 behaviours run honestly in modern for frozen scope
only. No WAL, no cost-based planner, no VDBE, no dlopen/load_extension, no
create_function16, no errmsg16. Parity UNVERIFIED everywhere (COMPARE never run).

## 9. Next call

Natural follow-ons, in order of yield:
1. **upsert-001 expression conflict targets** — `ON CONFLICT(expr)` / conflict-target
   matching against the run-17 index machinery; bounded parser+store work.
2. **create_collation** — per-connection collation registry mirroring the v18 UDF
   registry shape; unlocks collation-needed ORDER BY partials.
3. errmsg16 / create_function16 if the UTF-16 surface should finish end-to-end.
