# rtree-002 — Custom geometry/query callback API (MATCH)

Slice: `rtree` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

geometry_callback (legacy) and query_callback (scored traversal) create MATCH-usable SQL functions.

## Entrypoints (citations)

- `sqlite3_rtree_query_callback()` (api) — `ext/rtree/rtree.c:4457`, `ext/rtree/rtree.c:4433`, `ext/rtree/rtree.c:361`

## Inputs / outputs / observables

- MATCH with custom geometry functions; scored traversal via query callbacks (visibility/score per node)

## Behaviour (as implemented)

- sqlite3_rtree_geometry_callback (ext/rtree/rtree.c:4433, legacy) and query_callback (:4457, scored NN-capable) create SQL functions usable as 'col MATCH fn(args)'; callbacks receive bounding boxes and decide inclusion/priority (RTREE_MATCH/QUERY plumbing :361)

## Validation rules found in code

- MATCH function must have been registered on the same connection

## Edge cases found in code

- Query callbacks enable k-nearest-neighbour by score ordering

## Dependencies

- rtree-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/rtree/rtree.c:4457`
- `ext/rtree/rtree.c:4433`
- `ext/rtree/rtree.c:361`
