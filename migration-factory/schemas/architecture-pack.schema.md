# Architecture PACK schema (v0.1, draft)

**Machine schema for CI:** `architecture-pack.schema.json` (JSON Schema — validate PACK.yaml via ajv / equivalent). This markdown is human docs; automation must use the JSON Schema.

Path: `architecture/<MIGRATION_ID>/PACK.yaml`  
Overlay: `architecture/<MIGRATION_ID>/slice-overlays/<SLICE_ID>.yaml` (optional; deltas only)

## Status enum

`DRAFT` → `BOUND` → `SUPERSEDED`

- **DRAFT:** authoring; Conversion must refuse
- **BOUND:** immutable for Conversion; human `bound_by` + `bound_at` required
- **SUPERSEDED:** replaced by newer `version`; do not use for new Conversion PRs

## Required fields (`PACK.yaml`)

```yaml
pack_id: aviva-pmi-dotnet8-strangler
version: 1
status: DRAFT|BOUND|SUPERSEDED
migration_id: aviva-pmi
slice_ids: [ncd-quote]           # or broader; overlays narrow

target_stack:
  language: csharp
  runtime: net8.0
  framework: aspnet-core-minimal-api
  data_access: ef-core
  min_versions: { dotnet: "8.0" }

style: strangler-facade          # modular-monolith | services | strangler-facade | ...
layout:
  modern_root: modern/
  naming: Features/<Name>/

boundaries:
  in_scope: ["NCD read path"]
  stay_legacy: ["claims write"]
  anti_corruption: "modern NCD façade over legacy PolicyManager where needed"

api_contract_policy:
  mode: preserve-wire            # preserve-wire | versioned-break
  contract_paths: ["openapi/ncd.yaml"]
  sourced_vs_inferred: sourced

data:
  strategy: same-db-strangler    # same-db-strangler | dual-write | freeze-schema | ...
  schema_changes: none           # or explicit allow-list
  migration_tool: null

authn_authz:
  mode: keep
  notes: "bearer as today"

interop:
  modern_to_legacy: "none for this slice"
  legacy_to_modern: "router toggle / façade"

mapping_rules:
  - from: "WCF/SOAP operation"
    to: "Minimal API endpoint"
  - from: "EF6 DbContext"
    to: "EF Core DbContext + DI"
  - from: "God-class PolicyManager method"
    to: "Feature service + repository"

non_goals:
  - "No UI restyle"
  - "No microservice split this slice"
  - "No cloud rewrite"

quality_gates:
  characterization: REPLAY_GREEN
  commands: ["dotnet test tests/characterization/ncd-quote"]
  review_skills: ["migration-correctness"]   # if present

forbidden:
  - "string-concat SQL"
  - "new NuGet without allow-list"
  - "drive-by renames outside edit surface"

edit_surface:
  allow: ["modern/Features/Ncd/**", "modern/Program.cs"]
  deny: ["legacy/**", "ui/**"]

known_risks:
  - "Preserve NCD band semantics; characterization pins bands"
  - "No SQL concat in repositories"

evidence:
  discovery_manifest: discovery/ncd-quote/MANIFEST.yaml
  replay_green_run_ids: ["2026-08-04T1100Z-legacy-record"]

bound_by: null                   # required-when-BOUND (schema enforced)
bound_at: null                   # required-when-BOUND
# evidence.discovery_manifest + non-empty evidence.replay_green_run_ids required-when-BOUND
change_policy: "BOUND immutable; new version + SUPERSEDE + re-bind to change"

reference_skeleton: null         # optional path
```

## Overlay rules

- Overlay may only override **allow-listed** keys; must include `slice_id`, `parent_pack_id`, `parent_version`, `status`
- On conflict with parent for the same key: **fail validation** — do not auto-merge inventively
- **Effective pack resolution:** start from parent; for each allow-listed key present in overlay, overlay **wins**; keys missing in overlay are **inherited** from parent. Conversion must not guess beyond that.
- Conversion requires **both** parent `PACK.yaml` and the slice overlay (if an overlay file exists for that slice) to have `status: BOUND`. Parent BOUND + overlay DRAFT = **refuse Conversion**.
- If no overlay file exists for the slice, parent BOUND pack alone is the effective pack (slice must appear in parent `slice_ids`).

## Version retention

On SUPERSEDE: copy prior pack to `architecture/<MIGRATION_ID>/versions/v<N>/PACK.yaml` (and overlay snapshots if any). Leave citations to `pack_id@version` resolvable. Do not overwrite in place.


## BOUND + characterization waiver

When `status: BOUND` and `quality_gates.characterization` matches `^WAIVED_` (e.g. `WAIVED_PATHFINDER`):
- `evidence.replay_green_run_ids` **may be empty** (do **not** invent fake run IDs)
- **Required:** `quality_gates.waiver` (non-empty) and `evidence.waiver_record` (path to ADR)
- Prefer also setting `quality_gates.verification: DEFERRED` so Verification cannot claim PARITY=GREEN

Otherwise when BOUND: `replay_green_run_ids` must have `minItems: 1`.

## Conversion hard-fail (consumers)

1. Missing / schema-invalid / `status != BOUND` → exit
2. Evidence preconditions unmet → exit
3. Task touches `forbidden` or `non_goals` → exit
4. Ambiguous mapping_rules / empty required keys → `ARCHITECTURE_GAP`, exit
5. PR must cite `pack_id@version`
