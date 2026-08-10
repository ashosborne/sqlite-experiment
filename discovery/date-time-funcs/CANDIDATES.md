# CANDIDATES — date-time-funcs (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. Registry `sqlite3RegisterDateTimeFunctions` `src/date.c:1858`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Core conversions (date/time/datetime/julianday/unixepoch) | `src/date.c:1351,1309,1249,1210,1228` | Value-in/value-out functions over shared DateTime parser |
| 002 | strftime formatting | `src/date.c:1463` | Format-string surface with %-directives |
| 003 | Modifier grammar + Julian-day math | `computeJD` `src/date.c:260`; modifier application sites `src/date.c:763-898` | 'localtime', '+N days', 'weekday N' etc. — behaviour-dense |
| 004 | timediff | `src/date.c:1671` | Interval arithmetic |
