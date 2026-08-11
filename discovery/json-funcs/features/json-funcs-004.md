# json-funcs-004 — json_each / json_tree table-valued functions

Slice: `json-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Eponymous vtabs enumerate elements/paths; hidden columns key/value/type/path.

## Entrypoints (citations)

- `jsonEachModule` (other) — `src/json.c:5627`

## Inputs / outputs / observables

- json_each row set (key, value, type, atom, id, parent, fullkey, path); json_tree recursive walk incl. container rows

## Behaviour (as implemented)

- jsonEachModule (src/json.c:5627) eponymous TVFs; second arg = starting path; hidden columns expose the traversal state

## Validation rules found in code

- Malformed JSON errors at cursor open, not at prepare

## Edge cases found in code

- value column returns SQL values for scalars, JSON text for containers (same unwrap rule as extract)

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/json.c:5627`
