#!/usr/bin/env python3
"""Generate Phase A artefacts for THIN unbundled ext/misc slices (resume run 2).

Each thin slice = one evidence-cited candidate feature refining a run-1 umbrella row
(umbrella rows are never modified). Reads slice definitions from a JSON file:

  [{"id": "misc-series", "umbrella": "misc-vtab-packs", "kind": "vtab",
    "name": "...", "seed": "...", "locator": "sqlite3_series_init()",
    "evidence": ["ext/misc/series.c:939"], "summary": "...",
    "deps": ["vtab-core"], "open_questions": ["..."]}, ...]

Writes overnight/seeds/<id>.md + discovery/<id>/{CANDIDATES.md,MANIFEST.yaml,SME_BRIEF.md}.
Refuses to overwrite an existing discovery/<id>/ (no thrash of prior runs).
"""
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def main(defs_path: str) -> int:
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    for d in json.loads(Path(defs_path).read_text()):
        sid = d["id"]
        sdir = ROOT / "discovery" / sid
        if sdir.exists():
            raise SystemExit(f"refusing to overwrite existing slice {sid}")
        sdir.mkdir(parents=True)
        ev_md = ", ".join(f"`{e}`" for e in d["evidence"])

        (ROOT / "overnight" / "seeds" / f"{sid}.md").write_text(
            f"# Seed — {sid} (resume run 2, unbundled from {d['umbrella']})\n"
            f"SLICE_ID: {sid}\nSLICE_SEED: \"{d['seed']}\"\n"
            f"SEED_ENTRYPOINTS: {d['locator']}\n"
            f"OUT_OF_SCOPE: other {d['umbrella']} members; run-1 umbrella row (untouched)\n"
            f"Rationale: thin unbundle of the run-1 {d['umbrella']} cluster per resume charter.\n"
        )
        (sdir / "CANDIDATES.md").write_text(
            f"# CANDIDATES — {sid} (Phase A, unbound; resume run 2)\n\n"
            f"All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella "
            f"`{d['umbrella']}` (umbrella row untouched). STOP for human bind.\n\n"
            f"| # | Candidate | Evidence | Why in slice |\n| --- | --- | --- | --- |\n"
            f"| 001 | {d['name']} | {ev_md} | {d['summary']} |\n"
        )
        manifest = {
            "slice_id": sid,
            "seed": d["seed"],
            "seed_entrypoints": [d["locator"]],
            "out_of_scope_hints": [f"other {d['umbrella']} members"],
            "phase": "A",
            "updated_at": now,
            "features": [{
                "id": f"{sid}-001",
                "name": d["name"],
                "status": "candidate",
                "confidence": "observed-in-code",
                "summary": d["summary"],
                "entrypoints": [{"kind": d.get("epkind", "other"),
                                 "locator": d["locator"],
                                 "evidence": d["evidence"]}],
                "dependencies": d.get("deps", []),
                "open_questions": d.get("open_questions", []),
            }],
        }
        (sdir / "MANIFEST.yaml").write_text(yaml.safe_dump(manifest, sort_keys=False, width=120))
        (sdir / "SME_BRIEF.md").write_text(
            f"# SME brief — {sid} (Phase A, resume run 2)\n"
            f"**Found:** 1 thin candidate unbundled from run-1 `{d['umbrella']}` umbrella.\n"
            f"**Recommended binds:** accept only if this extension is deployed downstream; "
            f"else defer (prose recommendation).\n"
            f"STOPPED for human bind. No Phase B performed.\n"
        )
        print(f"wrote thin slice {sid}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
