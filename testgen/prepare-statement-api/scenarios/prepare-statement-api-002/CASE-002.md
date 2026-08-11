# prepare-statement-api-002-C002 — step after DONE without reset

Feature: `prepare-statement-api-002` · Kind: characterization · Assert mode: **TO_BE_RECORDED**
Evidence gate: observed-in-code · Citations: card Behaviour section ("DONE means the statement
completed and must be reset before re-stepping"), `src/vdbeapi.c:980`

## Preconditions / fixtures
- Continuation of C001 state: statement has returned DONE; **no** sqlite3_reset() issued.

## Boundary invoke
1. `rc3 = sqlite3_step(stmt)` (third step, post-DONE, no reset)

## Observables to capture
- `step3_after_done.rc` — the as-implemented code (the operator matrix names MISUSE as the
  card-derived shape; RECORD pins whatever the pinned build returns — no invented expects).

## Scrub
- None.

## Notes
- Capture immediately after C001's DONE on the same statement to keep the state machine exact.
