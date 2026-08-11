# CANDIDATES — misc-cksumvfs (Phase A, unbound; resume run 2)

All `candidate`, confidence `observed-in-code`. Thin unbundle refining run-1 umbrella `misc-vfs-shims` (umbrella row untouched). STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | cksumvfs per-page checksums | `ext/misc/cksumvfs.c:833` | 8-byte per-page checksums in reserve bytes; verification on read — FILE-FORMAT IMPACT (reserve-bytes requirement) |
