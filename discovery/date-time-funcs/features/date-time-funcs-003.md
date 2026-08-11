# date-time-funcs-003 — Modifier grammar (localtime, +N units, weekday, start of ...)

Slice: `date-time-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Sequential modifier application with Julian-day arithmetic; localtime uses OS tz conversion.

## Entrypoints (citations)

- `computeJD()+modifiers` (other) — `src/date.c:260`, `src/date.c:763`, `src/date.c:821`, `src/date.c:898`

## Inputs / outputs / observables

- Result shifts per modifier: '+N days/hours/...', 'start of month/year/day', 'weekday N', 'localtime'/'utc', 'floor'/'ceiling', 'subsec'

## Behaviour (as implemented)

- Modifiers applied strictly left-to-right over computeJD state (src/date.c:260, application sites :763-898); month/year arithmetic normalizes overflow (Jan 31 +1 month → Mar 2/3 depending on year) unless floor/ceiling modifiers adjust
- localtime/utc convert via OS localtime_r (VFS-independent host tz)

## Validation rules found in code

- Unknown modifier → NULL result

## Edge cases found in code

- 'weekday N' advances 0-6 days forward to the requested weekday (never backward)
- DST transitions make localtime non-invertible for one hour/year each way — characterization must pin TZ (run-1 flag)

## Dependencies

- vfs-os-abstraction

## Assumptions / unknowns

- localtime depends on host tz database — characterization must pin TZ

## Evidence

- `src/date.c:260`
- `src/date.c:763`
- `src/date.c:821`
- `src/date.c:898`
