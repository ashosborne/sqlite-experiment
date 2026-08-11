# Seed — jni-java-surface (resume run 2)
SLICE_ID: jni-java-surface
SLICE_SEED: "Java-side API tree: capi 1:1 layer, wrapper1 OO layer, fts5 bindings"
SEED_ENTRYPOINTS: ext/jni/src/org/sqlite/jni/capi/CApi.java, wrapper1/Sqlite.java
OUT_OF_SCOPE: C bridge (run-1 jni-binding umbrella), Java build tooling
Rationale: run-1 residual 6 — Java class tree not scanned (C bridge only).
