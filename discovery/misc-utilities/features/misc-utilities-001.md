# misc-utilities-001 — Utility extension pack (~14)

Slice: `misc-utilities` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

fileio (fs read/write from SQL — security flag), dbdump, eval, explain, memstat, diskused, noop, randomjson, remember, stmtrand, strdup, showauth, anycollseq, normalize.

## Entrypoints (citations)

- `ext/misc utilities` (other) — `ext/misc/fileio.c:1266`, `ext/misc/dbdump.c:695`, `ext/misc/eval.c:119`, `ext/misc/memstat.c:422`

## Inputs / outputs / observables

- fileio: readfile()/writefile()/lsdir via fsdir() TVF; dbdump programmatic .dump; eval('sql'[,'sep']); explain vtab; memstat vtab; diskused; noop(); randomjson; remember(V,PTR); stmtrand; strdup-demo; showauth; anycollseq; sqlite3_normalize()

## Behaviour (as implemented)

- Census card over ~14 non-unbundled utilities (this cluster was NOT split in run 2 — stays one card per charter's 'leftover really is one optional pack' rule): file/system access (fileio ext/misc/fileio.c:1266 — writes files from SQL), SQL meta (eval ext/misc/eval.c:119 runs dynamic SQL, dbdump ext/misc/dbdump.c:695), diagnostics (memstat :422, explain :312, diskused :848, showauth :93), determinism helpers (stmtrand :82, randomjson :217), demos (noop :74, strdup :98, remember :62, anycollseq :49), normalize (SQL text canonicalizer)

## Validation rules found in code

- fileio functions are DIRECTONLY-style risk surfaces (fs writes from SQL); eval executes arbitrary SQL — both flagged

## Edge cases found in code

- dbdump standalone main behind DBDUMP_STANDALONE (CLI recorded in run-1 structural index)

## Dependencies

- loadext-api

## Assumptions / unknowns

- Security audit flag from run 1 stands (fileio/eval powers); per-utility split possible later if any is deployed downstream
- fileio/eval grant SQL-level fs+exec-ish powers — deployment audit needed

## Evidence

- `ext/misc/fileio.c:1266`
- `ext/misc/dbdump.c:695`
- `ext/misc/eval.c:119`
- `ext/misc/memstat.c:422`
