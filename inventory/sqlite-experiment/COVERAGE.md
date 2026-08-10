# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages. Inventory progress, not migration progress.

- Generated: 2026-08-10T18:04:27Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-10T18:04:26Z by `estate-discovery-loop`

## Counts

| Metric | Count |
| --- | --- |
| Surfaces total | 91 |
| Behaviours known | 91 |
| Seeds scanned | 36 |
| Unscanned hints (residual) | 24 |
| legacy_green flags | 0 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| candidate | 91 |

## Behaviours by status

| Status | Count |
| --- | --- |
| candidate | 91 |

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
| connection-lifecycle-api | 4 |
| date-time-funcs | 4 |
| ddl-schema | 3 |
| dml-codegen | 2 |
| error-status-api | 3 |
| exec-convenience-api | 2 |
| expr-codegen | 3 |
| foreign-keys | 3 |
| json-funcs | 4 |
| loadext-api | 2 |
| name-resolution | 2 |
| pager | 2 |
| parser-grammar | 2 |
| pcache | 2 |
| pragma-surface | 2 |
| prepare-statement-api | 6 |
| printf-format | 3 |
| select-codegen | 3 |
| serialize-memdb-api | 2 |
| tokenizer | 2 |
| triggers | 2 |
| unlock-notify-api | 1 |
| upsert | 2 |
| vacuum | 2 |
| vdbe-engine | 2 |
| vfs-os-abstraction | 3 |
| wal | 2 |
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

## Unscanned hints (residual register)

- src: malloc-subsystem — allocators + memory status (src/malloc.c, src/mem0.c..mem5.c)
- src: mutex-subsystem — mutex implementations (src/mutex*.c)
- src: util-primitives — utf/random/hash/bitvec/rowset (src/utf.c, src/random.c, src/hash.c, src/bitvec.c, src/rowset.c)
- src: vtab-core — virtual table mechanism (src/vtab.c)
- src: introspection-vtabs — dbstat/dbpage/bytecode vtabs (src/dbstat.c, src/dbpage.c, src/vdbevtab.c)
- src: shell-cli — sqlite3 command-line shell (src/shell.c.in)
- src: tcl-binding — TCL language binding (src/tclsqlite.c)
- ext: fts5 — full-text search 5 (ext/fts5/)
- ext: fts3 — full-text search 3/4 legacy (ext/fts3/)
- ext: rtree — R-tree spatial index (ext/rtree/rtree.c)
- ext: geopoly — GeoJSON polygon vtab (ext/rtree/geopoly.c)
- ext: session — changesets/patchsets (ext/session/)
- ext: rbu — resumable bulk update (ext/rbu/)
- ext: recover — corrupt-db recovery API (ext/recover/)
- ext: intck — incremental integrity check (ext/intck/)
- ext: expert — index recommendation (ext/expert/)
- ext: icu — ICU collation/LIKE/upper-lower (ext/icu/)
- ext: qrf — query result formatter (ext/qrf/)
- ext: jni-binding — Java/JNI binding (ext/jni/)
- ext: wasm-binding — WASM/JS binding + fiddle (ext/wasm/)
- ext: misc-vfs-shims — VFS wrapper extensions (ext/misc: appendvfs, cksumvfs, vfsstat, vfstrace, vfslog, tmstmpvfs, mmapwarm, memtrace, pcachetrace)
- ext: misc-func-packs — SQL function extensions (ext/misc: base64, base85, basexx, decimal, ieee754, regexp, sha1, shathree, spellfix, totype, uint, uuid, fossildelta, compress, percentile, nextchar, rot13, urifuncs)
- ext: misc-vtab-packs — virtual-table extensions (ext/misc: series, csv, unionvtab, qpvtab, completion, closure, amatch, fuzzer, prefixes, wholenumber, zipfile, sqlar, stmt, templatevtab, vtablog, vtshim, btreeinfo, zorder)
- ext: misc-utilities — remaining misc utilities (ext/misc: fileio, dbdump, eval, explain, memstat, diskused, noop, normalize, randomjson, remember, stmtrand, strdup, showauth, anycollseq)

## Notes

Phase A radar only; candidates unbound. estate_scan=partial until METHOD_COVERAGE checklist addressed. src/test*.c harness adapters skipped-with-reason (see overnight/STRUCTURAL_INDEX.md).
