# APP_MANIFEST schema (v1)

Path: `inventory/<APP_ID>/APP_MANIFEST.yaml`  
Machine schema: `schemas/app-manifest.schema.json`

## Purpose

Portfolio-level **index** of an application’s known surfaces and behaviours for migration tracking when Discovery is **slice-scoped** and inventory is **incomplete by default**.

- Slice `discovery/<slice>/MANIFEST.yaml` = deep bound truth for one slice  
- `APP_MANIFEST.yaml` = wide, growing index + rolled-up status  
- Never pretends the app inventory is complete unless a **human** residual gate says so  

## Anti-greenwash (mandatory)

1. **No completion %.** Emit histogram/counts only (`verified`, `converted`, `documented`, `known_behaviours`, `unscanned_surfaces`, …).
2. **`verified` requires Verification COMPARE `PARITY=GREEN` only.**  
   Under `WAIVED_PATHFINDER` (or any characterization/verification waiver): **do not** set behaviour `status: verified`. Max = `converted` + `parity: WAIVED`.
3. **Split evidence flags (do not collapse):**
   - `legacy_green: true` only after Test execution **RECORD → REPLAY_GREEN** on legacy goldens  
   - `parity_green: true` only after Verification **COMPARE** modern vs those goldens  
   - Provisional modern unit tests (**never** set either flag)
4. **TRACEABILITY stays slice-local.** APP_MANIFEST links `behaviour_id` → paths + **rolled-up** status (weakest case). No duplicated case rows.
5. Ban treating provisional modern tests as `legacy_green` or `parity_green`.

## Status machines

### App `status`
`in_progress` → `residual_review` → `complete_bound` | `abandoned`

### App `completeness`
- `incomplete` — default  
- `bound_incomplete_ok` — human accepted known residuals  
- `complete_bound` — human residual gate passed (still not a %)

### Surface `status`
`candidate` | `accepted` | `deferred` | `rejected` | `unknown`

### Behaviour `status`
Happy path: `candidate` → `accepted` → `documented` → `converted` → `verified`  

Side exits: `deferred` | `rejected` | `unknown`

| Status | Meaning | Gate |
| --- | --- | --- |
| `documented` | Discovery card (+ usually testgen) exists | Discovery bind |
| `converted` | Conversion PR landed under BOUND PACK | Conversion; set `parity: UNVERIFIED` or `WAIVED` |
| `verified` | Verification COMPARE `PARITY=GREEN` | **Only** Verification; forbidden under waiver |

### Behaviour `parity`
`null` | `UNVERIFIED` | `GREEN` | `FAIL` | `WAIVED`

- `WAIVED` ⇔ characterization and/or verification explicitly waived (e.g. `WAIVED_PATHFINDER`)  
- `GREEN` ⇔ COMPARE passed — required for `status: verified`  
- Pathfinder waived max: `status: converted`, `parity: WAIVED`, `legacy_green: false`, `parity_green: false`

## Who writes what

| Actor | May write |
| --- | --- |
| Discovery | Create/update surfaces & behaviours; `candidate`→`accepted`/`documented`; `scanned_seeds` / `unscanned_hints`; never `complete_bound` |
| Test gen / Test exec | Pointers only (`traceability` path, `legacy_green` after real REPLAY_GREEN); no inventing surfaces |
| Conversion | `documented`→`converted`; `conversion_prs`; `parity: UNVERIFIED` or keep/set `WAIVED` if pack waiver in force |
| Verification | `converted`→`verified` **only** on `PARITY=GREEN`; else leave `converted` + `parity: FAIL` |
| Human | Residual gate; deferred/rejected; app `complete_bound`; linkage confirm |

Later stages **bump status + pointers**. New surfaces found mid-Conversion → `DISCOVERY_GAP` / candidate back to Discovery.

## Counts (dashboard — no %)

Recommended rollup (compute; optional `counts` block may be cached but must be regenerable):

- `surfaces_total` / by status  
- `behaviours_known` (= all except pure scratch if you filter unknowns)  
- `behaviours_by_status` histogram  
- `unscanned_hints` length  
- `legacy_green_count` / `parity_green_count` (flags, not status)

## Relationship to other artefacts

| Artefact | Role |
| --- | --- |
| `discovery/<slice>/MANIFEST.yaml` | Canonical slice detail — wins on conflict for card content |
| `testgen/` + `tests/characterization/` TRACEABILITY | Case-level SoT; APP_MANIFEST links only |
| Architecture `PACK.yaml` | Bound target; cite `pack_id@version` on behaviours when in scope |
| Conversion / Verification | Status bumps + PR / run pointers |
| OS Discovery + Linkage | Fill `linkage.*` after joint SME gate — do not stuff host processes in as behaviours |

## Schema versioning

`schema_version: 1` required. Breaking changes → v2 + migration note in Field Guide.
