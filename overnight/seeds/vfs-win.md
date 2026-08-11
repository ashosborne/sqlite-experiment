# Seed — vfs-win (resume run 2)
SLICE_ID: vfs-win
SLICE_SEED: "Windows VFS implementation (src/os_win.c)"
SEED_ENTRYPOINTS: sqlite3_os_init() [win], winOpen()
OUT_OF_SCOPE: unix VFS (run-1 vfs-os-abstraction-002), core VFS registry
Rationale: run-1 residual 4 — os_win.c was inventoried inside the vfs-os-abstraction umbrella but not seeded. New entrypoints (winVfs table, shm, mmap) not covered by the run-1 card.
