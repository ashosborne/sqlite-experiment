# vfs-unix-variants-003 — VxWorks variant paths

Slice: `vfs-unix-variants` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

OS_VXWORKS sections switch to named semaphores and RTOS path handling.

## Entrypoints (citations)

- `OS_VXWORKS sections` (other) — `src/os_unix.c:137`

## Inputs / outputs / observables

- VxWorks builds: named-semaphore locks; device-path filename normalization

## Behaviour (as implemented)

- OS_VXWORKS sections (src/os_unix.c:137 onward) swap posix locks for named semaphores (sem-style) and adjust unlink/path semantics for the RTOS

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- HDLC-ish path quirks; no /tmp conventions

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Embedded-RTOS scope question stands; census-level card by design
- Embedded RTOS in scope at all?

## Evidence

- `src/os_unix.c:137`
