# CANDIDATES — vdbe-engine (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Opcode interpreter loop (199 opcodes) | `sqlite3VdbeExec` `src/vdbe.c:902`; e.g. `OP_Halt` `:1349`, `OP_Column` `:3031`; opcode list extracted by tool/mkopcodeh.tcl (generated headers not cited) | The execution engine seam |
| 002 | Value/memory-cell semantics (type affinity, text encoding) | `src/vdbemem.c` (Mem representation + conversions) | Where SQLite's dynamic typing actually lives |
