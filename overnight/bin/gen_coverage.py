#!/usr/bin/env python3
"""Thin wrapper — the single source of truth for COVERAGE generation is
migration-factory/inventory/gen_coverage.py (Field Guide, Portfolio inventory)."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "migration-factory" / "inventory"))
from gen_coverage import generate  # noqa: E402

if __name__ == "__main__":
    sys.exit(generate(sys.argv[1] if len(sys.argv) > 1 else "sqlite-experiment"))
