#!/usr/bin/env python3
"""Generate inventory/<APP_ID>/COVERAGE.md from APP_MANIFEST.yaml.

COVERAGE.md is a generated glance surface — never hand-edit it (Field Guide,
Portfolio inventory). Counts and histograms only; completion percentages are
forbidden by the APP_MANIFEST anti-greenwash rules.
"""
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

import yaml

APP_ID = "sqlite-experiment"
ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "inventory" / APP_ID / "APP_MANIFEST.yaml"
COVERAGE = ROOT / "inventory" / APP_ID / "COVERAGE.md"


def main() -> int:
    m = yaml.safe_load(MANIFEST.read_text())
    surfaces = m.get("surfaces") or []
    behaviours = m.get("behaviours") or []
    scanned = m.get("scanned_seeds") or []
    hints = m.get("unscanned_hints") or []

    s_hist = Counter(s["status"] for s in surfaces)
    b_hist = Counter(b["status"] for b in behaviours)
    slice_hist = Counter((s.get("slice_id") or "?") for s in surfaces)
    legacy_green = sum(1 for b in behaviours if b.get("legacy_green"))
    parity_green = sum(1 for b in behaviours if b.get("parity_green"))

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    lines = [
        f"# COVERAGE — {m['app_id']}",
        "",
        "> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**",
        "> Counts only. No completion percentages. Inventory progress, not migration progress.",
        "",
        f"- Generated: {now}",
        f"- App status: `{m['status']}` · completeness: `{m['completeness']}`",
        f"- Manifest last_updated: {m.get('last_updated')} by `{m.get('updated_by')}`",
        "",
        "## Counts",
        "",
        f"| Metric | Count |",
        f"| --- | --- |",
        f"| Surfaces total | {len(surfaces)} |",
        f"| Behaviours known | {len(behaviours)} |",
        f"| Seeds scanned | {len(scanned)} |",
        f"| Unscanned hints (residual) | {len(hints)} |",
        f"| legacy_green flags | {legacy_green} |",
        f"| parity_green flags | {parity_green} |",
        "",
        "## Surfaces by status",
        "",
        "| Status | Count |",
        "| --- | --- |",
    ]
    for k in sorted(s_hist):
        lines.append(f"| {k} | {s_hist[k]} |")
    lines += ["", "## Behaviours by status", "", "| Status | Count |", "| --- | --- |"]
    for k in sorted(b_hist):
        lines.append(f"| {k} | {b_hist[k]} |")
    lines += ["", "## Surfaces per slice", "", "| Slice | Surfaces |", "| --- | --- |"]
    for k in sorted(slice_hist):
        lines.append(f"| {k} | {slice_hist[k]} |")
    lines += ["", "## Scanned seeds", ""]
    lines += [f"- `{s}`" for s in scanned] or ["- (none)"]
    lines += ["", "## Unscanned hints (residual register)", ""]
    lines += [f"- {h}" for h in hints] or ["- (none)"]
    if m.get("notes"):
        lines += ["", "## Notes", "", str(m["notes"])]
    lines.append("")

    COVERAGE.write_text("\n".join(lines))
    print(f"wrote {COVERAGE} (surfaces={len(surfaces)} behaviours={len(behaviours)} "
          f"scanned={len(scanned)} hints={len(hints)})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
