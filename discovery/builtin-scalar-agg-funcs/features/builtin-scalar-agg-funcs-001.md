# builtin-scalar-agg-funcs-001 — Core scalar function family

Slice: `builtin-scalar-agg-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

~70 scalar functions registered in aBuiltinFunc[]; NULL-propagation and UTF handling per function.

## Entrypoints (citations)

- `sqlite3RegisterBuiltinFunctions()` (other) — `src/func.c:3311`, `src/func.c:3322`, `src/func.c:3404`, `src/func.c:348`

## Inputs / outputs / observables

- SQL results of ~70 scalar functions; NULL propagation; UTF handling; SQLITE_TOOBIG on oversized results

## Behaviour (as implemented)

- Registry table aBuiltinFunc[] (src/func.c:3322) registered once globally via sqlite3RegisterBuiltinFunctions (src/func.c:3311); FuncDef flags control NULL short-circuit (most functions return NULL on any NULL arg), constantness, and encoding preference
- Families: string (length/substr/upper/lower/trim/replace/instr/hex/unhex/quote/char/unicode/concat...), numeric (abs/round/sign/random/randomblob), blob, control (coalesce/ifnull/iif/nullif/typeof/likely/unlikely)

## Validation rules found in code

- substr 1-based indexing with negative-start counting from end (src/func.c:348)
- round() banker's-adjacent formatting via printf engine

## Edge cases found in code

- length() on blob counts bytes, on text counts characters; on numeric arg stringifies first
- upper/lower are ASCII-only without ICU (see icu-001)
- random() returns signed 64-bit incl. INT64_MIN edge

## Dependencies

- utf-util

## Assumptions / unknowns

- Sub-clustering at Test-gen time (string/numeric/blob) per run-1 note — this card pins the registry seam
- Family needs sub-clustering at bind time (string vs numeric vs blob)

## Evidence

- `src/func.c:3311`
- `src/func.c:3322`
- `src/func.c:3404`
- `src/func.c:348`
