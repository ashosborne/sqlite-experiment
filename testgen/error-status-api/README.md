# testgen/error-status-api — characterization pack (run 4, spine batch)

Source: `discovery/error-status-api/` (bound + documented; card error-status-api-001).
Cases: 2 specified, assert mode TO_BE_RECORDED (see TRACEABILITY.yaml). Harness: C stub under
`harness/` (public C API only; no test/, no TCL, no testfixture, no sqllogictest).

## Ready-for-Test-execution checklist
- [x] Every approved case has a spec + TRACEABILITY row + card citation
- [x] No expected-value asserts (TO_BE_RECORDED only); no green claims
- [x] Harness stub present (NOT compiled this run — no library build in workspace; Test execution builds per overnight/BASELINE.md and captures the compileoption fingerprint)
- [x] DEFERRED.md explicit
- [ ] Test execution RECORD (next stage, human-triggered)
