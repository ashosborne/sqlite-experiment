# testgen/prepare-statement-api — characterization pack (run 4, spine batch)

Source: `discovery/prepare-statement-api/` (bound + documented; card prepare-statement-api-002 only).
Cases: 3 specified, assert mode TO_BE_RECORDED. Harness: C stub under `harness/` (public C API only;
no test/, no TCL, no testfixture, no sqllogictest).

## Ready-for-Test-execution checklist
- [x] Every approved case has a spec + TRACEABILITY row + card citation
- [x] No expected-value asserts (TO_BE_RECORDED only); no green claims
- [x] Harness stub present (NOT compiled this run); C003 armor caveat documented for RECORD
- [x] DEFERRED.md explicit (incl. all other prepare-statement IDs)
- [ ] Test execution RECORD (next stage, human-triggered)
