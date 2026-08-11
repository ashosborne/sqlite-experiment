# json-funcs-002 — JSON mutation family (set/insert/replace/patch/remove)

Slice: `json-funcs` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Create-or-update semantics per variant; jsonb_* return binary JSONB.

## Entrypoints (citations)

- `jsonSetFunc()` (other) — `src/json.c:4548`, `src/json.c:4389`, `src/json.c:5695`, `src/json.c:5696`

## Inputs / outputs / observables

- Mutation results as JSON text (json_*) or JSONB blob (jsonb_*); create-vs-overwrite matrix: set=upsert, insert=only-if-absent, replace=only-if-present; patch=RFC-7396 merge

## Behaviour (as implemented)

- jsonSetFunc (src/json.c:4548) handles set/insert/replace via flag bits (registry rows src/json.c:5673-5696); jsonPatchFunc (src/json.c:4389) implements MergePatch (null deletes keys); json_remove deletes paths

## Validation rules found in code

- Path must start with $; appending past array end with insert creates trailing element only at exact next index

## Edge cases found in code

- Setting into a scalar's child path silently no-ops per path-not-found rules (documented asymmetries between variants)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- JSONB byte-format stability question from run 1 stands (interchange caveat)
- JSONB byte-format stability guarantees for downstream storage?

## Evidence

- `src/json.c:4548`
- `src/json.c:4389`
- `src/json.c:5695`
- `src/json.c:5696`
