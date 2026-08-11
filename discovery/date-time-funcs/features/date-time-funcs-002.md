# date-time-funcs-002 — strftime formatting

Slice: `date-time-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

%-directive formatter over parsed DateTime.

## Entrypoints (citations)

- `strftimeFunc()` (other) — `src/date.c:1463`

## Inputs / outputs / observables

- strftime output per %-directive (%d %f %H %j %J %m %M %s %S %w %W %Y %e %F %I %k %l %p %P %R %T %u %G %g %V %U)

## Behaviour (as implemented)

- strftimeFunc (src/date.c:1463) renders parsed DateTime with directive table; unknown directive → NULL result

## Validation rules found in code

- %% literal percent; NULL on bad format

## Edge cases found in code

- %s is unixepoch seconds (integer); %f seconds with fractional part SS.SSS

## Dependencies

- date-time-funcs-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/date.c:1463`
