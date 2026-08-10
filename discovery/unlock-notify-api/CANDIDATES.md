# CANDIDATES — unlock-notify-api (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | unlock_notify registration + deadlock detection | `src/notify.c:148`; blocked/unlocked/closed hooks `src/notify.c:201,229,328` | Async callback fired when blocking connection releases; detects wait cycles (SQLITE_LOCKED) |
