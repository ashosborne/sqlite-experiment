#!/usr/bin/env bash
#
# Durable system setup for the SQLite source tree.
#
# This runs once to create the environment's base snapshot. It installs only
# system-level, out-of-/workspace state that survives a fresh git checkout:
# the toolchain packages and a writable TCL package directory. The actual
# in-tree build (./configure + make) lives in start.sh, because compiled
# artifacts written under /workspace are discarded when a new agent re-checks
# out the repository on boot.
#
# Safe to run repeatedly: apt installs are no-ops when packages are current,
# and the directory creation is guarded.
set -euo pipefail

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

# The TCL extension must be installed onto a directory that is on tclsh's
# $auto_path. On the stock image none of those are writable, so create the
# standard /usr/local/lib/tcltk entry (already on $auto_path) owned by the
# build user. start.sh's `make tclextension-install` then finds it with no
# further privileges required.
if [ ! -d /usr/local/lib/tcltk ]; then
  sudo install -d -o "$(id -un)" -g "$(id -gn)" /usr/local/lib/tcltk
fi
