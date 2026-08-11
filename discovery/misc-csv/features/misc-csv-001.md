# misc-csv-001 — CSV file virtual table

Slice: `misc-csv` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Reads external CSV files as tables (filename= or data= args); filesystem access from SQL — security-relevant

## Entrypoints (citations)

- `sqlite3_csv_init()` (other) — `ext/misc/csv.c:964`

## Inputs / outputs / observables

- CREATE VIRTUAL TABLE t USING csv(filename=...|data=..., header=, columns=, schema=); rows parsed per RFC-4180-ish rules

## Behaviour (as implemented)

- csv vtab (init ext/misc/csv.c:964) reads file or inline data; header row optionally names columns; all values TEXT; full-scan only (no index pushdown)

## Validation rules found in code

- filename and data mutually exclusive; columns= count enforced

## Edge cases found in code

- Quoted fields with embedded newlines/commas/quotes; CRLF tolerance

## Dependencies

- vtab-core
- loadext-api

## Assumptions / unknowns

- Filesystem access from SQL — security posture flag stands
- Filesystem-access posture downstream

## Evidence

- `ext/misc/csv.c:964`
