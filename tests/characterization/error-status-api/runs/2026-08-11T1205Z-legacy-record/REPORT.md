# RECORD run report — error-status-api (2026-08-11T1205Z-legacy-record)

Legacy RECORD on the pinned baseline (sqlite 3.54.0, bare-configure defaults; API_ARMOR off,
OMIT_AUTORESET off — full fingerprint in overnight/BASELINE.md).

| Case | Result | Golden |
| --- | --- | --- |
| error-status-api-001-C001 | RECORDED → immediate replay byte-match → **REPLAY_GREEN** | cases/error-status-api-001/C001.approved.txt |
| error-status-api-001-C002 | RECORDED → **REPLAY_GREEN** | cases/error-status-api-001/C002.approved.txt |

Observations vs the behaviour card: both match the card's shape (per-connection error state;
guarded NULL-db static answers). The errmsg English wording is frozen in the golden as captured but
is **not** a pass/fail contract — flagged for human review at golden approval.

Scrub: none needed (static inputs, integer rcs, fixed message text). Same-seed replay bit-matched.
**Golden approval: PENDING HUMAN** — do not let Conversion consume these until approved.
