# vfs-unix-variants-002 — Proxy locking (conch file protocol)

Slice: `vfs-unix-variants` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

proxyIoMethods route locks through a conch file; macOS-specific concurrency semantics.

## Entrypoints (citations)

- `proxyIoMethods` (other) — `src/os_unix.c:5928`, `src/os_unix.c:5923`, `src/os_unix.c:8207`

## Inputs / outputs / observables

- Proxy locking: conch file <db>.conch created/renewed; lock requests routed to proxy path; macOS-only pragmas (lock_proxy_file)

## Behaviour (as implemented)

- proxyIoMethods (src/os_unix.c:5928, proxyLock decl :5923) intercept lock calls (switch-in :8207,:8231): a host-specific conch file elects the lock-holder proxy path so AFP/NFS clients don't fight over byte-range locks

## Validation rules found in code

- Conch-format/version checks; takeover protocol on stale conch

## Edge cases found in code

- Conch breaking on host identity change (documented recovery path)

## Dependencies

- vfs-unix-variants-001

## Assumptions / unknowns

- macOS-only; defer stands unless such deployments exist

## Evidence

- `src/os_unix.c:5928`
- `src/os_unix.c:5923`
- `src/os_unix.c:8207`
