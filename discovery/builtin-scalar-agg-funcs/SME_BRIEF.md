# SME brief — builtin-scalar-agg-funcs (Phase A)
**Found:** 3 clustered candidates over ~111 registry rows. Deliberately NOT one candidate per function — the registry table is the honest index; bind-time sub-clustering recommended (string/numeric/blob scalars).
**Recommended binds:** accept all 3; 003 first if LIKE-driven query plans matter downstream.
STOPPED for human bind. No Phase B performed.
