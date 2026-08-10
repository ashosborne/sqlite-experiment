# Seed — printf-format
SLICE_ID: printf-format
SLICE_SEED: "printf/format SQL function + C-level mprintf/str builder"
SEED_ENTRYPOINTS: printf() SQL function, sqlite3_mprintf(), sqlite3_str_new()
OUT_OF_SCOPE: general builtin funcs
Rationale: one formatter engine (src/printf.c) exposed via three seams: SQL function, C API, string builder.
