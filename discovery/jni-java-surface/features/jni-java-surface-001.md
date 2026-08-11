# jni-java-surface-001 — capi 1:1 Java layer with callback interfaces

Slice: `jni-java-surface` · Status: `needs-SME` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

> **NEEDS-SME**: needs-SME: inferred-confidence card requires SME sign-off or waive-characterization before Test gen

## Summary

CApi.java static methods mirror C API; ~40 callback interfaces marshal hooks/UDFs.

## Entrypoints (citations)

- `org.sqlite.jni.capi.CApi` (other) — `ext/jni/src/org/sqlite/jni/capi/CApi.java`, `ext/jni/src/org/sqlite/jni/capi/AuthorizerCallback.java`, `ext/jni/src/org/sqlite/jni/capi/BusyHandlerCallback.java`

## Inputs / outputs / observables

- org.sqlite.jni.capi.CApi static methods (sqlite3_open_v2, prepare, step...); callback interfaces (~40: AuthorizerCallback, BusyHandlerCallback, CommitHookCallback, ConfigLogCallback...)

## Behaviour (as implemented)

- CApi.java exposes 1:1 static natives backed by the C bridge; callback interfaces are Java objects pinned by the bridge and invoked from sqlite3 hooks with JNI marshalling

## Validation rules found in code

- Java-side argument checks mirror C MISUSE contracts (per class docs)

## Edge cases found in code

- Output-pointer emulation via holder classes (e.g. OutputPointer types in the package)

## Dependencies

- jni-binding

## Assumptions / unknowns

- Confidence inferred: Java tree read at directory/class-name level; consumer question stands

## Evidence

- `ext/jni/src/org/sqlite/jni/capi/CApi.java`
- `ext/jni/src/org/sqlite/jni/capi/AuthorizerCallback.java`
- `ext/jni/src/org/sqlite/jni/capi/BusyHandlerCallback.java`
