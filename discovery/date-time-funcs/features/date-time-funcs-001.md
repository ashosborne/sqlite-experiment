# date-time-funcs-001 — Core date/time conversion functions

Slice: `date-time-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

date/time/datetime/julianday/unixepoch over shared DateTime parse + computeJD.

## Entrypoints (citations)

- `dateFunc()/datetimeFunc()` (other) — `src/date.c:1351`, `src/date.c:1249`, `src/date.c:1210`, `src/date.c:1228`, `src/date.c:1858`

## Inputs / outputs / observables

- Canonical output formats: date() YYYY-MM-DD, time() HH:MM:SS, datetime() combined, julianday() float, unixepoch() integer

## Behaviour (as implemented)

- All parse the same time-value grammar (ISO-8601 text, julian day number, 'now', unixepoch with modifier) into DateTime then render (src/date.c:1351,1309,1249,1210,1228)
- 'now' comes from the VFS xCurrentTimeInt64 and is stable within a statement

## Validation rules found in code

- Unparseable input → NULL (not error)
- Year range ~0000-9999; out-of-range → NULL

## Edge cases found in code

- Leap-second-free model; 2-digit fractional seconds preserved via subsec modifier only

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/date.c:1351`
- `src/date.c:1249`
- `src/date.c:1210`
- `src/date.c:1228`
- `src/date.c:1858`
