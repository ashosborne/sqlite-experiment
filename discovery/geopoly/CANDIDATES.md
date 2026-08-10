# CANDIDATES — geopoly (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | geopoly vtab + polygon function family | loadable init `ext/rtree/geopoly.c:1795`, vtab init `geopolyInit` `:1237`, `geopoly_overlap` `:1180` with rtree query-plan integration `:1495,1753` | GeoJSON polygon storage/query built over rtree |
