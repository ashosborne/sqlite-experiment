# geopoly-001 — Geopoly vtab and polygon functions

Slice: `geopoly` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

geopoly vtab over rtree; geopoly_overlap/within/area/json etc.; overloaded-function query planning.

## Entrypoints (citations)

- `sqlite3_geopoly_init()` (other) — `ext/rtree/geopoly.c:1795`, `ext/rtree/geopoly.c:1237`, `ext/rtree/geopoly.c:1180`, `ext/rtree/geopoly.c:1753`

## Inputs / outputs / observables

- geopoly vtab (USING geopoly(...)); geopoly_* functions (overlap/within/area/blob/json/svg/bbox/contains_point/xform/regular/group_bbox)

## Behaviour (as implemented)

- sqlite3_geopoly_init (ext/rtree/geopoly.c:1795) piggybacks on rtree: polygons stored as GeoJSON-ish arrays or binary blob; vtab (geopolyInit :1237) indexes bounding boxes in an rtree, query planning routes geopoly_overlap/within (:1180,1495,1753) through R-tree filtering then exact polygon test

## Validation rules found in code

- Malformed polygons → NULL results (not errors) in most functions

## Edge cases found in code

- Winding-order normalization; holes NOT supported (simple polygons only)

## Dependencies

- rtree

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/rtree/geopoly.c:1795`
- `ext/rtree/geopoly.c:1237`
- `ext/rtree/geopoly.c:1180`
- `ext/rtree/geopoly.c:1753`
