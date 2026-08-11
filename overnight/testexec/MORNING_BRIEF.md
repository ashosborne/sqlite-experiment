# MORNING BRIEF — sqlite-experiment run 10: RECORD connection-lifecycle + exec-convenience

Run: 2026-08-11 · `2026-08-11T1500Z-legacy-record-open-exec` · from `d763513` · committed as `sqlite-record-open-exec`
MODE=RECORD · TARGET=legacy · same branch, no PR. (Run-8 brief preserved as `MORNING_BRIEF-2026-08-11-run8.md`;
run-9 convert brief unchanged at `overnight/convert/MORNING_BRIEF.md`.)

## 1. Five goldens + replay + pin

Run-5 build reused; fingerprint re-verified live: **3.54.0, ENABLE_API_ARMOR=0, OMIT_AUTORESET=0** —
matches BASELINE (not rewritten); zero `src/`/`ext/` drift.

| Case | RECORD actuals | Replay |
| --- | --- | --- |
| connection-lifecycle-api-001-C001 | open.rc=0, db.nonnull=1 | **REPLAY_GREEN** |
| connection-lifecycle-api-001-C002 | close.rc=0 | **REPLAY_GREEN** |
| exec-convenience-api-001-C001 | exec.rc=0, cb.calls=1, cb.argc=1, cb.argv0=`1` | **REPLAY_GREEN** |
| exec-convenience-api-001-C002 | exec.rc=0 (NULL callback, side-effect run) | **REPLAY_GREEN** |
| exec-convenience-api-001-C003 | callback returned 1 → **exec.rc=4 (SQLITE_ABORT)**, cb.calls=1 | **REPLAY_GREEN** |

Goldens under `tests/characterization/{connection-lifecycle-api,exec-convenience-api}/cases/`;
actuals carry **full case ids** (run-7 collision lesson); `results.json` actual_paths unique per case.
`golden_approval: PENDING_HUMAN` — **no stamp this run**.

## 2. Not recorded (per DO_NOT_RECORD)

URI mode=/vfs=/cache= matrix (parked in DEFERRED — the pin's fingerprint lists no USE_URI),
empty-filename temp open, open16/UTF-16, illegal open_v2 flag MISUSE (needs flag-matrix SME),
and prepare-statement-api-002-C003 (still BLOCKED; no step(NULL)).

## 3. Untouched surfaces

`modern/` and the BOUND pack: zero diffs. The ten stamped goldens: md5-verified byte-identical.
No `src/`/`ext/`/`test/` edits.

## 4. Flags

`legacy_green` still **5** (unchanged — none set on connection/exec this run). `parity_green` still **0**.
APP_MANIFEST got traceability pointers only; schema VALID.

## 5. completeness: incomplete

Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes). 15 goldens now
exist: 10 stamped + 5 pending.

## 6. Closing ask

**Stamp these five, then pack v2 (SUPERSEDE v1 + human re-bind) + convert the new symbols
(`sqlite3_exec` + callback marshalling, open/close already in the crate) on this same branch?**
