#!/usr/bin/env python3
"""Phase B writer: sync slice MANIFEST feature statuses into APP_MANIFEST.

Mapping (app-manifest behaviour enum has no blocked/needs-SME):
  slice `accepted`   -> app `accepted`
  slice `documented` -> app `documented` + discovery_card pointer
  slice `blocked` / `needs-SME` -> app stays `accepted`, reason recorded in the behaviour's notes
Other statuses (candidate on non-bound rows, human side-exits) are left untouched.
Never invents IDs; never downgrades documented/converted/verified rows.

Regenerates COVERAGE.md and validates against the JSON schema.
"""
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

import jsonschema
import yaml

ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "inventory" / "sqlite-experiment" / "APP_MANIFEST.yaml"
SCHEMA = ROOT / "migration-factory" / "schemas" / "app-manifest.schema.json"
DOWNGRADE_GUARD = {"documented": 2, "converted": 3, "verified": 4}
RANK = {"candidate": 0, "accepted": 1, "documented": 2, "converted": 3, "verified": 4}


def main() -> int:
    app = yaml.safe_load(APP.read_text())
    beh = {b["behaviour_id"]: b for b in app["behaviours"]}
    n_doc = n_flag = 0
    for sf in sorted(ROOT.glob("discovery/*/MANIFEST.yaml")):
        m = yaml.safe_load(sf.read_text())
        for feat in m.get("features") or []:
            b = beh.get(feat["id"])
            if b is None:
                continue
            cur = b.get("status")
            fs = feat.get("status")
            if fs == "documented":
                if RANK.get(cur, 0) < DOWNGRADE_GUARD["documented"]:
                    b["status"] = "documented"
                    n_doc += 1
                b["discovery_card"] = f"discovery/{m['slice_id']}/{feat.get('behaviour_doc', '')}" \
                    if feat.get("behaviour_doc") else b.get("discovery_card")
            elif fs in ("blocked", "needs-SME"):
                note = f"{fs}: {'; '.join(feat.get('open_questions') or ['reason in slice MANIFEST'])}"
                base = (b.get("notes") or "").split(" | ")[0]
                b["notes"] = f"{base} | {note}" if base else note
                n_flag += 1
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
    print(f"phase-b upsert: documented total now {hist.get('documented', 0)} (+{n_doc} this pass); "
          f"blocked/needs-SME noted: {n_flag}; histogram {hist}; schema VALID")
    subprocess.run([sys.executable, str(ROOT / "overnight" / "bin" / "gen_coverage.py")], check=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
