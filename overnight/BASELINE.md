# Baseline pin — sqlite-experiment (run 3: bind-all + Phase B deepen)

Pinned by: Ash Osborne (operator charter, run 3). Written by: sqlite-deepen-all.

## Pathfinder build

**Default Unix amalgamation build of this tree:**

```bash
./configure && make sqlite3
```

- No `-DSQLITE_OMIT_*` and no extra `-DSQLITE_ENABLE_*` beyond what `./configure` (autosetup,
  no flags) selects on Linux for this tree. This is the configuration all Phase B cards describe
  unless a card explicitly says otherwise (platform/compile-gated cards call out their gate).
- Behaviour cards for compile-gated features (e.g. `SQLITE_ENABLE_STAT4`, `SQLITE_ENABLE_DESERIALIZE`
  era gates, `SQLITE_ENABLE_LOCKING_STYLE`, dbstat/dbpage vtabs) document the gate; they do not
  assume it is on unless the default build turns it on.

## Cheap fingerprint (when a build exists)

```sql
SELECT sqlite_compileoption_get(n) FROM generate_series(0,99) WHERE sqlite_compileoption_get(n) IS NOT NULL;
SELECT sqlite_compileoption_used('ENABLE_FTS5');
```

C-level equivalents: `sqlite3_compileoption_get()` / `sqlite3_compileoption_used()`
(`src/main.c:5253` / `src/main.c:5220`). This fingerprint is the oracle for verifying any future
characterization environment matches this pin.

## Census cards stay census cards

Per charter: `compile-options-omit-enable-002` (OMIT census) and `-003` (ENABLE census) remain
**one card each** documenting the matrix. No per-flag card explosion (77 + 51 guard references
stay counted, not carded).

## Completeness

`completeness: incomplete` — this pin scopes Phase B cards; it is not an estate-completeness claim.
