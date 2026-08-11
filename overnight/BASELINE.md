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

## Fingerprint status (run 4, 2026-08-11)

**Fingerprint not captured** — no build exists in this workspace and the run-4 charter forbids
spending the run compiling. Capture the compileoption dump when Test execution first builds the
pinned configuration (`./configure && make sqlite3`).

## Fingerprint CAPTURED (run 5 — Test execution RECORD, 2026-08-11)

Build: out-of-tree at /tmp/sqlite-build from this tree @ 23b6a1ed1 — `/workspace/configure && make sqlite3 sqlite3.c`
(default flags, no extra OMIT/ENABLE). Generated amalgamation/binaries NOT committed (tree does not track them).
sqlite_version(): **3.54.0**

Charter checks: **ENABLE_API_ARMOR = off** (expected off — confirmed), **OMIT_AUTORESET = off** (expected off — confirmed;
step-after-DONE therefore auto-resets in this baseline).

Full `sqlite_compileoption_get` dump (until NULL):

```
ATOMIC_INTRINSICS=1
COMPILER=clang-18.1.3
DEFAULT_AUTOVACUUM
DEFAULT_CACHE_SIZE=-2000
DEFAULT_FILE_FORMAT=4
DEFAULT_JOURNAL_SIZE_LIMIT=-1
DEFAULT_MMAP_SIZE=0
DEFAULT_PAGE_SIZE=4096
DEFAULT_PCACHE_INITSZ=20
DEFAULT_RECURSIVE_TRIGGERS
DEFAULT_SECTOR_SIZE=4096
DEFAULT_SYNCHRONOUS=2
DEFAULT_WAL_AUTOCHECKPOINT=1000
DEFAULT_WAL_SYNCHRONOUS=2
DEFAULT_WORKER_THREADS=0
DIRECT_OVERFLOW_READ
DQS=0
ENABLE_BYTECODE_VTAB
ENABLE_DBPAGE_VTAB
ENABLE_DBSTAT_VTAB
ENABLE_EXPLAIN_COMMENTS
ENABLE_FTS3
ENABLE_FTS4
ENABLE_MATH_FUNCTIONS
ENABLE_OFFSET_SQL_FUNC
ENABLE_PERCENTILE
ENABLE_RTREE
ENABLE_STMTVTAB
ENABLE_UNKNOWN_SQL_FUNCTION
HAVE_ISNAN
MALLOC_SOFT_LIMIT=1024
MAX_ATTACHED=10
MAX_COLUMN=2000
MAX_COMPOUND_SELECT=500
MAX_DEFAULT_PAGE_SIZE=8192
MAX_EXPR_DEPTH=1000
MAX_FUNCTION_ARG=1000
MAX_LENGTH=1000000000
MAX_LIKE_PATTERN_LENGTH=50000
MAX_MMAP_SIZE=0x7fff0000
MAX_PAGE_COUNT=0xfffffffe
MAX_PAGE_SIZE=65536
MAX_SCHEMA=10000000
MAX_SQL_LENGTH=1000000000
MAX_TRIGGER_DEPTH=1000
MAX_VARIABLE_NUMBER=32766
MAX_VDBE_OP=250000000
MAX_WORKER_THREADS=8
MUTEX_PTHREADS
STRICT_SUBTYPE
SYSTEM_MALLOC
TEMP_STORE=1
THREADSAFE=1
```

Note for cards/censuses: the bare-configure default is NOT feature-minimal — it enables
FTS3/FTS4, RTREE, MATH_FUNCTIONS, PERCENTILE, STMTVTAB, DBSTAT/DBPAGE/BYTECODE vtabs,
UNKNOWN_SQL_FUNCTION, and sets DQS=0 (double-quoted strings OFF, stricter than historic default).
This refines the compile-options census cards' assumptions; completeness remains incomplete.

## Two-level pin amendment (run 12, 2026-08-11) — IMPORTANT

The compileoption fingerprint above was dumped from the **sqlite3 CLI binary**, which the Makefile
builds with extra `SHELL_OPT` defines (DQS=0, ENABLE_BYTECODE_VTAB, DBPAGE, DBSTAT, EXPLAIN_COMMENTS,
FTS4, OFFSET_SQL_FUNC, PERCENTILE, RTREE, STMTVTAB, UNKNOWN_SQL_FUNCTION, STRICT_SUBTYPE). The
**characterization harnesses** compile `sqlite3.c` bare (`cc` with no `-D` flags), so the *harness
library pin* is the bare-default amalgamation: none of those SHELL_OPT features exist there, and the
Makefile-level `MATH_FUNCTIONS`/`PERCENTILE` opts are absent too. API_ARMOR and OMIT_AUTORESET are
off on both levels (verified), so all rc pins stand.

Consequences (honest record):
- `introspection-vtabs-001-C001` and `misc-percentile-001-C001` (run 11) pin **feature absence**
  (rc=1 `no such table: dbstat` / `no such function: median`) on the harness lib — annotated in
  their TRACEABILITY rows; golden bytes untouched; Rust mirrors the same absence.
- `misc-stmt` (run 12) is **DEFERRED**: freezing it positively would require compiling the harness
  with `SQLITE_ENABLE_STMTVTAB` — a compile-flag flip, forbidden by charter.
- Future runs that want the SHELL_OPT features must re-pin explicitly (new BASELINE section + new
  goldens), never silently.
