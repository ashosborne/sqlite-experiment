# misc-basexx-001 — base64 + base85 + combined basexx encoders (one optional pack)

Slice: `misc-basexx` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

base64()/base85() blob<->text conversions; basexx.c registers both — kept as the one genuine pack

## Entrypoints (citations)

- `sqlite3_basexx_init()` (other) — `ext/misc/basexx.c:70`, `ext/misc/base64.c:278`, `ext/misc/base85.c:355`

## Inputs / outputs / observables

- base64(X)/base64(TEXT) encode/decode pair semantics (blob→text, text→blob); base85 same shape; is_base85(T)

## Behaviour (as implemented)

- basexx (init ext/misc/basexx.c:70) registers both codecs (base64 ext/misc/base64.c:278, base85 ext/misc/base85.c:355): direction inferred from argument type (BLOB encodes, TEXT decodes)

## Validation rules found in code

- Invalid characters on decode → error (base85 stricter set)

## Edge cases found in code

- NULL passes through; empty blob → empty text

## Dependencies

- loadext-api

## Assumptions / unknowns

- Kept as the one genuine cluster (basexx registers both)

## Evidence

- `ext/misc/basexx.c:70`
- `ext/misc/base64.c:278`
- `ext/misc/base85.c:355`
