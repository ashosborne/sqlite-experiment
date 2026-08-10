# Seed — unlock-notify-api
SLICE_ID: unlock-notify-api
SLICE_SEED: "shared-cache unlock notification (sqlite3_unlock_notify)"
SEED_ENTRYPOINTS: sqlite3_unlock_notify()
OUT_OF_SCOPE: busy handler (connection-lifecycle-api), shared-cache internals (btree)
Rationale: tiny compile-gated async-callback seam; deadlock detection logic worth its own card.
