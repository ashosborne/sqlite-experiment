# error-status-api-001 — Error introspection family

Slice: `error-status-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

errcode/extended_errcode/errmsg/errstr/error_offset; UTF-8/16 variants; OOM fallbacks.

## Entrypoints (citations)

- `sqlite3_errmsg()` (api) — `src/main.c:2743`, `src/main.c:2851`, `src/main.c:2866`, `src/main.c:2896`, `src/main.c:2792`

## Inputs / outputs / observables

- errcode/extended_errcode/errmsg(16)/errstr/error_offset values after any failed API call

## Behaviour (as implemented)

- errmsg (src/main.c:2743) returns UTF-8 text for the most recent failure on the connection; falls back to static strings on OOM
- extended_errcode (src/main.c:2866) exposes the full extended code; errstr (src/main.c:2896) is a static table lookup usable without a db
- error_offset (src/main.c:2792) gives the byte offset into the SQL of the most recent parse/prepare error, or -1

## Validation rules found in code

- Calling on NULL db returns errcode=7 (SQLITE_NOMEM) and errmsg 'out of memory' (static guarded answers via sqlite3ErrStr). This is not MISUSE. [Patched run 6 to match the recorded pin — was: "'out of memory'/MISUSE-safe static answers (guarded)"]

## Edge cases found in code

- Error state is per-connection and overwritten by any subsequent API call, including successful ones

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/main.c:2743`
- `src/main.c:2851`
- `src/main.c:2866`
- `src/main.c:2896`
- `src/main.c:2792`

## Discovery note (run 6)

- Pinned by legacy RECORD run `2026-08-11T1205Z-legacy-record` on the baseline fingerprint in `overnight/BASELINE.md` (sqlite 3.54.0; API_ARMOR=0). Goldens human-accepted 2026-08-11 (Ash Osborne). errmsg English wording is `wording_deferred` (shape-only for future COMPARE); the NULL-handle 'out of memory' string is a contract (sqlite3ErrStr/SQLITE_NOMEM).
