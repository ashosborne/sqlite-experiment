#!/usr/bin/env python3
"""Record the human bind: ACCEPT_ALL_CURRENT_CANDIDATES (run 3 charter, Ash Osborne).

- Flips every `status: candidate` feature in every discovery/*/MANIFEST.yaml to `accepted`.
- Flips every `status: candidate` behaviour in APP_MANIFEST to `accepted`.
  (Surfaces stay `candidate`: the charter binds behaviours only; decision journaled.)
- Writes overnight/phase-b/BIND_ALL.md as the durable bind record (record-bind skill shape).
- Never touches non-candidate statuses; never invents or deletes IDs.
"""
import glob
import json
from datetime import datetime, timezone
from pathlib import Path

import jsonschema
import yaml

ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "inventory" / "sqlite-experiment" / "APP_MANIFEST.yaml"
SCHEMA = ROOT / "migration-factory" / "schemas" / "app-manifest.schema.json"
REASON = "operator requested full-catalogue deepen for this experiment"


def main() -> int:
    per_slice = {}
    for sf in sorted(ROOT.glob("discovery/*/MANIFEST.yaml")):
        m = yaml.safe_load(sf.read_text())
        flipped = []
        for feat in m.get("features") or []:
            if feat.get("status") == "candidate":
                feat["status"] = "accepted"
                flipped.append(feat["id"])
        if flipped:
            m["updated_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
            sf.write_text(yaml.safe_dump(m, sort_keys=False, width=120))
            per_slice[m["slice_id"]] = flipped

    app = yaml.safe_load(APP.read_text())
    n_beh = 0
    for b in app["behaviours"]:
        if b["status"] == "candidate":
            b["status"] = "accepted"
            n_beh += 1
    app["last_updated"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    app["updated_by"] = "sqlite-deepen-all"
    hist = {}
    for b in app["behaviours"]:
        hist[b["status"]] = hist.get(b["status"], 0) + 1
    app["counts"]["behaviours_by_status"] = hist
    jsonschema.validate(app, json.loads(SCHEMA.read_text()))
    header = (
        "# APP_MANIFEST — sqlite-experiment (portfolio inventory index)\n"
        "# Written by: estate-discovery-loop (Phase A) + sqlite-deepen-all (bind + Phase B). Humans residual-gate.\n"
        "# Schema: migration-factory/schemas/app-manifest.schema.json (v1)\n"
    )
    APP.write_text(header + yaml.safe_dump(app, sort_keys=False, width=120, allow_unicode=True))

    total = sum(len(v) for v in per_slice.values())
    out = ROOT / "overnight" / "phase-b" / "BIND_ALL.md"
    out.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "# Bind record — ALL current candidates (sqlite-experiment, run 3)",
        "",
        "Policy: `ACCEPT_ALL_CURRENT_CANDIDATES` (operator charter, run 3 paste = the human bind).",
        f"Reason (every row): {REASON}",
        "",
        f"**Accepted: {total} behaviours across {len(per_slice)} slices** "
        f"(slice MANIFEST features candidate→accepted; APP_MANIFEST behaviours candidate→accepted: {n_beh}).",
        "",
        "Scope notes:",
        "- APP_MANIFEST *surfaces* remain `candidate` — the charter binds behaviours; no surface decision implied.",
        "- The 3 unscanned hints (misc-mmapwarm, misc-memtrace, misc-pcachetrace) remain hints — not features, no cards.",
        "- No rejects, no defers in this bind. Prior prose defer *recommendations* in SME briefs stand as notes only.",
        "- No new IDs accepted; no slices deleted or merged.",
        "",
        "| Slice | Accepted feature IDs |",
        "| --- | --- |",
    ]
    for sid in sorted(per_slice):
        lines.append(f"| {sid} | {', '.join(per_slice[sid])} |")
    lines += [
        "",
        "Bound by: Ash Osborne, 2026-08-11 Europe/London, ACCEPT_ALL_CURRENT_CANDIDATES.",
        "Next: deepen Phase B for accepted only (skill: deepen-phase-b). Test gen stays locked.",
        "",
    ]
    out.write_text("\n".join(lines))
    print(f"bind recorded: {total} features accepted in {len(per_slice)} slices; "
          f"app behaviours accepted: {n_beh}; schema VALID")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
