# OS Discovery MANIFEST schema (v0.1, draft)

Path: `os-discovery/<HOST_GROUP_ID>/MANIFEST.yaml`

```yaml
host_group_id: payments-prod-win2012
capture_as_of: "2026-08-03T12:00:00Z"
dump_sources:
  - path: dumps/cs-ps-export.csv
    kind: process-inventory
    parser: crowdstrike-ps-v1   # or unknown
hosts:
  - id: host-01
    hostname: pay-app-01
    os: Windows Server 2012 R2
    evidence: [...]
phase: A|B
runtime:
  - id: payments-svc-01
    name: PaymentsWorker.exe
    status: candidate|keep|replace|retire|defer|noise|documented
    confidence: observed-on-host|inferred|needs-SME
    binary_path: "D:\\Apps\\Payments\\Worker.exe"
    hosts: [host-01, host-02]
    listeners: ["0.0.0.0:8443"]
    evidence: [...]
    runtime_doc: runtime/payments-svc-01.md
    linkage_stubs:
      - guess: payments-api-repo
        confidence: inferred
        reason: "path contains Payments"
```

Rules:
- No `documented` without evidence cites into dump files
- CSV exports generated from this file only
- Linkage stubs never auto-promote to confirmed links
