# builtin-scalar-agg-funcs-003 — LIKE/GLOB pattern matching + case sensitivity toggle

Slice: `builtin-scalar-agg-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

like/glob implemented as functions; case_sensitive_like re-registers; optimizer queries IsLikeFunction for index use.

## Entrypoints (citations)

- `likeFunc()` (other) — `src/func.c:921`, `src/func.c:2348`, `src/func.c:2361`, `src/func.c:2388`

## Inputs / outputs / observables

- LIKE/GLOB match results; case_sensitive_like pragma effect; ESCAPE clause handling; index usage visible in query plans

## Behaviour (as implemented)

- likeFunc (src/func.c:921) implements LIKE (case-insensitive ASCII default, %/_ wildcards) and GLOB (case-sensitive, */?/[] sets) via shared pattern engine with distinct wildcard structs
- sqlite3RegisterLikeFunctions (src/func.c:2348) re-registers on PRAGMA case_sensitive_like; sqlite3IsLikeFunction (src/func.c:2388) lets the planner rewrite LIKE 'prefix%' into index range scans

## Validation rules found in code

- LIKE pattern length capped by SQLITE_LIMIT_LIKE_PATTERN_LENGTH
- ESCAPE must be a single character → error otherwise

## Edge cases found in code

- LIKE with numeric operands stringifies per affinity rules
- Unicode case folding NOT applied (ASCII only) — parity trap vs other engines

## Dependencies

- where-optimizer

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/func.c:921`
- `src/func.c:2348`
- `src/func.c:2361`
- `src/func.c:2388`
