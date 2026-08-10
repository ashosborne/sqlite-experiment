# Seed — session
SLICE_ID: session
SLICE_SEED: "session extension: change tracking, changesets/patchsets, apply/conflict"
SEED_ENTRYPOINTS: sqlite3session_create(), sqlite3changeset_apply()
OUT_OF_SCOPE: changeset CLI tools (recorded in structural index), fuzz harness (skipped-with-reason)
