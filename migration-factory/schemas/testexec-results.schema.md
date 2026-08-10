# Test execution results + TRACEABILITY statuses (v0.1, draft)

## TRACEABILITY status enum (per case)

```text
TO_BE_RECORDED → RECORDING → RECORDED → REPLAY_GREEN
                      ↘ BLOCKED   (state: cannot execute / waiting — not a failure class)

Failure classes (on a completed-but-failed attempt):
  HARNESS_FAIL | FLAKE | DISCOVERY_GAP | BEHAVIOURAL_DELTA
```

COMPARE / modern parity statuses belong to Verification (e.g. `PARITY_GREEN` | `PARITY_FAIL`) — not this stage by default.

## `runs/<RUN_ID>/results.json` (sketch)

```yaml
run_id: 2026-08-04T1100Z-legacy-record
slice_id: account-opening
mode: RECORD|REPLAY
target: legacy
cases:
  - case_id: account-opening-001-C001
    feature_id: account-opening-001
    status: REPLAY_GREEN|BLOCKED|FAILED
    failure_class: null|HARNESS_FAIL|FLAKE|DISCOVERY_GAP|BEHAVIOURAL_DELTA
    golden_path: cases/account-opening-001/C001.approved.json
    actual_path: runs/.../actuals/C001.json
    failure:
      failure_id: optional
      repro: []
      expected: null | golden-ref
      actual_summary: string
      class: HARNESS_FAIL|FLAKE|DISCOVERY_GAP|BEHAVIOURAL_DELTA
      hypothesis: string
```

Rules:
- RECORD must not invent `expected` before golden write
- CSV/HTML exports from `results.json` only
- Golden rebase requires human approve flag in run metadata
- `BLOCKED` is a status/state, not a failure_class
- `BEHAVIOURAL_DELTA` → escalate Discovery; do not rewrite goldens
