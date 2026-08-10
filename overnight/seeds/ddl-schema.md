# Seed — ddl-schema
SLICE_ID: ddl-schema
SLICE_SEED: "CREATE/DROP TABLE, VIEW, INDEX + ALTER TABLE"
SEED_ENTRYPOINTS: sqlite3StartTable(), sqlite3CreateIndex(), sqlite3AlterRenameTable()
OUT_OF_SCOPE: triggers (triggers), ANALYZE (analyze-stats), parser grammar
