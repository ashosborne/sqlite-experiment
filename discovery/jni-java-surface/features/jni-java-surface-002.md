# jni-java-surface-002 — wrapper1 object-oriented Java layer

Slice: `jni-java-surface` · Status: `documented` · Confidence: `inferred` · Card written: 2026-08-11T10:34:29Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Sqlite/SqliteException OO wrapper with typed function classes.

## Entrypoints (citations)

- `org.sqlite.jni.wrapper1.Sqlite` (other) — `ext/jni/src/org/sqlite/jni/wrapper1/Sqlite.java`, `ext/jni/src/org/sqlite/jni/wrapper1/SqliteException.java`

## Inputs / outputs / observables

- org.sqlite.jni.wrapper1.Sqlite OO API; SqliteException; ScalarFunction/AggregateFunction/WindowFunction base classes

## Behaviour (as implemented)

- wrapper1 layers idiomatic Java (try-with-resources, exceptions) over capi; function base classes marshal UDF args/results

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- (none found in code)

## Dependencies

- jni-java-surface-001

## Assumptions / unknowns

- Confidence inferred; same consumer gate

## Evidence

- `ext/jni/src/org/sqlite/jni/wrapper1/Sqlite.java`
- `ext/jni/src/org/sqlite/jni/wrapper1/SqliteException.java`
