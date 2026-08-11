# Journal — run 3: bind-all + Phase B full-catalogue deepen (sqlite-experiment)

Charter: user paste = human bind (ACCEPT_ALL_CURRENT_CANDIDATES) + Phase B deepen for all accepted.
COMMIT_AS sqlite-deepen-all. No test gen / RECORD / PACK / Conversion. Writes: discovery/, inventory/, overnight/ only.
Resumed from 0f1fa004 on cursor/sqlite-estate-discovery-d22c.

---

## Setup + baseline + bind

- stop.txt: absent. Read record-bind + deepen-phase-b skills (context gate: Field Guide, runbook, discovery v0.2, both schemas re-read).
- Tooling added under overnight/bin/: bind_all.py (bind recorder), phase_b_upsert.py (Phase B status sync — run-1 radar upsert NOT used here; it force-writes candidate), gen_cards.py (v0.2 card writer; structural fields from slice MANIFESTs — no invented locators; resume-safe).
- overnight/BASELINE.md written: default Unix amalgamation pin + compileoption fingerprint; census cards stay one card each; completeness incomplete.
- BIND: 185 features candidate→accepted across 107 slice MANIFESTs; APP_MANIFEST behaviours candidate→accepted (185). Surfaces intentionally left candidate (charter binds behaviours; decision recorded in BIND_ALL.md). 3 unscanned hints untouched. Schema VALID.
- Bind record: overnight/phase-b/BIND_ALL.md — "Ash Osborne, 2026-08-11 Europe/London, ACCEPT_ALL_CURRENT_CANDIDATES."

## Batch 1 — core API spine (15 cards)

- stop.txt: absent.
- connection-lifecycle-api 001-004, prepare-statement-api 001-006, exec-convenience-api 001-002, backup-api 001-003 → documented, all observed-in-code (symbol+line evidence).
- Coercion matrix (prepare-statement-api-004) kept as one card pinning the seam; matrix rows deferred to Test-gen granularity.
- Upsert: PASS (documented=15). COVERAGE regenerated.

## Batch 2 — remaining public API seams (15 cards)

- blob-io(2), serialize-memdb(2), loadext(2), unlock-notify(1), auth(2), attach(3), error-status(3) → documented, observed-in-code.
- unlock-notify card documents the gated contract explicitly (ENABLE_UNLOCK_NOTIFY + shared-cache NOT in the default baseline build).
- Upsert: PASS (documented=30). COVERAGE regenerated.

## Batch 3 — SQL function surfaces (14 cards)

- builtin-scalar-agg-funcs(3), date-time-funcs(4), json-funcs(4), printf-format(3) → documented, observed-in-code.
- Registry/family cards stay clustered per charter (no per-function explosion); parity traps carried into cards (ASCII-only LIKE folding, sum overflow, %q/%Q, localtime TZ pinning, JSON5-in/canonical-out).
- Upsert: PASS (documented=44). COVERAGE regenerated.

## Batch 4 — SQL-language surfaces (14 cards)

- pragma-surface(2), window-functions(2), upsert(2), triggers(2), foreign-keys(3), ddl-schema(3) → documented, observed-in-code.
- Pragma census stays one card (charter); trigger/FK cascade behaviour carded with the run-1 async-chain flags preserved.
- Upsert: PASS (documented=58). COVERAGE regenerated.
