# Seed — auth-callback-api
SLICE_ID: auth-callback-api
SLICE_SEED: "compile-time authorizer callback (sqlite3_set_authorizer)"
SEED_ENTRYPOINTS: sqlite3_set_authorizer()
OUT_OF_SCOPE: parser call sites of auth checks (compiler seeds)
Rationale: small policy-injection seam invoked during statement compilation; DENY/IGNORE/OK semantics.
