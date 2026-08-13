# MORNING BRIEF — engine v44: the first internal-engine pack (rollback-journal pager) (run 55, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–54 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v44-pager, IMPLEMENT_PAGER_JOURNAL,
REQUIRE_PAGES_THROUGH_PCACHE, FORBID_FAKE_PAGER, FORBID_WHOLE_FILE_REWRITE_AS_PAGER,
DEEPEN_VDBE/BTREE/WAL false. MAX_NEW_CASES 40 (used 4).

## 1. Pack @44 BOUND — PAGER/NONE law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v43 → **v44**
(versions/1–44 retained; ADR `0042-engine-v44-pager.md`; schema VALID; 50 laws).

## 2. Journal design vs C

File-backed, `journal_mode=DELETE`. An open write transaction copies each changed page's
ORIGINAL bytes into `<db>-journal` before overwriting the page in the db file — so the journal
is **observably present during the txn** and gone after COMMIT/ROLLBACK, exactly like C.
`ROLLBACK` **replays** the journal (restores the pre-images, truncates rolled-back growth);
`COMMIT` drops it; autocommit writes run a journal-then-write-then-delete mini-txn. Not a
whole-file rewrite: page get/write is page-granular through the pager.

## 3. C-readable commit? YES

A helper (`pager44_write_committed_file`, gated on `PAGER44_OUT`) writes a committed db through
the modern pager; the **pinned C amalgamation opens it**, reads `1,one|2,two|3,three|4,pager-made-me`,
and `PRAGMA integrity_check` returns rc 0. The journal itself is Rust-private (magic `RJRNL01`),
so C hot-journal recovery is **not** pinned (stretch skipped, per charter).

## 4. pcache? coupling real, card stays none

Every page get/write routes through a methods2-shaped page cache; its xFetch/xUnpin/write
counters move under real file-txn traffic and **not** under a `:memory:` control (pinned by the
`pager44_pcache_coupling` anti-cheat). But pcache-001's named surface is the pluggable
`sqlite3_config(SQLITE_CONFIG_PCACHE2)` install seam, which is NOT implemented — so under-claim:
**pcache-001 stays none** (real coupling captured as a composed pin + ADR), pcache-002 stays none.

## 5. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 225 | **227** |
| partial | 41 | 42 |
| none | 78 | **77** |
| behaviours | 344 | 346 (+2 composed) |

partial→full: none (estate). none→partial: **pager-001**. Composed engine-pager44-001/002 full.
None dropped by 1 (the conservative outcome; pcache-001 held at none on purpose).

## 6. Freezes / cargo

4 new goldens under `tests/characterization/engine-pager44/` (001 lifecycle ×2, 002 restore/
persist ×2), two-run deterministic, delegated HUMAN_ACCEPTED, legacy_green 254. `cargo test`
(56 binaries): **all green**, including engine-txn, lookaside35, harvest43 TEMP/JSON and the
sqlite_sql_suite first slice. `SCRIPT_TABLE.len()==0`. Prior goldens untouched (the run-55
C002 golden was re-recorded with a literal UPDATE — a same-run correction, noted in ADR).

## 7. Not migrated

SQLite is **not migrated**. No two-phase commit, no hot-journal crash-recovery matrix, no WAL
pager, no btree cursors, no VDBE, no planner. This run put a real rollback journal under the
file-backed write path and nothing more.

## 8. Next call

btree-001 on this pager (page-level b-tree read/seek over the journalled file), OR the skipped
hot-journal stretch if the journal is switched to C's on-disk format (magic 0xd9d505f9..., page
records + checksums) so C can recover an interrupted txn. Then pager-002 journal-mode matrix.
