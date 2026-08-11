# jni-java-surface-003 — FTS5 Java extension bindings

Slice: `jni-java-surface` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Java projections of fts5_api / Fts5ExtensionApi for aux functions.

## Entrypoints (citations)

- `org.sqlite.jni.fts5` (other) — `ext/jni/src/org/sqlite/jni/fts5/fts5_api.java`, `ext/jni/src/org/sqlite/jni/fts5/Fts5ExtensionApi.java`

## Inputs / outputs / observables

- org.sqlite.jni.fts5 classes (fts5_api, Fts5ExtensionApi, Fts5Context, fts5_extension_function)

## Behaviour (as implemented)

- Java projections of the FTS5 extension API allowing Java-implemented aux functions over fts5 tables

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Only built when JNI build enables FTS5 (build-gate)

## Dependencies

- fts5

## Assumptions / unknowns

- Confidence inferred; depends on fts5 slice contracts
- Same consumer question as run 1: does anything downstream use the Java binding?

## Evidence

- `ext/jni/src/org/sqlite/jni/fts5/fts5_api.java`
- `ext/jni/src/org/sqlite/jni/fts5/Fts5ExtensionApi.java`
