# Portfolio inventory

Per-app source of truth for what the factory knows about a legacy application.

## Layout

```text
inventory/<APP_ID>/
  APP_MANIFEST.yaml   # source of truth (agents write; humans residual-gate)
  COVERAGE.md         # generated human glance. Never hand-edit
```

## How to start an app

1. Create `inventory/<APP_ID>/APP_MANIFEST.yaml` following `schemas/app-manifest.schema.md` (and `schemas/app-manifest.schema.json` for CI validation).
2. Generate `inventory/<APP_ID>/COVERAGE.md` from the manifest after each inventory update.
3. Never hand-edit `COVERAGE.md`. Regenerate it from `APP_MANIFEST.yaml`.

## Pack policy

This factory pack does **not** commit customer or pathfinder seed manifests. Operators bring their own `APP_ID` inventory. Generic methodology (Pathfinder sequencing, `WAIVED_PATHFINDER` as a waiver code) lives in the docs and skills, not under `inventory/`.

## Experimental growth path

Optional overnight Cloud Agent: `../prompts/estate-discovery-loop-v0.1.md` can propose candidate surfaces into this inventory. Humans still bind. Completeness remains a residual gate.
