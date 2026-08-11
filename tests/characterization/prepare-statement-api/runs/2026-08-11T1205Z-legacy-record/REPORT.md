# RECORD run report — prepare-statement-api (2026-08-11T1205Z-legacy-record)

Legacy RECORD on the pinned baseline (sqlite 3.54.0; API_ARMOR off, OMIT_AUTORESET off).

| Case | Result | Golden |
| --- | --- | --- |
| prepare-statement-api-002-C001 | RECORDED → **REPLAY_GREEN** (prepare=0, ROW=100, col=1, DONE=101) | cases/prepare-statement-api-002/C001.approved.txt |
| prepare-statement-api-002-C002 | RECORDED → **REPLAY_GREEN** (step-after-DONE = **100 ROW**, autoreset) | cases/prepare-statement-api-002/C002.approved.txt |
| prepare-statement-api-002-C003 | **BLOCKED** — unsafe capture (UAF); harness call removed; nothing frozen | — |

## Notable finding (for Discovery, not a golden rewrite)

C002: the third `sqlite3_step` after DONE returned **SQLITE_ROW (100)**, not MISUSE — this build
auto-resets completed statements (`OMIT_AUTORESET` off, confirmed in the fingerprint). The operator
charter predicted this. The behaviour card's sentence "DONE means the statement completed and must
be reset before re-stepping" reflects the documented/manual contract, not the autoreset
implementation — **suggest a card refinement note on prepare-statement-api-002** at next Discovery
touch. Golden pins the actual (ROW).

Scrub: none needed. Same-seed replay bit-matched all four goldens.
**Golden approval: PENDING HUMAN** — Conversion must not consume before approval.
