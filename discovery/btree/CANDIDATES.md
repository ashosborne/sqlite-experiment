# CANDIDATES — btree (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Btree handle + transaction boundary | `sqlite3BtreeOpen` `src/btree.c:2562`, `sqlite3BtreeBeginTrans` `:3835` | Storage-layer callable seam |
| 002 | Cursor operations (seek/insert/delete) | `sqlite3BtreeCursor` `src/btree.c:4799`, `TableMoveto` `:5837`, `Insert` `:9441`, `Delete` `:9873` | Row access/mutation contract incl. balancing side effects |
