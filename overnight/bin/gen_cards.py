#!/usr/bin/env python3
"""Write Discovery Phase B behaviour cards (discovery-agent v0.2 card shape).

Input: JSON list of authored card content. Structural fields (name, entrypoints,
evidence, dependencies) come from the slice MANIFEST — cards never invent locators.

  [{"id": "connection-lifecycle-api-001",
    "observables": ["..."], "behaviour": ["..."], "validation": ["..."],
    "edge_cases": ["..."], "assumptions": ["..."],
    "confidence": "observed-in-code" (default) | "inferred",
    "status": "documented" (default) | "needs-SME" | "blocked",
    "reason": "required for needs-SME/blocked",
    "extra_evidence": ["file:line", ...] (optional additions)}]

Effects per card: writes discovery/<slice>/features/<id>.md; flips the slice MANIFEST
feature to documented (+ behaviour_doc) or needs-SME/blocked (+ reason in open_questions).
Resume-safe: skips if the card file exists AND the manifest row is already documented.
"""
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def sect(title, items):
    if not items:
        return f"## {title}\n\n- (none found in code)\n"
    return f"## {title}\n\n" + "\n".join(f"- {i}" for i in items) + "\n"


def main(defs_path: str) -> int:
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    cards = json.loads(Path(defs_path).read_text())
    by_slice = {}
    for c in cards:
        sid = c["id"].rsplit("-", 1)[0]
        by_slice.setdefault(sid, []).append(c)

    written = skipped = 0
    for sid, items in by_slice.items():
        mf = ROOT / "discovery" / sid / "MANIFEST.yaml"
        m = yaml.safe_load(mf.read_text())
        feats = {f["id"]: f for f in m["features"]}
        fdir = ROOT / "discovery" / sid / "features"
        fdir.mkdir(exist_ok=True)
        for c in items:
            fid = c["id"]
            feat = feats.get(fid)
            if feat is None:
                raise SystemExit(f"unknown feature id {fid} (no invented IDs allowed)")
            card_path = fdir / f"{fid}.md"
            if card_path.exists() and feat.get("status") == "documented":
                skipped += 1
                continue
            if feat.get("status") not in ("accepted", "documented", "blocked", "needs-SME"):
                raise SystemExit(f"{fid}: status {feat.get('status')} not bound — refuse to deepen")
            status = c.get("status", "documented")
            conf = c.get("confidence", feat.get("confidence", "observed-in-code"))
            evidence = []
            for ep in feat.get("entrypoints") or []:
                evidence += ep.get("evidence") or []
            evidence += c.get("extra_evidence", [])
            evidence = list(dict.fromkeys(evidence))
            eps = "\n".join(
                f"- `{ep['locator']}` ({ep.get('kind','other')}) — " +
                ", ".join(f"`{e}`" for e in (ep.get("evidence") or []))
                for ep in feat.get("entrypoints") or [])
            body = [
                f"# {fid} — {feat.get('name')}",
                "",
                f"Slice: `{sid}` · Status: `{status}` · Confidence: `{conf}` · Card written: {now}",
                "As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.",
                "",
                f"## Summary\n\n{feat.get('summary','')}\n",
                f"## Entrypoints (citations)\n\n{eps}\n",
                sect("Inputs / outputs / observables", c.get("observables")),
                sect("Behaviour (as implemented)", c.get("behaviour")),
                sect("Validation rules found in code", c.get("validation")),
                sect("Edge cases found in code", c.get("edge_cases")),
                sect("Dependencies", feat.get("dependencies")),
                sect("Assumptions / unknowns", (c.get("assumptions") or []) +
                     (feat.get("open_questions") or [])),
                sect("Evidence", [f"`{e}`" for e in evidence]),
            ]
            if status in ("needs-SME", "blocked"):
                body.insert(4, f"> **{status.upper()}**: {c['reason']}\n")
            card_path.write_text("\n".join(body))
            feat["behaviour_doc"] = f"features/{fid}.md"
            feat["status"] = status
            feat["confidence"] = conf
            if status in ("needs-SME", "blocked"):
                oq = feat.get("open_questions") or []
                if c["reason"] not in oq:
                    feat["open_questions"] = oq + [c["reason"]]
            written += 1
        m["phase"] = "B"
        m["updated_at"] = now
        mf.write_text(yaml.safe_dump(m, sort_keys=False, width=120))
    print(f"cards written: {written}, skipped (resume-safe): {skipped}, slices touched: {len(by_slice)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1]))
