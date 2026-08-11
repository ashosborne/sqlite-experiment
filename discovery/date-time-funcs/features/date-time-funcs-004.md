# date-time-funcs-004 — timediff interval arithmetic

Slice: `date-time-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Returns +/-YYYY-MM-DD HH:MM:SS.SSS delta between two datetimes.

## Entrypoints (citations)

- `timediffFunc()` (other) — `src/date.c:1671`

## Inputs / outputs / observables

- timediff(A,B) returns signed +/-YYYY-MM-DD HH:MM:SS.SSS string

## Behaviour (as implemented)

- timediffFunc (src/date.c:1671) computes calendar-aware difference: whole years then months then days/time remainder, sign from comparison

## Validation rules found in code

- NULL on unparseable operands

## Edge cases found in code

- Asymmetric around month-length boundaries by design (calendar months, not fixed 30 days)

## Dependencies

- date-time-funcs-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/date.c:1671`
