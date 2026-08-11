#!/usr/bin/env python3
"""Generate inventory/<APP_ID>/COVERAGE.md from APP_MANIFEST.yaml.  (Factory SoT copy —
overnight/bin/gen_coverage.py is a thin wrapper around this file.)

COVERAGE.md is a generated glance surface — never hand-edit it (Field Guide,
Portfolio inventory). Counts and histograms only; completion percentages are
forbidden by the APP_MANIFEST anti-greenwash rules.

The document LEADS with **Operator progress** (behaviour `impl_in_modern`):
Done / Partial / Remaining in the modern implementation — the only signal that
answers "what is converted", as opposed to characterization flags.
"""
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def generate(app_id: str) -> int:
    manifest = ROOT / "inventory" / app_id / "APP_MANIFEST.yaml"
    coverage = ROOT / "inventory" / app_id / "COVERAGE.md"
    m = yaml.safe_load(manifest.read_text())
    surfaces = m.get("surfaces") or []
    behaviours = m.get("behaviours") or []
    scanned = m.get("scanned_seeds") or []
    hints = m.get("unscanned_hints") or []

    s_hist = Counter(s["status"] for s in surfaces)
    b_hist = Counter(b["status"] for b in behaviours)
    slice_hist = Counter((s.get("slice_id") or "?") for s in surfaces)
    legacy_green = sum(1 for b in behaviours if b.get("legacy_green"))
    parity_green = sum(1 for b in behaviours if b.get("parity_green"))

    # ---- Operator progress (impl_in_modern) ----
    def impl(b):
        return b.get("impl_in_modern") or "none"

    out_states = {"deferred", "rejected"}
    full = [b for b in behaviours if impl(b) == "full"]
    partial = [b for b in behaviours if impl(b) == "partial"]
    excluded = [b for b in behaviours if b["status"] in out_states]
    remaining = [b for b in behaviours
                 if impl(b) == "none" and b["status"] not in out_states]

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    lines = [
        f"# COVERAGE — {m['app_id']}",
        "",
        "> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**",
        "> Counts only. No completion percentages.",
        "> Characterization flags (`legacy_green`, replay-green tests) ≠ done;",
        "> use **Operator progress** below for modern-implementation status.",
        "",
        f"- Generated: {now}",
        f"- App status: `{m['status']}` · completeness: `{m['completeness']}`",
        f"- Manifest last_updated: {m.get('last_updated')} by `{m.get('updated_by')}`",
        "",
        "## Operator progress (modern implementation)",
        "",
        "| State | Count | Meaning |",
        "| --- | --- | --- |",
        f"| none | {len(remaining)} | Not started in modern |",
        f"| partial | {len(partial)} | Some modern execution; gaps in notes |",
        f"| full (converted) | {len(full)} | Behaviour done in modern; parity may still be UNVERIFIED |",
        f"| deferred / rejected | {len(excluded)} | Explicitly out |",
        "",
        "### Done in modern (impl_in_modern=full)",
        "",
    ]
    lines += [f"- `{b['behaviour_id']}` — {b.get('name') or ''}".rstrip(" —")
              for b in full] or ["- (none)"]
    def gaps_of(b):
        n = b.get("notes") or ""
        marker = "gaps:"
        if marker in n:
            return n[n.rindex(marker) + len(marker):].strip()
        return n or "(gaps not noted)"
    lines += ["", "### Partial in modern", ""]
    lines += [f"- `{b['behaviour_id']}` — {gaps_of(b)}" for b in partial] or ["- (none)"]
    lines += ["", "### Remaining (impl_in_modern=none|absent, not deferred)", ""]
    lines += [f"- `{b['behaviour_id']}` — {b.get('slice_id') or '?'} — "
              f"legacy_green {'yes' if b.get('legacy_green') else 'no'}"
              for b in remaining] or ["- (none)"]
    lines += [
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

    coverage.write_text("\n".join(lines))
    print(f"wrote {coverage} (behaviours={len(behaviours)} full={len(full)} "
          f"partial={len(partial)} remaining={len(remaining)})")
    return 0


if __name__ == "__main__":
    sys.exit(generate(sys.argv[1] if len(sys.argv) > 1 else "sqlite-experiment"))
