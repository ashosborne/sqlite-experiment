#!/usr/bin/env python3
"""Upsert slice discovery manifests into the app-level APP_MANIFEST.yaml.

Estate-discovery-loop writer. Rules enforced here (stock loop, v0.1.2):
- Reads every discovery/<SLICE_ID>/MANIFEST.yaml (Phase A stubs).
- Derives APP_MANIFEST surfaces (deduped by locator) and behaviours (by feature id).
- Writes only `candidate`/`unknown` statuses. NEVER downgrades or overwrites a
  human-set status (accepted/documented/deferred/rejected/converted/verified).
- Appends scanned seeds; removes only the matching unscanned_hints (honest shrink).
- Recomputes the cached counts histogram; validates against the JSON schema.
- Regenerates COVERAGE.md via gen_coverage.py.

Idempotent: re-running with the same slice manifests is a no-op delta.
"""
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

import yaml

APP_ID = "sqlite-experiment"
ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "inventory" / APP_ID / "APP_MANIFEST.yaml"
DISCOVERY = ROOT / "discovery"
SCHEMA = ROOT / "migration-factory" / "schemas" / "app-manifest.schema.json"

HUMAN_STATUSES = {"accepted", "documented", "deferred", "rejected", "converted", "verified"}
AGENT_STATUSES = {"candidate", "unknown"}


def slug(text: str) -> str:
    s = re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
    return s or "unnamed"


def hint_key(hint: str) -> str:
    """First token after the 'src:'/'ext:' prefix — matches SLICE_ID by construction."""
    body = hint.split(":", 1)[1].strip() if ":" in hint else hint.strip()
    return body.split(" ", 1)[0].strip()


def main() -> int:
    app = yaml.safe_load(MANIFEST.read_text())
    surfaces = {s["locator"]: s for s in (app.get("surfaces") or [])}
    behaviours = {b["behaviour_id"]: b for b in (app.get("behaviours") or [])}
    scanned = list(app.get("scanned_seeds") or [])
    hints = list(app.get("unscanned_hints") or [])

    n_new_s = n_new_b = 0
    slice_files = sorted(DISCOVERY.glob("*/MANIFEST.yaml"))
    for sf in slice_files:
        sm = yaml.safe_load(sf.read_text())
        slice_id = sm["slice_id"]
        if slice_id not in scanned:
            scanned.append(slice_id)
        hints = [h for h in hints if hint_key(h) != slice_id]

        for feat in sm.get("features") or []:
            eps = feat.get("entrypoints") or []
            primary_sid = None
            for ep in eps:
                locator = ep["locator"]
                evidence = list(ep.get("evidence") or [])
                sid = f"sur-{slug(locator)}"
                if primary_sid is None:
                    primary_sid = sid
                if locator in surfaces:
                    ex = surfaces[locator]
                    ev = list(dict.fromkeys((ex.get("discovery_evidence") or []) + evidence))
                    ex["discovery_evidence"] = ev
                    continue
                repo_path = evidence[0].split(":")[0] if evidence else None
                surfaces[locator] = {
                    "surface_id": sid,
                    "kind": "other",  # closed enum; C seams have no Mule kind
                    "locator": locator,
                    "repo_path": repo_path,
                    "status": "candidate",
                    "slice_id": slice_id,
                    "notes": f"seam-kind={ep.get('kind', 'other')}; Phase A candidate (unbound)",
                    "discovery_evidence": evidence,
                }
                n_new_s += 1

            bid = feat["id"]
            feat_status = feat.get("status", "candidate")
            if feat_status not in AGENT_STATUSES:
                feat_status = "candidate"  # loop may only write candidate/unknown
            if bid in behaviours:
                if behaviours[bid].get("status") in HUMAN_STATUSES:
                    continue  # never touch human-set rows
                behaviours[bid]["name"] = feat.get("name")
                behaviours[bid]["notes"] = (
                    f"confidence={feat.get('confidence', 'inferred')}; {feat.get('summary', '')}".strip()
                )
                continue
            behaviours[bid] = {
                "behaviour_id": bid,
                "surface_id": primary_sid or f"sur-{slug(slice_id)}",
                "name": feat.get("name"),
                "status": feat_status,
                "slice_id": slice_id,
                "discovery_card": None,  # Phase B artefact; forbidden in this loop
                "legacy_green": False,
                "parity_green": False,
                "notes": f"confidence={feat.get('confidence', 'inferred')}; {feat.get('summary', '')}".strip(),
            }
            n_new_b += 1

    app["surfaces"] = sorted(surfaces.values(), key=lambda s: (s.get("slice_id") or "", s["surface_id"]))
    app["behaviours"] = sorted(behaviours.values(), key=lambda b: b["behaviour_id"])
    app["scanned_seeds"] = scanned
    app["unscanned_hints"] = hints
    s_hist, b_hist = {}, {}
    for s in app["surfaces"]:
        s_hist[s["status"]] = s_hist.get(s["status"], 0) + 1
    for b in app["behaviours"]:
        b_hist[b["status"]] = b_hist.get(b["status"], 0) + 1
    app["counts"] = {
        "surfaces_total": len(app["surfaces"]),
        "surfaces_by_status": s_hist,
        "behaviours_known": len(app["behaviours"]),
        "behaviours_by_status": b_hist,
        "unscanned_hints": len(hints),
        "legacy_green": sum(1 for b in app["behaviours"] if b.get("legacy_green")),
        "parity_green": sum(1 for b in app["behaviours"] if b.get("parity_green")),
    }
    app["last_updated"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    app["updated_by"] = "estate-discovery-loop"

    import jsonschema
    jsonschema.validate(app, json.loads(SCHEMA.read_text()))

    header = (
        "# APP_MANIFEST — sqlite-experiment (portfolio inventory index)\n"
        "# Written by: estate-discovery-loop (Phase A radar). Humans residual-gate.\n"
        "# Schema: migration-factory/schemas/app-manifest.schema.json (v1)\n"
    )
    MANIFEST.write_text(header + yaml.safe_dump(app, sort_keys=False, width=120, allow_unicode=True))
    print(
        f"upserted {len(slice_files)} slice manifests: +{n_new_s} surfaces, +{n_new_b} behaviours; "
        f"totals surfaces={len(app['surfaces'])} behaviours={len(app['behaviours'])} "
        f"candidates_total={len(app['surfaces']) + len(app['behaviours'])} "
        f"scanned={len(scanned)} hints_left={len(hints)}"
    )
    subprocess.run([sys.executable, str(ROOT / "overnight" / "bin" / "gen_coverage.py")], check=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
