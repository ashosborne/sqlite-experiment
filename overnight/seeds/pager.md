# Seed — pager
SLICE_ID: pager
SLICE_SEED: "pager: page-level transactions, journaling modes, crash safety"
SEED_ENTRYPOINTS: sqlite3PagerOpen(), sqlite3PagerBegin(), sqlite3PagerCommitPhaseOne()
OUT_OF_SCOPE: WAL internals (wal), page cache eviction (pcache)
