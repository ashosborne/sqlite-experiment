# Seed — foreign-keys
SLICE_ID: foreign-keys
SLICE_SEED: "foreign key constraint checking + cascading actions"
SEED_ENTRYPOINTS: sqlite3FkCheck(), sqlite3FkActions()
OUT_OF_SCOPE: trigger machinery generally (triggers), index selection (where-optimizer)
