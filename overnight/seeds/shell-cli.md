# Seed — shell-cli
SLICE_ID: shell-cli
SLICE_SEED: "sqlite3 interactive shell / CLI"
SEED_ENTRYPOINTS: main() in src/shell.c.in
OUT_OF_SCOPE: library behaviour it drives; ext/misc extensions it embeds (own seeds)
