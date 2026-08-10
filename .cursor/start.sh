#!/usr/bin/env bash
#
# Per-boot build of the SQLite developer artifacts.
#
# Compiled outputs under /workspace do not survive the git checkout that runs
# when a fresh agent boots from a prebuilt environment, so the debug build is
# (re)produced here on every start against the source that is actually checked
# out. The build is incremental and idempotent: ./configure just regenerates
# the Makefile and make only rebuilds what changed.
#
# Produces:
#   ./sqlite3        - the CLI shell (the primary application)
#   ./sqlite3.c      - the amalgamation (single-file distribution form)
#   ./testfixture    - the TCL-based test runner used by test/testrunner.tcl
#   the SQLite TCL extension, version-aligned with the checked-out source
set -euo pipefail

cd "$(dirname "$0")/.."

# Guard: the TCL package directory normally comes from install.sh, but recreate
# it if this boot did not inherit that state so tclextension-install can write.
if [ ! -d /usr/local/lib/tcltk ]; then
  sudo install -d -o "$(id -un)" -g "$(id -gn)" /usr/local/lib/tcltk
fi

./configure --dev

njobs="$(nproc 2>/dev/null || echo 2)"
make -j"$njobs" sqlite3 sqlite3.c testfixture
make tclextension-install

echo "SQLite environment ready: $(./sqlite3 --version)"
