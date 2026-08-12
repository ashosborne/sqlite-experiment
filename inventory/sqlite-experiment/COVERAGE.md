# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages.
> Characterization flags (`legacy_green`, replay-green tests) ≠ done;
> use **Operator progress** below for modern-implementation status.

- Generated: 2026-08-12T19:16:53Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-13T20:30:00Z by `sqlite-engine-v32-compile-options`

## Operator progress (modern implementation)

| State | Count | Meaning |
| --- | --- | --- |
| none | 79 | Not started in modern |
| partial | 62 | Some modern execution; gaps in notes |
| full (converted) | 141 | Behaviour done in modern; parity may still be UNVERIFIED |
| deferred / rejected | 0 | Explicitly out |

### Done in modern (impl_in_modern=full)

- `auth-callback-api-002` — Column-read authorization (IGNORE yields NULL)
- `builtin-scalar-agg-funcs-002` — Aggregate function family
- `compile-options-omit-enable-001` — Compile-option diagnostics API (C + SQL)
- `date-time-funcs-001` — Core date/time conversion functions
- `date-time-funcs-002` — strftime formatting
- `date-time-funcs-003` — Modifier grammar (localtime, +N units, weekday, start of ...)
- `date-time-funcs-004` — timediff interval arithmetic
- `ddl-schema-001` — Table/view create-drop lifecycle
- `ddl-schema-002` — Index create-drop lifecycle
- `ddl-schema-003` — ALTER TABLE family (rename/add/rename-col/drop-col)
- `dml-codegen-002` — Constraint checks + ON CONFLICT resolution matrix
- `error-status-api-001` — Error introspection family
- `error-status-api-002` — Runtime limits (sqlite3_limit)
- `exec-convenience-api-001` — sqlite3_exec callback loop
- `exec-convenience-api-002` — get_table / free_table result marshalling
- `foreign-keys-001` — Immediate vs deferred FK checking
- `foreign-keys-002` — Cascading referential actions
- `loadext-api-002` — Auto-extension registry
- `malloc-subsystem-001` — Public malloc API and memory accounting
- `misc-basexx-001` — base64 + base85 + combined basexx encoders (one optional pack)
- `misc-ieee754-001` — IEEE754 float decomposition functions
- `misc-nextchar-001` — next_char() incremental completion function
- `misc-rot13-001` — rot13() function + collation
- `misc-sha1-001` — sha1() hash functions
- `misc-shathree-001` — sha3() hash functions
- `misc-totype-001` — strict conversion functions tointeger/toreal
- `misc-uint-001` — UINT collating sequence
- `misc-uuid-001` — uuid generation/conversion functions
- `misc-zorder-001` — z-order curve mapping functions
- `name-resolution-001` — Column/table name lookup and ambiguity rules
- `name-resolution-002` — ORDER BY / GROUP BY alias and ordinal resolution
- `prepare-statement-api-001` — Prepare family (v1/v2/v3, UTF-8/16, prepFlags)
- `prepare-statement-api-002` — Step execution state machine
- `prepare-statement-api-003` — Parameter binding (typed)
- `prepare-statement-api-004` — Column result access (typed, with coercions)
- `prepare-statement-api-005` — Reset / finalize / auto-reprepare on schema change
- `printf-format-001` — SQL printf()/format() functions
- `select-codegen-002` — Compound SELECT set operations
- `serialize-memdb-api-001` — Serialize / deserialize byte-image round-trip
- `triggers-001` — Trigger DDL lifecycle
- `triggers-002` — Row-trigger firing semantics
- `upsert-001` — Conflict-target resolution to unique index
- `upsert-002` — DO UPDATE / DO NOTHING execution
- `window-functions-001` — Built-in window function family
- `engine-kitchen-001` — Kitchen-spine row round-trip
- `engine-files-001` — Durable file round-trip
- `engine-files-002` — Durable multi-page tables (size/pages)
- `engine-files-003` — Durable schema objects on disk (FK/trigger/ALTER/upsert/IPK)
- `engine-files-004` — Durable catalogue file-twins
- `engine-subquery-001` — Scalar subqueries in SELECT list / WHERE incl. LIMIT-in-subquery
- `engine-subquery-002` — EXISTS / NOT EXISTS and correlated subqueries
- `engine-join-001` — INNER JOIN / comma-join equi + non-equi + aliases + empty
- `engine-join-002` — Three-table / self-join / join aggregates / GROUP BY / LEFT JOIN
- `engine-datetime-001` — Real julian-day date/time engine
- `engine-setops-001` — Compound set operations INTERSECT/EXCEPT
- `engine-constraints-001` — CHECK/NOT NULL/FK-action/name errors
- `engine-views-001` — View lifecycle + expansion
- `engine-triggers-001` — Trigger matrix BEFORE/AFTER x I/U/D + WHEN
- `engine-funcs-001` — printf/decimal/rot13-collation/scalar batch
- `engine-pragma-001` — Connection pragma get/set batch
- `engine-agg-having-001` — DISTINCT/FILTER aggregates + HAVING
- `engine-misc2-001` — Thin extension completions (sha_query/base85/ieee754-blob/decimal/uuid/printf)
- `engine-order2-001` — ORDER BY collation + NULLS placement
- `engine-fk2-001` — FK action matrix completion (ON UPDATE / SET DEFAULT)
- `engine-trig2-001` — Trigger surface completion (INSTEAD OF/DROP/RAISE/OF/recursive)
- `engine-ddl2-001` — ALTER RENAME COLUMN / DROP COLUMN
- `engine-upsert2-001` — Upsert DO UPDATE ... WHERE
- `engine-overflow-001` — Overflow page chains (durable large payloads)
- `engine-indexes-001` — On-disk UNIQUE / secondary index b-trees
- `engine-prepare-001` — Prepare + step through the shared engine
- `engine-prepare-002` — Typed bind matrix
- `engine-prepare-003` — Column accessor matrix
- `engine-window2-001` — General window-function engine
- `engine-raise-001` — RAISE family + INSTEAD OF UPDATE/DELETE
- `engine-upsert3-001` — Upsert multi-assignment
- `engine-printf3-001` — printf comma grouping + %p
- `engine-decimal2-001` — decimal_exp
- `engine-prepare2-001` — prepare_v3 / auto-reprepare / EQP
- `engine-serialize2-001` — Populated serialize/deserialize
- `engine-str2-001` — sqlite3_str completion
- `engine-txn-001` — BEGIN/COMMIT/ROLLBACK atomicity
- `engine-txn-002` — OR IGNORE inside an explicit txn
- `engine-savepoint-001` — SAVEPOINT/RELEASE/ROLLBACK TO nesting
- `engine-orrollback-001` — OR ROLLBACK conflict semantics + get_autocommit
- `engine-txnfile-001` — Durable txn reopen (commit/rollback/close)
- `engine-checkupd-001` — CHECK-on-UPDATE script scope
- `engine-checkupd-002` — OR-mode statement semantics
- `engine-checkupd-003` — Durable CHECK-on-UPDATE twins
- `engine-idxlookup-001` — Index-driven equality/range lookups
- `engine-idxlookup-002` — Multi-column/expression/partial explicit indexes
- `engine-idxfile-001` — Durable + multi-leaf indexes, C interop
- `engine-udf-001` — Scalar create_function + SQL invoke
- `engine-udf-002` — Aggregate UDF xStep/xFinal
- `engine-value-001` — sqlite3_value_*/sqlite3_result_* marshalling
- `engine-utf16-001` — UTF-16 prepare family (prepare16/_v2/_v3)
- `engine-utf16-002` — UTF-16 column/name/decltype accessors + bind_text16
- `engine-collation-001` — create_collation[_v2] + registry-driven COMPARE/ORDER BY
- `engine-collation-002` — column COLLATE + per-connection reopen honesty
- `engine-collation-003` — collation_needed lazy factory
- `engine-upsert-expr-001` — expression UNIQUE conflict targets
- `engine-upsert-expr-002` — prior conflict-target surface regression pins
- `engine-upsert-expr-003` — partial UNIQUE index targets
- `engine-wal-001` — WAL mode + sidecars + durability + C interop (composed pins)
- `engine-wal-002` — checkpoint pragmas, single-connection regime (composed pins)
- `engine-harvest23-001` — auto-extension cancel/reset/multi-entry
- `engine-harvest23-002` — malloc accounting APIs
- `engine-harvest23-003` — snprintf + str_append
- `engine-harvest23-004` — window function leftovers
- `engine-harvest23-005` — sqlite3_limit matrix + enforcement
- `engine-harvest23-006` — errstr + extended error codes
- `engine-harvest23-007` — deferred foreign keys
- `engine-harvest23-008` — authorizer statement-class codes
- `engine-harvest23-009` — sqlite3_complete nesting
- `engine-vacuum-001` — VACUUM rebuild basics
- `engine-vacuum-002` — durable VACUUM + C interop
- `engine-vacuum-003` — VACUUM INTO
- `engine-blob-001` — blob handle lifecycle
- `engine-blob-002` — incremental read/write, bounds, expiry
- `engine-blob-003` — durable blob I/O + C interop
- `engine-conn-001` — close / close_v2 handle tracking
- `engine-conn-002` — busy handling over the file write lock
- `engine-conn-003` — commit/update hooks + trace_v2
- `engine-analyze-001` — ANALYZE writes sqlite_stat1
- `engine-analyze-002` — ANALYZE coexistence (WAL/VACUUM)
- `engine-harvest28-001` — sqlite3_get_table / free_table
- `engine-harvest28-002` — sqlite3_status64 / db_status
- `engine-harvest28-003` — authorizer column-READ IGNORE
- `engine-harvest28-004` — compress / uncompress round-trips
- `engine-harvest28-006` — next_char incremental completion
- `engine-harvest28-007` — wholenumber + completion vtabs
- `engine-harvest28-008` — thin parser / complete / pragma-TVF pins
- `engine-none29-001` — qualified-name-in-trigger DML rejection
- `engine-attach30-001` — ATTACH owns tables
- `engine-attach30-002` — DETACH clears objects
- `engine-attach30-003` — attached-schema cross-db fixation
- `engine-vtab31-001` — module registration and vtab lifecycle
- `engine-vtab31-002` — declare_vtab column shape
- `engine-vtab31-003` — SELECT through the module cursor
- `engine-compile32-001` — compileoption diagnostics C + SQL
- `engine-compile32-002` — OMIT census on the pin
- `engine-compile32-003` — ENABLE census on the pin

### Partial in modern

- `analyze-stats-001` — confidence=observed-in-code; Per-index row sampling into stat tables; stat4 behind SQLITE_ENABLE_STAT4. run-37: PARTIAL - ANALYZE performs real scans and writes sqlite_stat1 in the pinned C text format (ceil selectivity with the near-1.0 rounding quirk, pinned by 11-rows/10-distinct -> "11 1"); whole-db/main/table/index scoping, re-ANALYZE replacement, DROP maintenance, WITHOUT ROWID PK pseudo-index, durable + two-direction C interop all real. RESIDUAL: sqlite_stat4 (off on the pinned build), PRAGMA optimize history, attached-schema stats, sz=/unordered annotation tokens (not emitted by pinned data).
- `attach-detach-001` — ATTACH tracked as a real namespace count; attached schemas cannot own tables/DDL yet run-40: attached schemas now OWN real tables (CREATE/INSERT/SELECT/UPDATE/DELETE via schema.table; qualified + unqualified resolution with main winning collisions; dup/reserved errors; file-backed attachments persist across reopen). RESIDUAL: URI/encryption attach maze, DETACH-locked edges, cross-schema transaction-join semantics.
- `attach-detach-002` — DETACH updates the namespace list for real; no second-schema object semantics run-40: DETACH now tears down the schema and ALL its objects (later qualified access fails); cannot-detach-main and no-such-database errors pinned; re-ATTACH gives a fresh empty schema. RESIDUAL: "database is locked" DETACH with an open statement not modelled.
- `attach-detach-003` — confidence=observed-in-code; cross-db name fixation for DDL. run-39: PARTIAL — qualified table names in a non-TEMP trigger body INSERT/UPDATE/DELETE are rejected with C's exact message (trigger not created); TEMP triggers exempt; qualified SELECT inside a trigger allowed. RESIDUAL: the attached-schema (aux3) DDL and cross-db VIEW fixation forms need real attached-schema tables, which modern lacks (attach-detach-001/002 partial); proven on the main schema only. legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED run-40: aux residual reclaimed — a non-TEMP trigger in/for an attached schema rejects qualified DML, and an attached-schema VIEW referencing another schema errors "view V cannot reference objects in database Y". RESIDUAL: firing a trigger whose body targets an attached table (unqualified body resolution at trigger execution) is not implemented.
- `auth-callback-api-001` — authorizer dispatch real but only the SQLITE_SELECT deny path implemented run-33: deny paths for INSERT/UPDATE/DELETE/CREATE_TABLE/PRAGMA landed (rc 23). REMAINING: per-object callback arguments (s1-s4 NULL today), SQLITE_IGNORE column semantics, remaining ~28 action codes.
- `backup-api-001` — backup lifecycle real for empty/trivial source DBs only
- `backup-api-002` — remaining/pagecount real for the pinned single-page sequence only
- `backup-api-003` — write-between-steps restart pinned; no general page-level coordination
- `blob-io-api-001` — confidence=observed-in-code; Opens a handle on (db,table,column,rowid); reopen repositions to a new row without re-resolving. run-35: PARTIAL - open/close/reopen/bytes real on the store path with the pinned C validation order and error text (view/table/quoted-column/indexed-write/rowid/null-type), rowid aliasing INTEGER PRIMARY KEY, text cells readable. RESIDUAL: attached-schema and UTF-16 name forms, WITHOUT ROWID targets, and open-inside-transaction interactions are unpinned.
- `blob-io-api-002` — confidence=observed-in-code; read/write at offset via shared blobReadWrite; handles expire on row modification (SQLITE_ABORT). run-35: PARTIAL - offset reads/writes, C bounds errors with untouched buffers, fixed-size writes, read-only rc 8, zeroblob preallocate + interior write, and expiry (rc 4, bytes->0) on UPDATE/DELETE all real and pinned; durable + C interop proven. RESIDUAL: expiry granularity is connection-write, not per-row like C (pins only modify the handle's own row); TEXT-cell writes via handles unpinned.
- `builtin-scalar-agg-funcs-001` — ~24 of ~60 core scalars real (adds round/trim family/replace/instr/scalar min-max/sign/char/unhex/concat/concat_ws/octet_length/unicode)
- `builtin-scalar-agg-funcs-003` — LIKE (ESCAPE + case_sensitive_like) and GLOB real; unicode case-fold edges and LIKE index optimization absent
- `compile-options-omit-enable-002` — confidence=inferred; 77 SQLITE_OMIT_* guard references in sqliteInt.h remove surfaces (auth, vtab, wal, window, ...) run 1 carded as default-present.; file-only evidence run-42: PARTIAL - pinned OMIT census via the diagnostics oracle: every probed OMIT_* gate (LOAD_EXTENSION, WAL, VIRTUALTABLE, TRIGGER, ATTACH, SUBQUERY, VIEW, AUTORESET, COMPILEOPTION_DIAGS) reports 0 and the full enumeration contains zero OMIT_-prefixed entries, matching the pin build. RESIDUAL: the 77-guard per-feature census (what each OMIT would remove) is NOT claimed; no OMIT build variants are modelled.
- `compile-options-omit-enable-003` — confidence=observed-in-code; 51 SQLITE_ENABLE_* guard references add surfaces (STAT4, DESERIALIZE, DBSTAT_VTAB, LOCKING_STYLE, ...). run-42: PARTIAL - pinned ENABLE census via the diagnostics oracle: every probed ENABLE_* gate (FTS5, FTS3, RTREE, GEOPOLY, STAT4, API_ARMOR, UNLOCK_NOTIFY, SESSION) reports 0 and the full enumeration contains zero ENABLE_-prefixed entries, matching the pin build (API_ARMOR/AUTORESET agree with the charter PIN line). RESIDUAL: the 51-guard per-feature census is NOT claimed; no ENABLE build variants are modelled.
- `connection-lifecycle-api-001` — open/close + MISUSE ordering real for :memory: and plain paths; URI parsing and open flags absent
- `connection-lifecycle-api-002` — confidence=observed-in-code; sqlite3_close fails with SQLITE_BUSY on unfinalized statements; close_v2 defers (zombie connection). run-36: PARTIAL - close refuses (rc 5, exact errmsg) while statements or blob handles live; close_v2 zombies and tears down at the last finalize/blob_close; NULL no-ops; open-txn close rolls back. RESIDUAL: unfinished sqlite3_backup coupling and the post-close MISUSE matrix are unpinned (use-after-close is UB territory - deliberately not frozen).
- `connection-lifecycle-api-003` — confidence=observed-in-code; Per-connection lock-contention callback; busy_timeout installs default sleeping handler. run-36: PARTIAL - busy_handler/busy_timeout registration, replacement and clearing real; handler retry counts and rc 5 "database is locked" pinned against a REAL in-process file write lock (BEGIN IMMEDIATE holds; COMMIT releases; commit-time flush + sibling reload). RESIDUAL: the lock model is single-process - C cross-process file locking, shared cache and unlock-notify are NOT implemented.
- `connection-lifecycle-api-004` — confidence=observed-in-code; commit_hook/update_hook/trace_v2 register observable per-connection callbacks. run-36: PARTIAL - update_hook (op/db/table/IPK-aliased rowid), commit_hook (autocommit + explicit, non-zero aborts commit with rollback), trace_v2 STMT/ROW/CLOSE/PROFILE with unset semantics all real and pinned. RESIDUAL: STMT/PROFILE fire per exec call (not per prepared statement in multi-statement scripts); WITHOUT ROWID suppression and truncate fast-path behaviour unpinned; legacy sqlite3_trace/profile not implemented.
- `dml-codegen-001` — INSERT/UPDATE/DELETE real on store + durable files; WHERE expressiveness limited vs full DML codegen
- `error-status-api-003` — confidence=observed-in-code; status64/db_status expose current/highwater counters with optional reset. run-38: PARTIAL — sqlite3_status64(MEMORY_USED, bad-op MISUSE) and sqlite3_db_status(LOOKASIDE/SCHEMA_USED, bad-op ERROR) real; SCHEMA_USED grows with objects. RESIDUAL: the rest of the status/db_status op matrix returns honest zero, not tracked.
- `expr-codegen-001` — arithmetic/concat/CAST real in a typed evaluator; full affinity matrix and collation resolution absent
- `expr-codegen-002` — 3-valued AND/OR/NOT with NULL propagation real; broader jump-codegen surface absent
- `expr-codegen-003` — IN/IS [NOT] semantics real for pinned shapes; expression-equivalence machinery absent
- `foreign-keys-003` — DROP-parent rc=19 bookkeeping real; drop-order edges beyond pins absent
- `global-init-config-001` — initialize/shutdown state machine real; OS/VFS init side effects absent
- `global-init-config-002` — pinned sqlite3_config ops real; most of the config op matrix absent
- `global-init-config-003` — pinned db_config ops real; most per-connection options absent
- `json-funcs-001` — real JSON parser + json_extract/->/->>; path grammar subset (no wildcards/#), JSONB and JSON5 absent
- `json-funcs-002` — json_set/insert/replace/patch/remove real on parsed trees; array-path mutation and JSONB absent
- `json-funcs-003` — json_valid/json_type real; json_valid flags argument and JSONB validation absent
- `json-funcs-004` — json_each over arrays/objects real as a FROM source; json_tree and full vtab columns absent
- `loadext-api-001` — shared-library dlopen sqlite3_load_extension NOT implemented; in-process sqlite3_create_function[_v2] is a DIFFERENT surface and IS done (engine-udf/engine-value)
- `misc-completion-001` — confidence=observed-in-code; Suggests keywords/schema names for interactive completion; embedded by the shell | legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED 2026-08-11 (run-11 delegated stamp) | impl_in_modern=none (run-19 scoreboard): deferred (pack v8): completion vtab not implemented in modern run-38: PARTIAL — completion(prefix) yields keyword + schema-name candidates for the prefix. RESIDUAL: full shell completion phases (functions/pragmas/collations, wildcard ranking) not claimed.
- `misc-compress-001` — confidence=observed-in-code; Deflate-based blob compression with size-prefixed format | legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED 2026-08-11 (run-12 delegated stamp) | impl_in_modern=none (run-19 scoreboard): deferred (pack v8): zlib byte-compat not implemented in modern run-38: PARTIAL — compress/uncompress round-trip for real (reversible size-prefixed RLE; repetitive data shrinks; blobs/empty handled). RESIDUAL: not zlib byte-format, so a C-written compressed blob is not cross-decodable — round-trips only.
- `misc-decimal-001` — add/sub/cmp/mul/decimal(X)/pow2/exp/collation real (exact scaled i128); true arbitrary precision absent
- `misc-func-packs-001` — decimal_mul + REGEXP of the pack execute for real (v8 defer reclaimed); remaining ~16 pack functions absent
- `misc-prefixes-001` — prefixes() real as FROM row source; vtab constraint pushdown absent
- `misc-regexp-001` — regexp operator real for literal/dot/anchor patterns; full NFA regex engine absent
- `misc-series-001` — generate_series(a,b[,step]) real as FROM row source; vtab constraint pushdown absent
- `misc-urifuncs-001` — non-URI connection answers computed from real connection state; URI parameter parsing absent
- `misc-wholenumber-001` — confidence=observed-in-code; Infinite integer sequence vtab (predecessor of generate_series) | legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED 2026-08-11 (run-11 delegated stamp) | impl_in_modern=none (run-19 scoreboard): deferred (pack v8): wholenumber vtab not implemented in modern run-38: PARTIAL — CREATE VIRTUAL TABLE ... USING wholenumber registers a bounded generator; WHERE-bounded SELECT/aggregates match C. RESIDUAL: vtab-core general module system (xBestIndex cost, unbounded scans) not implemented.
- `mutex-subsystem-001` — alloc/enter/leave/free real; pluggable mutex methods and static-mutex semantics absent
- `parser-grammar-001` — grammar subset real (pinned DDL/DML/SELECT/pragma catalogue); full parse.y productions absent
- `parser-grammar-002` — confidence=observed-in-code; %fallback lets many keywords double as identifiers — silent dialect compatibility. | legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED 2026-08-11 (run-11 delegated stamp) | impl_in_modern=none (run-19 scoreboard): keyword-fallback table absent in modern; pinned error observable reproduced by honest parse failure run-38: PARTIAL — keywords usable as UNQUOTED identifiers (key/value/offset/action as columns) real. RESIDUAL: quoted reserved words as TABLE names (FROM "select") still trip the tokenizer.
- `pragma-surface-001` — 27 of ~70 pragmas real (get/set incl. busy_timeout set-returns-value, journal_mode by backing store); rest of dispatcher absent run-32: journal_mode grew real wal/delete set semantics on files + wal_checkpoint family; card stays partial (dispatcher breadth still bounded).
- `pragma-surface-002` — table_info/foreign_key_list/index_list/database_list projections real; compile_options/function_list/module_list/pragma_list registries deferred run-38: pragma_function_list / pragma_pragma_list TVFs added (bare + parenthesized forms).
- `prepare-statement-api-006` — stmt_readonly/busy + EXPLAIN QUERY PLAN (this engine's honest nested-loop SCAN; planner-artifact EQP deliberately unfrozen) + EXPLAIN column shape real; EXPLAIN bytecode listing absent (no VDBE)
- `printf-format-002` — mprintf subset real; vmprintf/snprintf variants absent run-33: sqlite3_snprintf real (truncation/NUL/n<=0 pinned). REMAINING: sqlite3_vmprintf requires a C va_list, which stable Rust cannot define — honest platform residual.
- `printf-format-003` — str_new/appendf/appendchar/reset/length/value/errcode/finish (empty->NULL) real; raw append(z,n) and vappendf (varargs ABI) absent run-33: raw sqlite3_str_append(z,n) real. REMAINING: vappendf (same va_list platform residual).
- `select-codegen-001` — joins/subqueries/FROM depth run real (nested loop); full select.c orchestration, flattening and planner absent
- `select-codegen-003` — pinned observable executes via direct subquery evaluation; the flattening rewrite itself does not exist in modern
- `serialize-memdb-api-002` — in-memory stores are real; the memdb VFS surface (URI attach, shared named memdb) absent
- `tokenizer-001` — hex/exp/blob/bracket-ident token classes real in the eval tokenizer; full tokenize.c class coverage absent
- `tokenizer-002` — sqlite3_complete real for plain statements and simple trigger bodies; full nesting grammar absent run-33: real BEGIN/CASE/END nesting scan (nested + multi-statement trigger bodies pinned). REMAINING: string-literal-aware lexing inside complete(). run-38: sqlite3_complete now strips string literals and comments (;/END inside them no longer count) and handles quoted END in trigger bodies.
- `util-primitives-001` — confidence=observed-in-code; UTF-8/16 read/convert with invalid-sequence policy; ChaCha20-based randomness (public API); string hash tables. run-29: real UTF-8<->UTF-16 codec (surrogate pairs) now lands in modern for the prepare16/column16 surface. STILL PARTIAL: string hash tables and internal hash/PRNG primitives not implemented — do not flip to full on codec alone. legacy RECORD REPLAY_GREEN + HUMAN_ACCEPTED
- `vacuum-001` — confidence=observed-in-code; Rebuilds db into temp then swaps; applies pending page_size/auto_vacuum changes. run-34: PARTIAL — the rebuild itself is real in modern (implicit rowids renumber, IPK/WITHOUT ROWID keys kept, freelist page model reclaimed, durable rewrite C reads with integrity ok, txn ban with C errmsg). RESIDUAL: pending page_size / auto_vacuum application during VACUUM and attached-schema forms (VACUUM <schema>) are not implemented — the card names them, so full would over-claim. Old vacuum-001-C001 golden (deferred since the recognizer era) now replays for real.
- `vacuum-002` — confidence=observed-in-code; Rebuild into named URI target; source unchanged. run-34: PARTIAL — VACUUM INTO writes a fresh C-readable file (tables+indexes, integrity ok), source untouched; exists/txn/invalid-path errors match pins; :memory: export works; runtime-target anti-cheat green. RESIDUAL: URI filename target forms (file:...?...) not pinned or implemented — named in the card, so full would over-claim.
- `vtab-core-001` — confidence=observed-in-code; create_module(+v2 destructor); CREATE VIRTUAL TABLE → xCreate; reconnect → xConnect. run-41: PARTIAL - real per-connection module registry: sqlite3_create_module/_v2 (C-ABI sqlite3_module table; redefine replaces and runs the _v2 destructor, close runs remaining destructors); CREATE VIRTUAL TABLE invokes xCreate with the C argv convention (module/db/table/raw args); unknown module -> "no such module: X" exact; xCreate failure surfaces the constructor error and leaves no schema entry; DROP TABLE -> xDestroy; sqlite_master carries rootpage 0 + CREATE VIRTUAL TABLE sql; fresh connections must re-register (pinned). RESIDUAL: xConnect on schema reload for file DBs, eponymous-only modules (xCreate==NULL), sqlite3_drop_modules, deferred destructor while instances hold the module, xUpdate/xRename/xSavepoint family.
- `vtab-core-002` — confidence=observed-in-code; Column-shape declaration from inside xCreate/xConnect; constraint support flags. run-41: PARTIAL - sqlite3_declare_vtab fixes the column shape from inside xCreate/xConnect (names/types visible to SELECT and pragma table_info; MISUSE outside a constructor); HIDDEN columns excluded from SELECT * but selectable/filterable by name (checked via xColumn); typeof flows through the xColumn result API. RESIDUAL: sqlite3_vtab_config negotiation (CONSTRAINT_SUPPORT/INNOCUOUS/DIRECTONLY) not claimed; HIDDEN-column constraints via xBestIndex/xFilter argv not claimed (zero-constraint full scan is the pinned v1 contract).
- `wal-001` — confidence=observed-in-code; Frame append with commit records; readers pin mxFrame snapshots via wal-index. run-32: PARTIAL — real WAL write path (C-valid frame format, C interop proven), mode persistence, reopen recovery and single-process commit visibility landed (pack v22). RESIDUAL: commits rewrite the -wal with the full committed image (not C frame-level appends); no multi-connection mxFrame reader snapshots; no shm/wal-index locking protocol; no torn-write/corruption recovery matrix. Do not flip to full on the v22 slice.
- `wal-002` — confidence=observed-in-code; Four checkpoint modes differing in blocking and wal-reset behaviour. run-32: PARTIAL — all four modes + bare form pinned and real in the SINGLE-CONNECTION regime (backfill observable wal-blind; TRUNCATE zeroes -wal). RESIDUAL: the modes differ precisely in busy/blocking behaviour across connections, which is unexercised — full would greenwash that distinction. No wal_autocheckpoint.
- `window-functions-002` — RANGE-with-peers default, ROWS (UNBOUNDED/N PRECEDING) and GROUPS N PRECEDING real; EXCLUDE and offset RANGE absent

### Remaining (impl_in_modern=none|absent, not deferred)

- `analyze-stats-002` — analyze-stats — legacy_green no
- `btree-001` — btree — legacy_green no
- `btree-002` — btree — legacy_green no
- `expert-001` — expert — legacy_green no
- `fts3-001` — fts3 — legacy_green no
- `fts5-001` — fts5 — legacy_green no
- `fts5-002` — fts5 — legacy_green no
- `fts5-003` — fts5 — legacy_green no
- `geopoly-001` — geopoly — legacy_green no
- `icu-001` — icu — legacy_green no
- `intck-001` — intck — legacy_green no
- `introspection-vtabs-001` — introspection-vtabs — legacy_green yes
- `jni-binding-001` — jni-binding — legacy_green no
- `jni-java-surface-001` — jni-java-surface — legacy_green no
- `jni-java-surface-002` — jni-java-surface — legacy_green no
- `jni-java-surface-003` — jni-java-surface — legacy_green no
- `malloc-subsystem-002` — malloc-subsystem — legacy_green no
- `misc-amatch-001` — misc-amatch — legacy_green no
- `misc-appendvfs-001` — misc-appendvfs — legacy_green no
- `misc-btreeinfo-001` — misc-btreeinfo — legacy_green no
- `misc-cksumvfs-001` — misc-cksumvfs — legacy_green no
- `misc-closure-001` — misc-closure — legacy_green no
- `misc-csv-001` — misc-csv — legacy_green yes
- `misc-fossildelta-001` — misc-fossildelta — legacy_green yes
- `misc-fuzzer-001` — misc-fuzzer — legacy_green no
- `misc-percentile-001` — misc-percentile — legacy_green yes
- `misc-qpvtab-001` — misc-qpvtab — legacy_green no
- `misc-spellfix-001` — misc-spellfix — legacy_green no
- `misc-stmt-001` — misc-stmt — legacy_green no
- `misc-templatevtab-001` — misc-templatevtab — legacy_green no
- `misc-tmstmpvfs-001` — misc-tmstmpvfs — legacy_green no
- `misc-unionvtab-001` — misc-unionvtab — legacy_green no
- `misc-utilities-001` — misc-utilities — legacy_green yes
- `misc-vfs-shims-001` — misc-vfs-shims — legacy_green no
- `misc-vfslog-001` — misc-vfslog — legacy_green no
- `misc-vfsstat-001` — misc-vfsstat — legacy_green no
- `misc-vfstrace-001` — misc-vfstrace — legacy_green no
- `misc-vtab-packs-001` — misc-vtab-packs — legacy_green no
- `misc-vtablog-001` — misc-vtablog — legacy_green no
- `misc-vtshim-001` — misc-vtshim — legacy_green no
- `misc-zipfile-sqlar-001` — misc-zipfile-sqlar — legacy_green no
- `pager-001` — pager — legacy_green no
- `pager-002` — pager — legacy_green no
- `pcache-001` — pcache — legacy_green no
- `pcache-002` — pcache — legacy_green no
- `qrf-001` — qrf — legacy_green no
- `rbu-001` — rbu — legacy_green no
- `rbu-002` — rbu — legacy_green no
- `recover-001` — recover — legacy_green no
- `recover-002` — recover — legacy_green no
- `rtree-001` — rtree — legacy_green no
- `rtree-002` — rtree — legacy_green no
- `session-001` — session — legacy_green no
- `session-002` — session — legacy_green no
- `session-003` — session — legacy_green no
- `shell-cli-001` — shell-cli — legacy_green no
- `shell-cli-002` — shell-cli — legacy_green no
- `tcl-binding-001` — tcl-binding — legacy_green no
- `unlock-notify-api-001` — unlock-notify-api — legacy_green no
- `vdbe-engine-001` — vdbe-engine — legacy_green no
- `vdbe-engine-002` — vdbe-engine — legacy_green no
- `vfs-kv-001` — vfs-kv — legacy_green no
- `vfs-kv-002` — vfs-kv — legacy_green no
- `vfs-os-abstraction-001` — vfs-os-abstraction — legacy_green no
- `vfs-os-abstraction-002` — vfs-os-abstraction — legacy_green no
- `vfs-os-abstraction-003` — vfs-os-abstraction — legacy_green no
- `vfs-unix-variants-001` — vfs-unix-variants — legacy_green no
- `vfs-unix-variants-002` — vfs-unix-variants — legacy_green no
- `vfs-unix-variants-003` — vfs-unix-variants — legacy_green no
- `vfs-win-001` — vfs-win — legacy_green no
- `vfs-win-002` — vfs-win — legacy_green no
- `wasm-binding-001` — wasm-binding — legacy_green no
- `wasm-js-api-001` — wasm-js-api — legacy_green no
- `wasm-js-api-002` — wasm-js-api — legacy_green no
- `wasm-js-api-003` — wasm-js-api — legacy_green no
- `wasm-opfs-001` — wasm-opfs — legacy_green no
- `wasm-opfs-002` — wasm-opfs — legacy_green no
- `where-optimizer-001` — where-optimizer — legacy_green no
- `where-optimizer-002` — where-optimizer — legacy_green no

## Counts

| Metric | Count |
| --- | --- |
| Surfaces total | 237 |
| Behaviours known | 282 |
| Seeds scanned | 109 |
| Unscanned hints (residual) | 3 |
| legacy_green flags | 192 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| accepted | 52 |
| candidate | 185 |

## Behaviours by status

| Status | Count |
| --- | --- |
| converted | 140 |
| documented | 142 |

## Surfaces per slice

| Slice | Surfaces |
| --- | --- |
| analyze-stats | 2 |
| attach-detach | 3 |
| auth-callback-api | 2 |
| backup-api | 3 |
| blob-io-api | 2 |
| btree | 2 |
| builtin-scalar-agg-funcs | 3 |
| compile-options-omit-enable | 3 |
| connection-lifecycle-api | 4 |
| date-time-funcs | 4 |
| ddl-schema | 3 |
| dml-codegen | 2 |
| engine-agg-having | 1 |
| engine-analyze | 1 |
| engine-attach30 | 1 |
| engine-blob | 1 |
| engine-checkupd | 1 |
| engine-collation | 1 |
| engine-compile32 | 1 |
| engine-conn | 1 |
| engine-constraints | 1 |
| engine-datetime | 1 |
| engine-ddl2 | 1 |
| engine-decimal2 | 1 |
| engine-files | 1 |
| engine-fk2 | 1 |
| engine-funcs | 1 |
| engine-harvest23 | 1 |
| engine-harvest28 | 1 |
| engine-idxfile | 1 |
| engine-idxlookup | 1 |
| engine-indexes | 1 |
| engine-join | 1 |
| engine-kitchen | 1 |
| engine-misc2 | 1 |
| engine-none29 | 1 |
| engine-order2 | 1 |
| engine-orrollback | 1 |
| engine-overflow | 1 |
| engine-pragma | 1 |
| engine-prepare | 1 |
| engine-prepare2 | 1 |
| engine-printf3 | 1 |
| engine-raise | 1 |
| engine-savepoint | 1 |
| engine-serialize2 | 1 |
| engine-setops | 1 |
| engine-str2 | 1 |
| engine-subquery | 1 |
| engine-trig2 | 1 |
| engine-triggers | 1 |
| engine-txn | 1 |
| engine-txnfile | 1 |
| engine-udf | 1 |
| engine-upsert-expr | 1 |
| engine-upsert2 | 1 |
| engine-upsert3 | 1 |
| engine-utf16 | 1 |
| engine-vacuum | 1 |
| engine-value | 1 |
| engine-views | 1 |
| engine-vtab31 | 1 |
| engine-wal | 1 |
| engine-window2 | 1 |
| error-status-api | 3 |
| exec-convenience-api | 2 |
| expert | 1 |
| expr-codegen | 3 |
| foreign-keys | 3 |
| fts3 | 1 |
| fts5 | 3 |
| geopoly | 1 |
| global-init-config | 3 |
| icu | 1 |
| intck | 1 |
| introspection-vtabs | 1 |
| jni-binding | 1 |
| jni-java-surface | 3 |
| json-funcs | 4 |
| loadext-api | 2 |
| malloc-subsystem | 2 |
| misc-amatch | 1 |
| misc-appendvfs | 1 |
| misc-basexx | 1 |
| misc-btreeinfo | 1 |
| misc-cksumvfs | 1 |
| misc-closure | 1 |
| misc-completion | 1 |
| misc-compress | 1 |
| misc-csv | 1 |
| misc-decimal | 1 |
| misc-fossildelta | 1 |
| misc-func-packs | 1 |
| misc-fuzzer | 1 |
| misc-ieee754 | 1 |
| misc-nextchar | 1 |
| misc-percentile | 1 |
| misc-prefixes | 1 |
| misc-qpvtab | 1 |
| misc-regexp | 1 |
| misc-rot13 | 1 |
| misc-series | 1 |
| misc-sha1 | 1 |
| misc-shathree | 1 |
| misc-spellfix | 1 |
| misc-stmt | 1 |
| misc-templatevtab | 1 |
| misc-tmstmpvfs | 1 |
| misc-totype | 1 |
| misc-uint | 1 |
| misc-unionvtab | 1 |
| misc-urifuncs | 1 |
| misc-utilities | 1 |
| misc-uuid | 1 |
| misc-vfs-shims | 1 |
| misc-vfslog | 1 |
| misc-vfsstat | 1 |
| misc-vfstrace | 1 |
| misc-vtab-packs | 1 |
| misc-vtablog | 1 |
| misc-vtshim | 1 |
| misc-wholenumber | 1 |
| misc-zipfile-sqlar | 1 |
| misc-zorder | 1 |
| mutex-subsystem | 1 |
| name-resolution | 2 |
| pager | 2 |
| parser-grammar | 2 |
| pcache | 2 |
| pragma-surface | 2 |
| prepare-statement-api | 6 |
| printf-format | 3 |
| qrf | 1 |
| rbu | 2 |
| recover | 2 |
| rtree | 2 |
| select-codegen | 3 |
| serialize-memdb-api | 2 |
| session | 3 |
| shell-cli | 2 |
| tcl-binding | 1 |
| tokenizer | 2 |
| triggers | 2 |
| unlock-notify-api | 1 |
| upsert | 2 |
| util-primitives | 1 |
| vacuum | 2 |
| vdbe-engine | 2 |
| vfs-kv | 2 |
| vfs-os-abstraction | 3 |
| vfs-unix-variants | 3 |
| vfs-win | 2 |
| vtab-core | 2 |
| wal | 2 |
| wasm-binding | 1 |
| wasm-js-api | 3 |
| wasm-opfs | 2 |
| where-optimizer | 2 |
| window-functions | 2 |

## Scanned seeds

- `backup-api`
- `connection-lifecycle-api`
- `exec-convenience-api`
- `prepare-statement-api`
- `blob-io-api`
- `loadext-api`
- `serialize-memdb-api`
- `unlock-notify-api`
- `attach-detach`
- `auth-callback-api`
- `builtin-scalar-agg-funcs`
- `error-status-api`
- `date-time-funcs`
- `json-funcs`
- `pragma-surface`
- `printf-format`
- `foreign-keys`
- `triggers`
- `upsert`
- `window-functions`
- `analyze-stats`
- `ddl-schema`
- `tokenizer`
- `vacuum`
- `expr-codegen`
- `name-resolution`
- `parser-grammar`
- `select-codegen`
- `btree`
- `dml-codegen`
- `vdbe-engine`
- `where-optimizer`
- `pager`
- `pcache`
- `vfs-os-abstraction`
- `wal`
- `malloc-subsystem`
- `mutex-subsystem`
- `util-primitives`
- `vtab-core`
- `fts5`
- `introspection-vtabs`
- `shell-cli`
- `tcl-binding`
- `fts3`
- `geopoly`
- `rtree`
- `session`
- `expert`
- `intck`
- `rbu`
- `recover`
- `icu`
- `jni-binding`
- `qrf`
- `wasm-binding`
- `misc-func-packs`
- `misc-utilities`
- `misc-vfs-shims`
- `misc-vtab-packs`
- `compile-options-omit-enable`
- `vfs-kv`
- `vfs-unix-variants`
- `vfs-win`
- `global-init-config`
- `jni-java-surface`
- `wasm-js-api`
- `wasm-opfs`
- `misc-csv`
- `misc-series`
- `misc-unionvtab`
- `misc-zipfile-sqlar`
- `misc-amatch`
- `misc-closure`
- `misc-completion`
- `misc-qpvtab`
- `misc-fuzzer`
- `misc-prefixes`
- `misc-stmt`
- `misc-wholenumber`
- `misc-btreeinfo`
- `misc-templatevtab`
- `misc-vtablog`
- `misc-vtshim`
- `misc-basexx`
- `misc-sha1`
- `misc-shathree`
- `misc-zorder`
- `misc-decimal`
- `misc-ieee754`
- `misc-percentile`
- `misc-totype`
- `misc-nextchar`
- `misc-regexp`
- `misc-spellfix`
- `misc-uint`
- `misc-compress`
- `misc-fossildelta`
- `misc-rot13`
- `misc-uuid`
- `misc-appendvfs`
- `misc-cksumvfs`
- `misc-urifuncs`
- `misc-vfsstat`
- `misc-tmstmpvfs`
- `misc-vfslog`
- `misc-vfstrace`
- `engine-kitchen (composed kitchen spine, run 13 — not an estate hint)`
- `engine-files (composed durability spine, run 15)`

## Unscanned hints (residual register)

- ext: misc-mmapwarm — mmap cache pre-warming helper (ext/misc/mmapwarm.c)
- ext: misc-memtrace — malloc tracing hooks (ext/misc/memtrace.c)
- ext: misc-pcachetrace — page-cache tracing hooks (ext/misc/pcachetrace.c)

## Notes

Phase B cards: all 185 behaviours documented (10 needs-SME at slice level, 12 confidence=inferred); surfaces still candidate (behaviours-only bind, see overnight/phase-b/BIND_ALL.md); completeness incomplete. estate_scan=partial until METHOD_COVERAGE checklist addressed. src/test*.c harness adapters skipped-with-reason (see overnight/STRUCTURAL_INDEX.md).
