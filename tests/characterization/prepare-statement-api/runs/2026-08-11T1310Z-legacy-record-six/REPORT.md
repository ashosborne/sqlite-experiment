# RECORD run report — prepare-statement-api features 001/003/005 (2026-08-11T1310Z-legacy-record-six)

Legacy RECORD on the SAME pin as run 5 (fingerprint re-verified: 3.54.0, API_ARMOR=0, OMIT_AUTORESET=0;
zero src/ext diffs since the run-5 build base — no rebuild, no second pin).

| Case | Result | Key actuals |
| --- | --- | --- |
| 001-C001 | **REPLAY_GREEN** | prepare=0, stmt non-NULL, pzTail consumed |
| 001-C002 | **REPLAY_GREEN** | whitespace/comment SQL → OK + NULL stmt (card shape confirmed) |
| 003-C001 | **REPLAY_GREEN** | bind=0, ROW, column=7 |
| 003-C002 | **REPLAY_GREEN** | bind index OOR → 25 (SQLITE_RANGE) |
| 005-C001 | **REPLAY_GREEN** | explicit reset-from-DONE: reset=0, re-step ROW, bound 42 preserved |
| 005-C002 | **REPLAY_GREEN** | finalize on live stmt → 0 (contract = finalize.rc only) |

All six replays byte-matched their fresh goldens. The four HUMAN_ACCEPTED goldens verified
byte-identical (md5). C003 remains BLOCKED (UAF) — not captured, no sqlite3_step(NULL) invented.

**legacy_green NOT set on 001/003/005** — the stamp is a later human look.
**Golden approval for these six: PENDING HUMAN.**
