#!/usr/bin/env bash
#
# Idempotent Cloud Agent bootstrap for the SQLite source tree.
#
# Installs the system prerequisites that are not part of the base image,
# configures a debug (--dev) build, and compiles the artifacts a developer
# needs on first use: the CLI shell, the amalgamation, the TCL test runner
# binary, and the TCL extension used by the test harness.
#
# This script is safe to run repeatedly: apt installs are no-ops when the
# packages are already present, ./configure simply regenerates the Makefile,
# and make targets rebuild incrementally.
set -euo pipefail

cd "$(dirname "$0")/.."

export DEBIAN_FRONTEND=noninteractive

# System packages beyond the base image:
#   tcl-dev            - required to build ./testfixture and run the TCL tests
#   zlib1g-dev         - the shell/library builds enable SQLITE_HAVE_ZLIB
#   libclang-rt-18-dev - clang's sanitizer runtime; `make devtest` builds
#                        fuzzcheck with -fsanitize=address and =undefined and
#                        the default cc on this image is clang.
sudo apt-get update -qq
sudo apt-get install -y --no-install-recommends \
  gcc \
  make \
  tcl-dev \
  zlib1g-dev \
  libclang-rt-18-dev

# The TCL extension must land on a directory that is on tclsh's $auto_path.
# On the stock image none of those directories are writable, so create the
# standard /usr/local/lib/tcltk entry (already on $auto_path) owned by the
# build user. tclextension-install then finds it automatically.
if [ ! -d /usr/local/lib/tcltk ]; then
  sudo install -d -o "$(id -un)" -g "$(id -gn)" /usr/local/lib/tcltk
fi

# Debug build with the developer feature set.
./configure --dev

# Core developer artifacts.
make sqlite3        # the CLI shell
make sqlite3.c      # the amalgamation (single-file distribution form)
make testfixture    # the TCL-based test runner binary
make tclextension-install
