# Discovery MANIFEST schema (v0.2, draft)

Path: `discovery/<SLICE_ID>/MANIFEST.yaml`

```yaml
slice_id: account-opening
seed: "account opening"
seed_entrypoints: []
out_of_scope_hints: []
phase: A|B
updated_at: ISO-8601

features:
  - id: account-opening-001          # required after bind; provisional_id ok in phase A
    name: string
    status: candidate|accepted|rejected|deferred|documented|blocked|needs-SME
    confidence: observed-in-code|inferred|needs-SME
    summary: string
    entrypoints:
      - kind: route|api|screen|job|other
        locator: string              # path, symbol, URL, etc.
        evidence: [file:line or symbol]
    behaviour_doc: features/account-opening-001.md   # phase B
    dependencies: [string]
    open_questions: [string]
```

Validation rules (human / CI):
- Phase B: every `accepted`/`documented` feature has `id`, `evidence`, `behaviour_doc`
- No feature may be `documented` with empty evidence
- Spreadsheet exports must be generated from this file, not the reverse
