# PASTE — estate discovery overnight (sqlite-experiment)

Use the **stock** operator playbook loop. Do not invent a parallel discovery process.

1. Read and obey in full: `migration-factory/prompts/estate-discovery-loop-v0.1.md`
2. Read: `migration-factory/docs/FIELD-GUIDE.md`, `migration-factory/docs/OPERATOR-RUNBOOK.md`, `migration-factory/prompts/discovery-agent-v0.2.md`
3. Apply the operator charter below (replaces the example charter in the loop prompt). If the loop file shows example caps like MAX_SLICES_PHASE_A: 12 or MAX_NEW_CANDIDATES: 40, **ignore those** — this charter wins.
4. Run the loop until a stop condition; write `overnight/MORNING_BRIEF.md`

This estate is a large C library (SQLite). Prefer module/API seams in `src/` (and optionally `ext/`) as seeds — callable boundaries, subsystems with clear ownership — not "whole engine" mega-slices. Treat build/tooling and documentation trees as out of scope unless they define a real runtime entrypoint.

---

## Operator charter

```yaml
OPERATOR: Ash Osborne
APP_ID: sqlite-experiment
REPO_ROOT: .
FACTORY_ROOT: migration-factory
MAX_ITERATIONS: 40
MAX_NEW_SEEDS_PER_ITER: 4   # breadth: multiple thin Phase A seeds per iteration
PHASE_B: false
AUTO_BIND: false
AUTO_ACCEPT: false
ALLOW_CONVERSION: false
STOP_WHEN_NO_NEW_SURFACES: 4
INITIAL_SEEDS: []
ALLOWLIST_PATHS:
  - src
  - ext
OUT_OF_SCOPE_HINTS:
  - art/
  - autoconf/
  - autosetup/
  - doc/
  - tool/
  - mptest/
  - autoconf/
  - test/          # upstream test harness / fixtures — not product migration slices
  - .fossil-settings/
MAX_FILES_TOUCHED: 8000
MAX_RUNTIME_HINT_HOURS: 14
MAX_NEW_CANDIDATES: 250
MAX_SLICES_PHASE_A: 0   # 0 = no artificial Phase A slice cap; keep going until stop conditions
WRITE_SCOPE: factory-artefacts-only
NO_COMMITS_TO_DEFAULT_BRANCH: true
COMMIT_AS: estate-discovery-loop
```

Optimise for **breadth**: keep seeding new thin slices until stop conditions. Do not stop early because a soft “enough slices” threshold feels reached.

## Hard rules (stock loop)

- Candidates only. No bind, no Phase B, no test gen, no conversion.
- No completeness percentages. No "all slices found."
- Write only under `discovery/**`, `inventory/**`, `overnight/**` (and factory pack if needed).
- Do not edit SQLite product source under `src/` or `ext/`.
- Abort if `overnight/stop.txt` appears; still write MORNING_BRIEF.

## Success

Richer `inventory/sqlite-experiment/APP_MANIFEST.yaml` + `COVERAGE.md` + `overnight/MORNING_BRIEF.md` with proposed next human binds. Estate scan remains incomplete by design.
