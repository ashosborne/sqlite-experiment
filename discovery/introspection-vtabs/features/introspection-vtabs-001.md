# introspection-vtabs-001 — Introspection vtab trio (dbstat / sqlite_dbpage / bytecode)

Slice: `introspection-vtabs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:30:12Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Compile-gated eponymous vtabs exposing page stats, raw pages (writable!), and VDBE programs.

## Entrypoints (citations)

- `sqlite3DbstatRegister()` (other) — `src/dbstat.c:874`, `src/dbpage.c:473`, `src/vdbevtab.c:436`

## Inputs / outputs / observables

- dbstat rows (name,path,pageno,pagetype,ncell,payload,unused,mx_payload); sqlite_dbpage SELECT/UPDATE of raw pages; bytecode()/tables_used() TVFs

## Behaviour (as implemented)

- Compile-gated trio: sqlite3DbstatRegister (src/dbstat.c:874, gate SQLITE_ENABLE_DBSTAT_VTAB), DbpageRegister (src/dbpage.c:473 — WRITABLE: UPDATE writes raw pages), VdbeBytecodeVtabInit (src/vdbevtab.c:436, gate SQLITE_ENABLE_BYTECODE_VTAB); no-op stubs when gated off

## Validation rules found in code

- dbpage writes require defensive mode OFF

## Edge cases found in code

- dbpage write path can corrupt the db by design (repair tool) — security/scope question from run 1 stands

## Dependencies

- vtab-core

## Assumptions / unknowns

- Default baseline: none of the three gates on — cards document gated contracts
- sqlite_dbpage write path is dangerous — is it enabled downstream?

## Evidence

- `src/dbstat.c:874`
- `src/dbpage.c:473`
- `src/vdbevtab.c:436`
