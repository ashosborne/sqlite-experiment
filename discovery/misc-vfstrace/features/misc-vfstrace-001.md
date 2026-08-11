# misc-vfstrace-001 — vfstrace call-tracing shim

Slice: `misc-vfstrace` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Wraps a VFS and prints every method call with args — debugging aid; registered via C API

## Entrypoints (citations)

- `vfstrace_register()` (other) — `ext/misc/vfstrace.c:1138`, `ext/misc/vfstrace.c:21`

## Inputs / outputs / observables

- Every VFS call printed with args/results to the configured output; nested shim naming ('trace/<parent>')

## Behaviour (as implemented)

- vfstrace_register (ext/misc/vfstrace.c:1138; usage doc :21): creates a named tracing VFS over a parent; per-method printf-style logging with a user output callback

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Can stack multiple shims (trace over trace)

## Dependencies

- vfs-os-abstraction

## Assumptions / unknowns

- Dev tooling; defer stands

## Evidence

- `ext/misc/vfstrace.c:1138`
- `ext/misc/vfstrace.c:21`
