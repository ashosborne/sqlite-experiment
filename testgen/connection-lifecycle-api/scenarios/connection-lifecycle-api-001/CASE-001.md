# connection-lifecycle-api-001-C001 — open ':memory:'

Feature: `connection-lifecycle-api-001` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: `src/main.c:3742` (sqlite3_open), `src/main.c:3380` (openDatabase)

## Preconditions / fixtures
- Pinned baseline (overnight/BASELINE.md — 3.54.0, run-5 fingerprint is law). Fresh process.

## Boundary invoke
1. `rc = sqlite3_open(":memory:", &db)`

## Observables to capture
- `open.rc`, `db.nonnull` (1/0)

## Scrub
- None (no filename on disk, no PII).
