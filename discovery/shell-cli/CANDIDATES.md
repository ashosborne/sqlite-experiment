# CANDIDATES — shell-cli (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind. Note: shell.c.in is preprocessed by tool/mkshellc.tcl (out-of-scope path) — evidence cites the source template only.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | CLI entry + SQL input loop | `main` `src/shell.c.in:13784`, `process_input` `:13237`, `runOneSqlLine` `:13121` | Batch/interactive entrypoint (closest analogue to a batch main) |
| 002 | Dot-command surface (.mode, .import, .dump, ...) | `do_meta_command` `src/shell.c.in:9773`, dot-cmd parsing `:459` | Large operator-facing command language |
