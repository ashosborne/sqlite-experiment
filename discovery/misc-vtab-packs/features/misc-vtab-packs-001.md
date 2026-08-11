# misc-vtab-packs-001 — Loadable virtual-table extension pack (~18)

Slice: `misc-vtab-packs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

generate_series, csv, zipfile/sqlar, unionvtab/swarmvtab, qpvtab, completion, closure, amatch, fuzzer, prefixes, wholenumber, stmt, templatevtab, vtablog, vtshim, btreeinfo, zorder.

## Entrypoints (citations)

- `ext/misc vtab packs` (other) — `ext/misc/series.c:939`, `ext/misc/csv.c:964`, `ext/misc/zipfile.c:2294`, `ext/misc/unionvtab.c:1371`

## Inputs / outputs / observables

- Per-extension vtab behaviour (see 17 thin cards)

## Behaviour (as implemented)

- Cluster card fully refined by 17 thin run-2 slices (misc-series ... misc-zorder) — each carries its own Phase B card
- Common shape: loadable extension registering a vtab module (often eponymous/TVF) via sqlite3_create_module in sqlite3_X_init

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- vtab-core
- loadext-api

## Assumptions / unknowns

- Umbrella card delegates; generate_series build-config question stands on misc-series card
- generate_series is commonly assumed built-in downstream — confirm build config

## Evidence

- `ext/misc/series.c:939`
- `ext/misc/csv.c:964`
- `ext/misc/zipfile.c:2294`
- `ext/misc/unionvtab.c:1371`
