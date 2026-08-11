# jni-binding-001 — JNI adapter layer (C-to-Java bridge)

Slice: `jni-binding` · Status: `needs-SME` · Confidence: `inferred` · Card written: 2026-08-11T10:32:22Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

> **NEEDS-SME**: needs-SME: inferred-confidence card requires SME sign-off or waive-characterization before Test gen

## Summary

1:1-ish JNI bridges over the C API with Java callback (UDF/hook) marshalling and thread rules.

## Entrypoints (citations)

- `ext/jni/src/c/sqlite3-jni.c` (other) — `ext/jni/src/c/sqlite3-jni.c`, `ext/jni/src/c/sqlite3-jni.h`

## Inputs / outputs / observables

- Java-visible native methods backing the capi layer; callback marshalling for hooks/UDFs/authorizer; JNI local-ref discipline

## Behaviour (as implemented)

- ext/jni/src/c/sqlite3-jni.c defines the JNI bridges via macro families (JniFuncName pattern ext/jni/src/c/sqlite3-jni.c:179, :5443): 1:1-ish wrappers over the C API translating jstring/jbyteArray ↔ UTF-8/blobs, pinning Java callback objects for sqlite3-side hooks, per-env cache of method IDs
- Generated header sqlite3-jni.h carries the native declarations consumed by the Java capi classes

## Validation rules found in code

- Thread rules: JNIEnv affinity handled via env cache (asserted in code comments)

## Edge cases found in code

- UDF exceptions in Java surface as SQL errors; OOM paths return JNI-safe errors

## Dependencies

- connection-lifecycle-api
- prepare-statement-api

## Assumptions / unknowns

- Confidence inferred: bridge internals not traced line-by-line this run (file+macro evidence); consumer-existence SME question stands
- Is the Java binding a consumed product surface downstream?

## Evidence

- `ext/jni/src/c/sqlite3-jni.c`
- `ext/jni/src/c/sqlite3-jni.h`
