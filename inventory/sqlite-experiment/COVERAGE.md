# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages.
> Characterization flags (`legacy_green`, replay-green tests) ≠ done;
> use **Operator progress** below for modern-implementation status.

- Generated: 2026-08-11T20:54:11Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-11T20:54:10Z by `sqlite-engine-v10-completion-sweep`

## Operator progress (modern implementation)

| State | Count | Meaning |
| --- | --- | --- |
| none | 103 | Not started in modern |
| partial | 72 | Some modern execution; gaps in notes |
| full (converted) | 26 | Behaviour done in modern; parity may still be UNVERIFIED |
| deferred / rejected | 0 | Explicitly out |

### Done in modern (impl_in_modern=full)

- `date-time-funcs-001` — Core date/time conversion functions
- `date-time-funcs-002` — strftime formatting
- `date-time-funcs-003` — Modifier grammar (localtime, +N units, weekday, start of ...)
- `date-time-funcs-004` — timediff interval arithmetic
- `ddl-schema-001` — Table/view create-drop lifecycle
- `exec-convenience-api-001` — sqlite3_exec callback loop
- `misc-rot13-001` — rot13() function + collation
- `misc-zorder-001` — z-order curve mapping functions
- `name-resolution-001` — Column/table name lookup and ambiguity rules
- `select-codegen-002` — Compound SELECT set operations
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

### Partial in modern

- `attach-detach-001` — ATTACH tracked as a real namespace count; attached schemas cannot own tables/DDL yet
- `attach-detach-002` — DETACH updates the namespace list for real; no second-schema object semantics
- `auth-callback-api-001` — authorizer dispatch real but only the SQLITE_SELECT deny path implemented
- `backup-api-001` — backup lifecycle real for empty/trivial source DBs only
- `backup-api-002` — remaining/pagecount real for the pinned single-page sequence only
- `backup-api-003` — write-between-steps restart pinned; no general page-level coordination
- `builtin-scalar-agg-funcs-001` — ~24 of ~60 core scalars real (adds round/trim family/replace/instr/scalar min-max/sign/char/unhex/concat/concat_ws/octet_length/unicode)
- `builtin-scalar-agg-funcs-002` — count/sum/total/avg/min/max/group_concat real; DISTINCT aggregates and FILTER absent
- `builtin-scalar-agg-funcs-003` — LIKE (ESCAPE + case_sensitive_like) and GLOB real; unicode case-fold edges and LIKE index optimization absent
- `connection-lifecycle-api-001` — open/close + MISUSE ordering real for :memory: and plain paths; URI parsing and open flags absent
- `ddl-schema-002` — CREATE INDEX tracked, UNIQUE enforced in-session; no real index b-trees or on-disk UNIQUE autoindexes
- `ddl-schema-003` — ALTER RENAME TO / ADD COLUMN DEFAULT real incl. durable; RENAME COLUMN and DROP COLUMN absent
- `dml-codegen-001` — INSERT/UPDATE/DELETE real on store + durable files; WHERE expressiveness limited vs full DML codegen
- `dml-codegen-002` — IGNORE/REPLACE/ABORT/FAIL + CHECK/NOT NULL/UNIQUE/FK real; OR ROLLBACK absent (no transactions), CHECK-on-UPDATE unpinned
- `error-status-api-001` — errcode/extended_errcode/errmsg real for implemented error paths; errstr and full extended-code matrix absent
- `error-status-api-002` — sqlite3_limit get/set with prior-value semantics real for the pinned limit id only
- `expr-codegen-001` — arithmetic/concat/CAST real in a typed evaluator; full affinity matrix and collation resolution absent
- `expr-codegen-002` — 3-valued AND/OR/NOT with NULL propagation real; broader jump-codegen surface absent
- `expr-codegen-003` — IN/IS [NOT] semantics real for pinned shapes; expression-equivalence machinery absent
- `foreign-keys-001` — immediate FK enforcement real (memory + durable); deferred FKs absent
- `foreign-keys-002` — ON DELETE CASCADE + SET NULL + RESTRICT real; SET DEFAULT and ON UPDATE actions absent
- `foreign-keys-003` — DROP-parent rc=19 bookkeeping real; drop-order edges beyond pins absent
- `global-init-config-001` — initialize/shutdown state machine real; OS/VFS init side effects absent
- `global-init-config-002` — pinned sqlite3_config ops real; most of the config op matrix absent
- `global-init-config-003` — pinned db_config ops real; most per-connection options absent
- `json-funcs-001` — real JSON parser + json_extract/->/->>; path grammar subset (no wildcards/#), JSONB and JSON5 absent
- `json-funcs-002` — json_set/insert/replace/patch/remove real on parsed trees; array-path mutation and JSONB absent
- `json-funcs-003` — json_valid/json_type real; json_valid flags argument and JSONB validation absent
- `json-funcs-004` — json_each over arrays/objects real as a FROM source; json_tree and full vtab columns absent
- `loadext-api-001` — enable-gate + not-authorized path real; actual shared-library loading absent by design
- `loadext-api-002` — auto-extension register/invoke on open real; cancel/reset entry points absent
- `malloc-subsystem-001` — malloc64/free/msize real allocator; memory accounting (memory_used/highwater) absent
- `misc-basexx-001` — base64 encode/decode real; base85 and combined basexx absent
- `misc-decimal-001` — decimal_add/sub/cmp/mul real (exact scaled i128, mul trims trailing zeros); decimal(X)/pow2/collation absent
- `misc-func-packs-001` — decimal_mul + REGEXP of the pack execute for real (v8 defer reclaimed); remaining ~16 pack functions absent
- `misc-ieee754-001` — ieee754/ieee754_mantissa/ieee754_exponent (+2-arg form) real; from_blob/to_blob absent
- `misc-prefixes-001` — prefixes() real as FROM row source; vtab constraint pushdown absent
- `misc-regexp-001` — regexp operator real for literal/dot/anchor patterns; full NFA regex engine absent
- `misc-series-001` — generate_series(a,b[,step]) real as FROM row source; vtab constraint pushdown absent
- `misc-sha1-001` — sha1(x) real digest; sha1_query absent
- `misc-shathree-001` — sha3(x,bits) real Keccak; sha3_query absent
- `misc-totype-001` — tointeger/toreal strict conversion real for pinned forms; blob/overflow edges unpinned
- `misc-uint-001` — uint comparison real in expressions (COLLATE uint); registered collation for ORDER BY/indexes absent
- `misc-urifuncs-001` — non-URI connection answers computed from real connection state; URI parameter parsing absent
- `misc-uuid-001` — uuid() v4 via PRNG + uuid_str/uuid_blob round-trip real; RFC-variant edges unpinned
- `mutex-subsystem-001` — alloc/enter/leave/free real; pluggable mutex methods and static-mutex semantics absent
- `name-resolution-002` — ORDER BY alias/ordinal + GROUP BY expressions real; COLLATE terms and NULLS FIRST/LAST absent
- `parser-grammar-001` — grammar subset real (pinned DDL/DML/SELECT/pragma catalogue); full parse.y productions absent
- `pragma-surface-001` — 27 of ~70 pragmas real (get/set incl. busy_timeout set-returns-value, journal_mode by backing store); rest of dispatcher absent
- `pragma-surface-002` — table_info/foreign_key_list/index_list/database_list projections real; compile_options/function_list/module_list/pragma_list registries deferred
- `prepare-statement-api-001` — prepare rc/error paths real; UTF-16 variants and prepFlags absent; no real SQL compilation in prepare
- `prepare-statement-api-002` — step state machine + autoreset semantics real for pinned statements; general VDBE execution absent
- `prepare-statement-api-003` — bind_int with range checking real; other typed binds absent
- `prepare-statement-api-004` — column_int/text/type with pinned coercions real; full typed-access matrix absent
- `prepare-statement-api-005` — reset/finalize + OMIT_AUTORESET=off behaviour real; auto-reprepare on schema change absent
- `prepare-statement-api-006` — stmt_readonly/stmt_busy real; explain introspection absent
- `printf-format-001` — flags/width/precision + d,i,u,f,e,E,g,G,x,X,o,s,c,q,Q real; %w, positional args, # flag absent
- `printf-format-002` — mprintf subset real; vmprintf/snprintf variants absent
- `printf-format-003` — str_new/appendf/errcode/finish real; appendchar/reset and grow edges absent
- `select-codegen-001` — joins/subqueries/FROM depth run real (nested loop); full select.c orchestration, flattening and planner absent
- `select-codegen-003` — pinned observable executes via direct subquery evaluation; the flattening rewrite itself does not exist in modern
- `serialize-memdb-api-001` — serialize/deserialize + FREEONCLOSE ownership real for empty images; populated-image round-trip absent
- `serialize-memdb-api-002` — in-memory stores are real; the memdb VFS surface (URI attach, shared named memdb) absent
- `tokenizer-001` — hex/exp/blob/bracket-ident token classes real in the eval tokenizer; full tokenize.c class coverage absent
- `tokenizer-002` — sqlite3_complete real for plain statements and simple trigger bodies; full nesting grammar absent
- `triggers-001` — BEFORE/AFTER x INSERT/UPDATE/DELETE + WHEN parse and persist; INSTEAD OF and DROP TRIGGER absent
- `triggers-002` — old.*/new.* bindings, WHEN filtering, per-row B-then-A firing real; RAISE(), recursive triggers, UPDATE OF absent
- `upsert-001` — conflict-target to PK/UNIQUE column real; index-expression targets and target WHERE absent
- `upsert-002` — DO NOTHING / DO UPDATE SET excluded.col real; UPDATE-set breadth and WHERE on DO UPDATE absent
- `util-primitives-001` — PRNG (sqlite3_randomness) real; UTF codecs and hash primitives absent
- `window-functions-001` — row_number + sum OVER real for pinned shapes; the built-in window family breadth absent
- `window-functions-002` — ROWS BETWEEN 1 PRECEDING AND CURRENT ROW real; RANGE/GROUPS/EXCLUDE absent

### Remaining (impl_in_modern=none|absent, not deferred)

- `analyze-stats-001` — analyze-stats — legacy_green yes
- `analyze-stats-002` — analyze-stats — legacy_green no
- `attach-detach-003` — attach-detach — legacy_green yes
- `auth-callback-api-002` — auth-callback-api — legacy_green no
- `blob-io-api-001` — blob-io-api — legacy_green no
- `blob-io-api-002` — blob-io-api — legacy_green no
- `btree-001` — btree — legacy_green no
- `btree-002` — btree — legacy_green no
- `compile-options-omit-enable-001` — compile-options-omit-enable — legacy_green no
- `compile-options-omit-enable-002` — compile-options-omit-enable — legacy_green no
- `compile-options-omit-enable-003` — compile-options-omit-enable — legacy_green no
- `connection-lifecycle-api-002` — connection-lifecycle-api — legacy_green no
- `connection-lifecycle-api-003` — connection-lifecycle-api — legacy_green no
- `connection-lifecycle-api-004` — connection-lifecycle-api — legacy_green no
- `error-status-api-003` — error-status-api — legacy_green no
- `exec-convenience-api-002` — exec-convenience-api — legacy_green no
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
- `misc-completion-001` — misc-completion — legacy_green yes
- `misc-compress-001` — misc-compress — legacy_green yes
- `misc-csv-001` — misc-csv — legacy_green yes
- `misc-fossildelta-001` — misc-fossildelta — legacy_green yes
- `misc-fuzzer-001` — misc-fuzzer — legacy_green no
- `misc-nextchar-001` — misc-nextchar — legacy_green yes
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
- `misc-wholenumber-001` — misc-wholenumber — legacy_green yes
- `misc-zipfile-sqlar-001` — misc-zipfile-sqlar — legacy_green no
- `pager-001` — pager — legacy_green no
- `pager-002` — pager — legacy_green no
- `parser-grammar-002` — parser-grammar — legacy_green yes
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
- `vacuum-001` — vacuum — legacy_green yes
- `vacuum-002` — vacuum — legacy_green no
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
- `vtab-core-001` — vtab-core — legacy_green no
- `vtab-core-002` — vtab-core — legacy_green no
- `wal-001` — wal — legacy_green no
- `wal-002` — wal — legacy_green no
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
| Surfaces total | 196 |
| Behaviours known | 201 |
| Seeds scanned | 109 |
| Unscanned hints (residual) | 3 |
| legacy_green flags | 111 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| accepted | 11 |
| candidate | 185 |

## Behaviours by status

| Status | Count |
| --- | --- |
| converted | 26 |
| documented | 175 |

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
| engine-constraints | 1 |
| engine-datetime | 1 |
| engine-files | 1 |
| engine-funcs | 1 |
| engine-join | 1 |
| engine-kitchen | 1 |
| engine-pragma | 1 |
| engine-setops | 1 |
| engine-subquery | 1 |
| engine-triggers | 1 |
| engine-views | 1 |
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
