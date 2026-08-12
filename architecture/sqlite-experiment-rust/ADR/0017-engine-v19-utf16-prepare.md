# ADR 0017 — engine v19: UTF-16 prepare family + UTF-16 column accessors

Status: accepted (pack v19 BOUND, Ash Osborne, delegated autonomy run 29)

## Context

`prepare-statement-api-001` was the cleanest remaining named partial on the prepare
family: UTF-8 `prepare_v2` / `prepare_v3` were real since v13, but the UTF-16 prepare
variants were the sole gap. The matching UTF-16 column surfaces (`column_text16`,
`column_bytes16`, `column_name16`, `column_decltype16`) were missing, so landing only
prepare16 would leave the public UTF-16 text API half-done. `util-primitives-001`
listed UTF codecs as absent.

## Decision

1. **Shared prepare core, real codec.** `sqlite3_prepare16[_v2|_v3]` decode the caller's
   UTF-16LE buffer (nByte in BYTES, negative = to NUL; surrogate pairs handled by the
   std `from_utf16` path) into UTF-8 and run the *same* prepare core as the UTF-8 twins.
   No parallel compile path; no canned UTF-16 pins.
2. **pzTail stays in the caller's buffer.** The consumed UTF-8 prefix length is mapped
   back to UTF-16 code units and the tail pointer is offset into the original UTF-16
   buffer — pinned by the two-statement C case (tail = `" SELECT 2"`, second prepare16
   of the tail runs).
3. **Endian/BOM decision:** native-endian (little-endian on the pinned x86-64 baseline)
   UTF-16 without BOM is what the harness feeds and what the goldens pin. A BOM case was
   drafted but not frozen: BOM handling is endianness-dependent surface we do not claim.
4. **column16 accessors reuse engine values.** text16/bytes16/name16/decltype16 convert
   the same rendered value / colname / decltype the UTF-8 accessors expose;
   `bytes16 = 2 × code units` (emoji = 2 units = 4 bytes, pinned). SQL NULL,
   before-step, after-done and out-of-range return NULL/0 per C.
5. **decltype is now tracked.** The store parses declared column types from
   CREATE TABLE (`stmt_decltypes`); bare-column SELECT items resolve to that type,
   expression columns to NULL — this also lands UTF-8 `sqlite3_column_decltype`.
6. **bind_text16** decodes and copies, mirroring bind_text.

## Fixes forced by real UTF-16 input

- `eval::find_kw_top` sliced a `String` at byte offsets and panicked on multi-byte SQL
  (`SELECT 'café 😀'`); it now scans bytes.
- `SELECTT 1` was mis-dispatched as SELECT+junk; statement keywords are now
  word-boundary matched, so the pinned `near "SELECTT": syntax error` shape holds.

## Out of scope (unchanged)

`sqlite3_create_function16`, window UDFs, `dlopen`/load_extension, WAL, cost-based
planner, UTF-16 errmsg surface (`sqlite3_errmsg16`), embedded-NUL text policy.

## Consequences

18 new goldens (engine-utf16-001 ×10, engine-utf16-002 ×8) replay byte-identical;
2 anti-cheat tests prove runtime codec + prepare16. `prepare-statement-api-001` can
honestly flip to full; `util-primitives-001` stays partial (hash/PRNG residuals).
