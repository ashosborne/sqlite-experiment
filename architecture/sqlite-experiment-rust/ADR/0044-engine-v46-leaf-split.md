# ADR 0044 — engine v46: the cursor-path first split (btree-002 residual shrink)

Status: accepted (run 57, pack v46)
Operator: Ash Osborne (delegated stamps, full-autonomy charter)
Pin: sqlite 3.54.0 bare amalgamation, ENABLE_API_ARMOR=off, OMIT_AUTORESET=off

## Scope

v45's cursor bailed to `dbfile::write_db_bytes` the moment one leaf was full —
and the whole-image encoder has done multi-leaf since disk-debt, so that
fallback is NOT a split. This run makes the cursor path itself split. btree-002
stays partial. SQLite is NOT migrated.

## Probe (file-backed, DELETE mode, page_size 4096)

12 × 500-char rows on top of 4 small rows: C's `page_count` goes 2 → 4, the
table root (page 2) turns from leaf 0x0d into **interior 0x05**, pages 3 and 4
are 0x0d leaves; reopen returns all 16 rows; `integrity_check` ok. A post-split
INSERT lands and the root stays interior.

## Split algorithm vs C

Mirrors the C-proven layout the disk-debt encoder already emits (probed, not
invented): when the cursor's cells no longer fit one 0x0d page,
`rewrite_table_leaf` (now tree-capable):
1. chunks the full cell bytes greedily into leaves (header 8 + 2/cell + content
   under the page),
2. keeps the table's **root page number** (schema unchanged) and rewrites it as
   an interior 0x05 — divider cells `[4-byte left child][largest-rowid varint]`,
   right-most child in the 12-byte header,
3. reuses existing child pages first and **appends** new pages at the file end,
4. updates the db-size-in-pages header field (offset 28) and the change counter.
A `split_count` counter increments; `read_table_cells` lifts the v45 0x05
refusal (one-level trees) so post-split writes stay on the cursor path
(re-split). Geometry matches C: page_count 2→4, root 0x05, two 0x0d leaves.

## How the cursor path avoids the dbfile fallback

`cursor_or_whole_image` calls the tree-capable cursor first; for the overflow
INSERT it returns `Some` (twin-asserted via `split_count` — the counter moves on
the overflow INSERT and on the post-split write, and stays still on a `:memory:`
control). The whole-image fallback remains only for scopes the cursor refuses:
shrink-below-split (merge is out of scope), deeper trees, oversized single
cells — all named in the residual.

## How C was actually exec'd (not an env-gated modern-only claim)

Two ways, both real C:
1. **RECORD golden** `engine-btree46-002-C001`: the modern crate first wrote the
   deterministic split file; the RECORD harness — the pinned C amalgamation
   itself — then opened it and its observations (all rowids, leaf probe,
   `PRAGMA integrity_check` = ok, root type 5) were frozen.
2. **In the cargo suite**: `exec_pinned_c` compiles (cached) a reader against
   `/tmp/sqlite-build/sqlite3.c` — the same pin RECORD uses — and **executes**
   it on the modern-written file; the twin takes the integrity line from that C
   process (never from the kitchen integrity walk, which is a CHECK-constraint
   pass and is not proof). The runtime-payload anti-cheat also goes through the
   exec'd C. The test fails loudly if the pin is unavailable.

## btree-002 residual (stays partial)

First split landed on the cursor path. Remaining: no sibling-balance matrix
(the split redistributes cells across leaves rather than balancing siblings in
place), no merge on DELETE (shrink-below-split falls back to the whole-image
writer), no 3-level trees, no index b-trees, no WITHOUT ROWID, no overflow-page
chains on the cursor path, no saved-position restore; live query reads still
evaluate over the in-memory store.

## Estate outcome

| Card | Outcome |
|---|---|
| btree-002 | stays **partial** — residual rewritten (single-leaf line → first split landed) |
| btree-001 / pager / pcache / vdbe / where / wal | untouched |
| engine-btree46-001..002 | composed full |

Scoreboard: 229 full / 44 partial / 75 none of 348 → **231 full / 44 partial /
75 none of 350** (+2 composed; none unchanged). `SCRIPT_TABLE.len()==0`.
btree45, pager44, engine-txn and the sqlite_sql_suite first slice all green
(59 binaries). One non-reproducible harvest43 flake was observed once in a full
parallel run and vanished across five re-runs — noted for a hardening pass, not
introduced by this run's file-path-only changes.
