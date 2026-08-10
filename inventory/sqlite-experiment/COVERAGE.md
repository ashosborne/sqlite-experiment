# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages. Inventory progress, not migration progress.

- Generated: 2026-08-10T18:12:05Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-10T18:12:05Z by `estate-discovery-loop`

## Counts

| Metric | Count |
| --- | --- |
| Surfaces total | 121 |
| Behaviours known | 121 |
| Seeds scanned | 56 |
| Unscanned hints (residual) | 4 |
| legacy_green flags | 0 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| candidate | 121 |

## Behaviours by status

| Status | Count |
| --- | --- |
| candidate | 121 |

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
| expert | 1 |
| expr-codegen | 3 |
| foreign-keys | 3 |
| fts3 | 1 |
| fts5 | 3 |
| geopoly | 1 |
| icu | 1 |
| intck | 1 |
| introspection-vtabs | 1 |
| jni-binding | 1 |
| json-funcs | 4 |
| loadext-api | 2 |
| malloc-subsystem | 2 |
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
| vfs-os-abstraction | 3 |
| vtab-core | 2 |
| wal | 2 |
| wasm-binding | 1 |
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

## Unscanned hints (residual register)

- ext: misc-vfs-shims — VFS wrapper extensions (ext/misc: appendvfs, cksumvfs, vfsstat, vfstrace, vfslog, tmstmpvfs, mmapwarm, memtrace, pcachetrace)
- ext: misc-func-packs — SQL function extensions (ext/misc: base64, base85, basexx, decimal, ieee754, regexp, sha1, shathree, spellfix, totype, uint, uuid, fossildelta, compress, percentile, nextchar, rot13, urifuncs)
- ext: misc-vtab-packs — virtual-table extensions (ext/misc: series, csv, unionvtab, qpvtab, completion, closure, amatch, fuzzer, prefixes, wholenumber, zipfile, sqlar, stmt, templatevtab, vtablog, vtshim, btreeinfo, zorder)
- ext: misc-utilities — remaining misc utilities (ext/misc: fileio, dbdump, eval, explain, memstat, diskused, noop, normalize, randomjson, remember, stmtrand, strdup, showauth, anycollseq)

## Notes

Phase A radar only; candidates unbound. estate_scan=partial until METHOD_COVERAGE checklist addressed. src/test*.c harness adapters skipped-with-reason (see overnight/STRUCTURAL_INDEX.md).
