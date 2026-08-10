# Seed — wal
SLICE_ID: wal
SLICE_SEED: "write-ahead log: frames, snapshots, checkpoints"
SEED_ENTRYPOINTS: sqlite3WalOpen(), sqlite3WalFrames(), sqlite3WalCheckpoint()
OUT_OF_SCOPE: pager integration points (pager), shared-memory OS plumbing (vfs-os-abstraction)
