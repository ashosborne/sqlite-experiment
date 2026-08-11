# Seed — compile-options-omit-enable (resume run 2)
SLICE_ID: compile-options-omit-enable
SLICE_SEED: "compile-time option matrix: SQLITE_OMIT_*/SQLITE_ENABLE_* gated surface visibility"
SEED_ENTRYPOINTS: sqlite3_compileoption_used(), sqlite3_compileoption_get(), SQLITE_OMIT_*/SQLITE_ENABLE_* guards
OUT_OF_SCOPE: behaviour of the gated features themselves (own slices); tool/-generated option lists
Rationale: run-1 METHOD_COVERAGE residual 1. The option matrix decides which surfaces exist per build — a migration baseline decision, discoverable via the diagnostics API + guard census.
