# Seed — btree
SLICE_ID: btree
SLICE_SEED: "B-tree storage layer (cursors, transactions, balancing)"
SEED_ENTRYPOINTS: sqlite3BtreeOpen(), sqlite3BtreeCursor(), sqlite3BtreeInsert()
OUT_OF_SCOPE: page cache (pager), WAL, shared-cache locking detail
