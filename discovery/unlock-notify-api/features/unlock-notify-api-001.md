# unlock-notify-api-001 — Unlock notification with deadlock detection

Slice: `unlock-notify-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Registers callback fired when the blocking connection unlocks; cycle detection returns SQLITE_LOCKED; fire-and-forget async seam.

## Entrypoints (citations)

- `sqlite3_unlock_notify()` (api) — `src/notify.c:148`, `src/notify.c:201`, `src/notify.c:229`, `src/notify.c:328`

## Inputs / outputs / observables

- Callback fired (possibly immediately) with array of user-args; SQLITE_LOCKED return when registration would deadlock

## Behaviour (as implemented)

- sqlite3_unlock_notify (src/notify.c:148) registers a callback against the connection currently blocking this one; fires when the blocker's transaction ends (sqlite3ConnectionUnlocked src/notify.c:229)
- Cycle detection walks the blocked-connection graph; would-be deadlock → SQLITE_LOCKED without registering

## Validation rules found in code

- Only one pending unlock-notify per connection; re-registering replaces
- Registering when not blocked invokes the callback immediately

## Edge cases found in code

- Blocker closing (sqlite3ConnectionClosed src/notify.c:328) also fires the notification
- Callbacks run holding the notify mutex — must not call back into sqlite

## Dependencies

- btree

## Assumptions / unknowns

- Requires SQLITE_ENABLE_UNLOCK_NOTIFY + shared-cache; NOT part of the default baseline build — card documents the gated contract
- SQLITE_ENABLE_UNLOCK_NOTIFY + shared-cache both required — enabled in target build?

## Evidence

- `src/notify.c:148`
- `src/notify.c:201`
- `src/notify.c:229`
- `src/notify.c:328`
