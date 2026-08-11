# MORNING BRIEF — factory fix: operator "done vs remaining" scoreboard (run 19)

APP_ID: sqlite-experiment · Branch: cursor/sqlite-estate-discovery-d22c
Charter: FULL_AUTONOMY, COMMIT_AS factory-operator-progress-scoreboard. No pack supersede,
no goldens touched, no parity flips, no completion percentages.

## 1. Factory gap statement

After engine runs v5–v9 a human operator could not answer "what is done in Rust and what
remains": `legacy_green` only means a C behaviour is frozen, discovery cards stay
`documented` forever, `parity_green` is rightly 0 until COMPARE, and COVERAGE.md led with
seed histograms while explicitly disclaiming migration progress. The only conversion-progress
signal did not exist in the schema, so no run was obliged to write it — a core factory
defect surfaced by this experiment, not an operator mistake.

## 2. Core factory files touched (Job 1 — so this never recurs)

- `migration-factory/schemas/app-manifest.schema.json` — additive behaviour field
  **`impl_in_modern: none|partial|full`** (absent = none, schema_version stays 1) with
  gates: `converted` ⇒ `impl_in_modern: full` + parity set; `partial` ⇒ status must stay
  documented/accepted (+ notes required); `verified` ⇒ `full`. Current manifests validate.
- `migration-factory/schemas/app-manifest.schema.md` — field documented with the one-line
  operator meaning: **done in modern / partial / not started**.
- `migration-factory/docs/FIELD-GUIDE.md` — Portfolio inventory: operator glance path
  (COVERAGE **Operator progress** first), anti-pattern (`legacy_green` ≠ migrated), hard
  rule: conversion/engine runs MUST bump inventory in the same change set + regenerate
  COVERAGE ("shipping modern code without the inventory bump is a process defect");
  Conversion feature loop gained mandatory step 6 (inventory bump); generator SoT path.
- `migration-factory/docs/OPERATOR-RUNBOOK.md` — glance path + anti-pattern + routing-table
  row updated.
- `migration-factory/prompts/conversion-agent-v0.1.md` — mandatory Done-checklist item:
  classify each in-batch behaviour, bump only `full` to converted, `partial` stays
  documented with gaps, parity UNVERIFIED, regenerate COVERAGE, brief pastes the histogram.
- `migration-factory/prompts/overnight-conductor-v0.1.md` — same rule for
  `ALLOW_CONVERSION` runs.
- `migration-factory/skills/operator/conversion-pr/SKILL.md` + `skills/architecture-pack/SKILL.md`
  — done-checks include the inventory bump + COVERAGE regen.
- **`migration-factory/inventory/gen_coverage.py`** — new factory-owned SoT generator;
  `overnight/bin/gen_coverage.py` is now a thin wrapper. COVERAGE.md leads with
  **Operator progress** (Done / Partial / Remaining) and the banner now reads
  "characterization flags ≠ done". Still no percentages; still never hand-edited.

## 3. sqlite-experiment scoreboard (Job 2 — honest catch-up)

194 behaviours classified; schema-validated; `updated_by: factory-operator-progress-scoreboard`.

| State | Count | Meaning |
|---|---|---|
| none | **108** | Not started in modern |
| partial | **75** | Some real modern execution; gaps listed in notes |
| full (converted) | **11** | Done in modern; parity **UNVERIFIED** |
| deferred / rejected | 0 | Explicitly out |

**Done in modern (11, now `status: converted`, `parity: UNVERIFIED`, pack sqlite-experiment-c-to-rust@9):**
engine-kitchen-001 · engine-files-001/-002/-003/-004 · engine-subquery-001/-002 ·
engine-join-001/-002 · exec-convenience-api-001 (the exec callback loop itself) ·
misc-zorder-001 (complete 2-function extension, computed).

**Partial (75) — representative gaps (full list in COVERAGE.md):**
select-codegen-001 (nested-loop joins/subqueries; **no** flattening/planner) ·
pragma-surface-001 (11 of ~70 pragmas) · builtin-scalar-agg-funcs-001 (~10 of ~60 scalars) ·
json-funcs-001..004 (real parser; no JSONB/JSON5/wildcard paths) · triggers (AFTER INSERT only) ·
foreign-keys (immediate + CASCADE only) · prepare-statement-api (state machine, no VDBE) ·
backup/serialize (empty-image scope) · window-functions (2 pinned shapes).
Deliberately under-claimed: **no** partial slice was bumped to converted.

**Remaining (108) — top slices still `none`:** date-time-funcs (4),
compile-options-omit-enable, connection-lifecycle-api tail, fts5, jni-java-surface,
session, vfs-os-abstraction, vfs-unix-variants, wasm-js-api (3 each), analyze-stats,
blob-io-api, btree, pager, wal… — i.e. the whole storage/concurrency/extension estate.

## 4. Explicit honesty line

**SQLite is NOT migrated.** 11 of 194 behaviours are done in modern; all 11 are
`parity: UNVERIFIED` (Verification COMPARE has never run; parity_green remains 0;
no behaviour is `verified`). Characterization (104 legacy_green) is evidence of frozen
C behaviour, not of conversion.

## 5. Next call

Resume engine work using the new scoreboard: (a) date/time engine (moves 4 none → full
candidates and clears the v8 defers), (b) disk debt (overflow pages + on-disk UNIQUE
autoindexes — upgrades ddl-schema-002 and engine-files notes), or (c) WAL law change.
Pack v10 + goldens first; whichever run lands modern behaviour must bump
`impl_in_modern` in the same change set — the factory now enforces that expectation.
