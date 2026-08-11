# MORNING BRIEF — sqlite-experiment run 8: stamp six + run-folder tidy

Run: 2026-08-11 · from `f48f47f4a` · committed as `sqlite-stamp-six` · STAGE: stamp (no RECORD, no COMPARE, no PACK, no Conversion)
(Run-7 brief preserved as `MORNING_BRIEF-2026-08-11-run7.md`.)

## 1. Six IDs HUMAN_ACCEPTED

`golden_approval` in both `testgen/prepare-statement-api/TRACEABILITY.yaml` and
`tests/characterization/prepare-statement-api/TRACEABILITY.yaml` now reads HUMAN_ACCEPTED
(Ash Osborne, 2026-08-11, Europe/London) with `stamped_case_ids` = the 002 pair **plus** the six
run-7 cases (run-6 stamp extended, not overwritten; history noted). `pending_golden_approval`
cleared with a note. All eight cases stay REPLAY_GREEN. All ten goldens verified **byte-identical**
(md5 before/after). No renames.

## 2. Run-folder collision fixed

`runs/2026-08-11T1310Z-legacy-record-six/actuals/` previously aliased six cases onto two files
(`C001.replay.txt` held 005-C001's lines; results.json pointed three cases at it). Fixed by
**splitting `logs/record_raw.txt`** — no harness re-run:
- six full-id files written (`prepare-statement-api-00X-C00n.replay.txt`), each containing only that case's OBS lines
- colliding `C001.replay.txt`/`C002.replay.txt` deleted
- every `results.json` `actual_path` re-pointed at the matching full-id file (REPORT.md had no stale links)
- byte-check: each new actual matches its `cases/<FEATURE_ID>/C00n.approved.txt` golden exactly

## 3. Glance lines match the pin

- prepare-statement-api-002 card Summary (and the observables line): "SQLITE_ROW/DONE/BUSY/MISUSE contract"
  → "return codes as implemented on this pin: ROW/DONE (autoreset after DONE); BUSY only where evidenced.
  Finalized-handle is UAF / not observed — MISUSE is not a recorded contract."
- APP_MANIFEST prepare-statement-api-002 notes: same tidy; "not conversion-ready while C003 BLOCKED" kept.
- COVERAGE.md regenerated from the manifest.

## 4. legacy_green now 5

`error-status-api-001`, `prepare-statement-api-001/002/003/005` — all with human-stamped goldens.
Other 180 behaviours stay `legacy_green: false`. Every `parity_green` stays false.

## 5. C003 still BLOCKED

UAF; `golden_path: null`; no `sqlite3_step(NULL)` invented under C003 or C004.

## 6. What this run did NOT do

No RECORD, no new `*.approved.txt`, no harness execution, no COMPARE, no Conversion, no PACK.yaml,
no Discovery seeding, no product edits.

## 7. completeness: incomplete

Estate residuals unchanged (3 hints, 10 needs-SME cards, METHOD_COVERAGE holes). 5 of 185 behaviours
legacy-green; nothing parity-green; Conversion still refused (no BOUND pack exists).

## 8. Closing ask

Two doors, pick one (or neither):
1. **More RECORD** — next spine batch: `connection-lifecycle-api-001` / `exec-convenience-api-001`
   (cards documented + observed-in-code; harness pattern proven; same pin).
2. **Land the DRAFT pack** — author `architecture/<MIGRATION_ID>/PACK.yaml` as `status: DRAFT` on
   this branch (still **not** BIND — human architect binds in git per the Architecture skill).
