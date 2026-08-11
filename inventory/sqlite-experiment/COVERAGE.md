# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages. Inventory progress, not migration progress.

- Generated: 2026-08-11T09:30:35Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-11T09:30:35Z by `estate-discovery-loop`

## Counts

| Metric | Count |
| --- | --- |
| Surfaces total | 146 |
| Behaviours known | 146 |
| Seeds scanned | 68 |
| Unscanned hints (residual) | 3 |
| legacy_green flags | 0 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| candidate | 146 |

## Behaviours by status

| Status | Count |
| --- | --- |
| candidate | 146 |

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
| misc-func-packs | 1 |
| misc-utilities | 1 |
| misc-vfs-shims | 1 |
| misc-vtab-packs | 1 |
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

## Unscanned hints (residual register)

- ext: misc-vtab-unbundle — split run-1 misc-vtab-packs umbrella into thin named seams (new SLICE_IDs; do not thrash umbrella row)
- ext: misc-func-unbundle — split run-1 misc-func-packs umbrella into thin named seams (new SLICE_IDs; do not thrash umbrella row)
- ext: misc-vfs-unbundle — split run-1 misc-vfs-shims umbrella into thin named seams (new SLICE_IDs; do not thrash umbrella row)

## Notes

Phase A radar only; candidates unbound. estate_scan=partial until METHOD_COVERAGE checklist addressed. src/test*.c harness adapters skipped-with-reason (see overnight/STRUCTURAL_INDEX.md).
