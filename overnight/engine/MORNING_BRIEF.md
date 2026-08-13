# MORNING BRIEF — engine v46: the cursor-path first split (run 57, overnight)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c · Runs 1–56 stamped alongside.
Charter: FULL_AUTONOMY, COMMIT_AS sqlite-engine-v46-leaf-split, IMPLEMENT_BTREE_LEAF_SPLIT,
FORBID_DBFILE_FALLBACK_AS_SPLIT, REQUIRE_PINNED_C_OPENS_SPLIT_FILE,
FORBID_KITCHEN_INTEGRITY_AS_PROOF. MAX_NEW_CASES 40 (used 4).

## 1. Pack @46 BOUND — SPLIT/CURSOR law

`architecture/sqlite-experiment-rust/PACK.yaml` superseded v45 → **v46**
(versions/1–46 retained; ADR `0044-engine-v46-leaf-split.md`; schema VALID; 52 laws).

## 2. Split vs C

Probed: 12 × 500-char rows overflow one 4096 leaf; C's `page_count` goes **2 → 4**, the table
root (page 2) turns **0x0d → interior 0x05**, pages 3/4 are 0x0d leaves; reopen returns all 16
rows; a post-split INSERT lands with the root still interior. Modern's cursor path now produces
the same geometry: greedy cell chunking into ≥2 leaves on appended pages, an interior root with
C's divider-cell layout (4-byte left child + largest-rowid varint, right-most in the header),
db-size header + change counter updated. Root page number never changes (schema untouched).

## 3. Cursor path vs fallback

The overflow INSERT is taken by the tree-capable cursor (`rewrite_table_leaf` returns Some) —
**not** by `dbfile::write_db_bytes` (which has done multi-leaf since disk-debt and is not a
split). Proof: a `split_count` counter moves on the overflow INSERT **and** on the post-split
write (the interior read was lifted so the second write re-splits on the cursor path), and stays
still on a `:memory:` control. The whole-image fallback remains only for shrink-below-split
(merge out of scope), deeper trees and oversized cells — named in the residual.

## 4. How C was exec'd

Not an env-gated modern-only writer:
1. **RECORD golden** `engine-btree46-002`: modern wrote the deterministic split file FIRST; the
   RECORD harness binary — the pinned C amalgamation — opened it and its read (all rowids, leaf
   probe, `integrity_check` ok, root type 5) is the frozen golden.
2. **In the cargo suite**: `exec_pinned_c` compiles (cached) and **executes** a reader against
   `/tmp/sqlite-build/sqlite3.c` (the RECORD pin) on modern-written files — the twin's integrity
   line and the runtime-payload anti-cheat both come from that C process. Kitchen
   `PRAGMA integrity_check` (a CHECK-constraint walk) is never used as proof.

## 5. btree-002 residual (stays partial)

First split landed on the cursor path. Remaining: sibling-balance matrix (the split
redistributes cells across leaves rather than balancing siblings in place), merge on DELETE
(shrink-below-split falls back), 3-level trees, index b-trees, WITHOUT ROWID, overflow chains
on the cursor path, saved-position restore; live query reads still store-based.

## 6. Scoreboard

| | before | after |
| --- | --- | --- |
| full | 229 | **231** |
| partial | 44 | 44 |
| none | 75 | 75 |
| behaviours | 348 | 350 (+2 composed) |

Composed engine-btree46-001/002 full. No estate flips (by design — residual shrink only).

## 7. Freezes / cargo

4 goldens under `tests/characterization/engine-btree46/` (001 split geometry + reopen +
post-split ×3, 002 C-reads-modern ×1), two-run deterministic, delegated HUMAN_ACCEPTED,
legacy_green 259. `cargo test` (59 binaries): **all green** including btree45, pager44,
engine-txn and the sqlite_sql_suite first slice. `SCRIPT_TABLE.len()==0`. One non-reproducible
harvest43 flake seen once, gone across five re-runs (noted in ADR; file-path-only changes this run).

## 8. Not migrated

SQLite is **not migrated**. No sibling balancing, no merge, no 3-level trees, no index b-trees,
no VDBE, no planner; live query evaluation is still the in-memory kitchen.

## 9. Next call

Page **merge** on DELETE (shrink below the split without orphaning pages — needs freelist
handling), or the **VDBE first slice** (a real opcode loop feeding this cursor) as the next
none-cut.
