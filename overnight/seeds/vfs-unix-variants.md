# Seed — vfs-unix-variants (resume run 2)
SLICE_ID: vfs-unix-variants
SLICE_SEED: "alternate unix locking styles: proxy locking, dotlock/flock/sem/afp/nfs, VxWorks"
SEED_ENTRYPOINTS: proxyIoMethods, SQLITE_ENABLE_LOCKING_STYLE paths in src/os_unix.c
OUT_OF_SCOPE: default posix path (run-1 vfs-os-abstraction-002 — new entrypoints only)
Rationale: run-1 residual 4 — explicitly left out of the run-1 unix card.
