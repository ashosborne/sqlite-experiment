# Seed — triggers
SLICE_ID: triggers
SLICE_SEED: "CREATE TRIGGER DDL + row-trigger firing"
SEED_ENTRYPOINTS: sqlite3BeginTrigger(), sqlite3CodeRowTrigger()
OUT_OF_SCOPE: FK action triggers (foreign-keys), general DDL (ddl-schema)
