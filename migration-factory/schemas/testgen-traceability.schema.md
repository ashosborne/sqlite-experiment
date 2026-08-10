# Test generation TRACEABILITY schema (v0.1, draft)

Path: `testgen/<SLICE_ID>/TRACEABILITY.yaml`

```yaml
slice_id: account-opening
source_discovery: discovery/account-opening/MANIFEST.yaml
phase: A|B
updated_at: ISO-8601
batch_card_ids: [account-opening-001, account-opening-002]

cases:
  - id: account-opening-001-C001
    feature_id: account-opening-001
    title: "Open account — primary path as implemented"
    status: proposed|approved|dropped|deferred|specified|blocked
    kind: characterization|contract
    evidence_gate: observed-in-code|waived-inferred|blocked
    discovery_evidence: [path:symbol or card anchors]
    boundary: "POST /api/accounts/open"
    spec_path: scenarios/account-opening-001/CASE-001.md
    harness_path: harness/account_opening_api.py   # optional until Phase B
    assert_mode: TO_BE_RECORDED
    observables: ["http.status", "body.accountId", "db.accounts.row"]
    scrub: ["PII.name", "tokens"]
    deferred_reasons: []    # or ["perf", "security-abuse", "cross-slice"]

deferred:
  - feature_id: account-opening-003
    reason: "needs-SME auth matrix; no observable on card"
```

Rules:
- No `specified` without `feature_id` + discovery evidence link
- `assert_mode` default `TO_BE_RECORDED` until Test execution freezes goldens
- CSV exports generated from this file only
